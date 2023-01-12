use rocket::serde::{Deserialize, json::Json};
use diesel::prelude::*;
use jwt_simple::prelude::*;
use crate::get_url;
use rocket::http::{Status, ContentType};
use crate::jwt::{JWTKeys, JWTAuth, JWTNewAccount};
use crate::pool::Conn;

const BANNED_WORDS: [&'static str; 6] = ["lion", "engineering", "engineer", "nhs", "nhsd", "password"];

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum PasswordValidity {
	BasicRequirement(&'static str),
	BannedWord(&'static str),
	Valid
}

impl PasswordValidity {
	fn check(password: &str) -> Self {
		if password.len() < 8 {
			return Self::BasicRequirement("password must be least 8 characters");
		}

		if !password.chars().any(|c| c.is_ascii_lowercase()) {
			return Self::BasicRequirement("password must contain at least one lowercase character");
		}

		if !password.chars().any(|c| c.is_ascii_uppercase()) {
			return Self::BasicRequirement("password must contain at least one uppercase character");
		}

		if !password.chars().any(|c| c.is_ascii_digit()) {
			return Self::BasicRequirement("password must contain at least one numeric character");
		}

		let lower = password.to_lowercase();
		for banned_word in BANNED_WORDS {
			if lower.contains(banned_word) {
				return Self::BannedWord(banned_word)
			}
		}

		Self::Valid
	}
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo<'a> {
	username: &'a str,
	password: &'a str
}

#[cfg(any(test, feature = "debug"))]
#[rocket::post("/signup", data = "<user_info>")]
pub fn signup(mut conn: Conn, user_info: Json<UserInfo<'_>>) -> Json<PasswordValidity> {
	use crate::models::NewUser;
	use sha3::{Sha3_512, Digest};
	use crate::schema::users;

	// yes I know, and the answer is "not in my threat model"
	let hashed_password = format!("{:x}", Sha3_512::digest(user_info.password));
	let new_user = NewUser {
		username: user_info.username,
		hashed_password,
		is_admin: true
	};

	diesel::insert_into(users::table)
		.values(&new_user)
		.execute(&mut *conn).unwrap();

	Json(PasswordValidity::check(user_info.password))
}

#[cfg(not(any(test, feature = "debug")))]
#[rocket::post("/signup")]
pub fn signup() -> (Status, &'static str) {
	(Status::ImATeapot, "signup is only enabled in test/debug mode, try the invite system instead")
}

fn password_correct(conn: &mut SqliteConnection, user_id: i32, password: &str) -> bool {
	use sha3::{Sha3_512, Digest};
	use crate::schema::users::dsl;

	let provided_hashed_password = format!("{:x}", Sha3_512::digest(password));
	let provided_bytes = provided_hashed_password.as_bytes();

	let retrieved_hashed_password: String = dsl::users.find(user_id).select(dsl::hashed_password).first(conn).unwrap();
	let retrieved_bytes = retrieved_hashed_password.as_bytes();

	constant_time_eq::constant_time_eq(provided_bytes, retrieved_bytes)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthResponse {
	jwt: String,
	weak_hint: PasswordValidity
}

#[rocket::post("/login", data = "<user_info>")]
pub fn login(mut conn: Conn, user_info: Json<UserInfo<'_>>, keyring: &rocket::State<JWTKeys>) -> (Status, Option<Json<AuthResponse>>) {
	use crate::schema::users::dsl;

	let (user_id, is_admin): (i32, bool) = dsl::users.select((dsl::id, dsl::is_admin)).filter(dsl::username.eq(user_info.username)).first(&mut *conn).unwrap();

	match password_correct(&mut *conn, user_id, &user_info.password) {
		true => {
			let weak_hint = PasswordValidity::check(user_info.password);

			let user_auth = JWTAuth {
				user_id,
				is_admin
			};
			let claims = Claims::with_custom_claims(user_auth, Duration::from_mins(5));

			(Status::Ok, Some(Json(AuthResponse {
				jwt: keyring.user_key.authenticate(claims).unwrap(),
				weak_hint
			})))
		},
		false => (Status::Forbidden, None)
	}
}

#[rocket::post("/invite", data = "<username>")]
pub fn invite(jwt: JWTAuth, username: String, keyring: &rocket::State<JWTKeys>) -> (Status, (ContentType, String)) {
	use fast_qr::ECL;
	use fast_qr::qr::QRBuilder;
	use fast_qr::convert::svg::SvgBuilder;

	if !jwt.is_admin {
		return (Status::Forbidden, (ContentType::Plain, String::from("you are not permitted to invite users")));
	}

	let key = &keyring.new_account_key;

	let invitation = JWTNewAccount {
		username
	};
	let claims = Claims::with_custom_claims(invitation, Duration::from_mins(5));
	let new_jwt = key.authenticate(claims).unwrap();

	let mut invite_url = get_url();
	invite_url.push_str("/?invite=");
	invite_url.push_str(&new_jwt);

	#[cfg(feature = "debug")]
	println!("[debug] invite url: {}", invite_url);

	let qrcode = QRBuilder::new(invite_url).ecl(ECL::H).build().unwrap();
	let qr_svg = SvgBuilder::default().to_str(&qrcode);

	(Status::Ok, (ContentType::SVG, qr_svg))
}

#[rocket::post("/redeem_invite", data = "<password>")]
pub fn redeem_invite(jwt: JWTNewAccount, mut conn: Conn, password: String) -> (Status, Json<PasswordValidity>) {
	use crate::models::NewUser;
	use sha3::{Sha3_512, Digest};
	use crate::schema::users;

	match PasswordValidity::check(&password) {
		PasswordValidity::Valid => (),
		problem => return (Status::BadRequest, Json(problem))
	};

	// yes I'm aware, but probably not that big of a deal tbh
	let hashed_password = format!("{:x}", Sha3_512::digest(password));

	let new_user = NewUser {
		username: &jwt.username,
		hashed_password,
		is_admin: false
	};

	diesel::insert_into(users::table)
		.values(&new_user)
		.execute(&mut *conn).unwrap();

	(Status::Ok, Json(PasswordValidity::Valid))
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PasswordChangeInfo<'a> {
	old_password: &'a str,
	new_password: &'a str
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum PasswordChangeResult {
	Success,
	PasswordIncorrect,
	InvalidPassword(PasswordValidity)
}

#[rocket::post("/change_password", data = "<change_info>")]
pub fn change_password(jwt: JWTAuth, mut conn: Conn, change_info: Json<PasswordChangeInfo<'_>>) -> (Status, Json<PasswordChangeResult>) {
	use sha3::{Sha3_512, Digest};
	use crate::schema::users::dsl;

	match password_correct(&mut conn, jwt.user_id, change_info.old_password) {
		true => {
			match PasswordValidity::check(change_info.new_password) {
				PasswordValidity::Valid => {
					let new_hashed = format!("{:x}", Sha3_512::digest(change_info.new_password));

					diesel::update(dsl::users.find(jwt.user_id))
						.set(dsl::hashed_password.eq(new_hashed))
						.execute(&mut *conn).unwrap();

					(Status::Ok, Json(PasswordChangeResult::Success))
				},
				problem => (Status::BadRequest, Json(PasswordChangeResult::InvalidPassword(problem)))
			}
		},
		false => (Status::Forbidden, Json(PasswordChangeResult::PasswordIncorrect))
	}
}
