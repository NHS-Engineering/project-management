use rocket::serde::{Deserialize, json::Json};
use diesel::prelude::*;
use jwt_simple::prelude::*;
use engineering_web_portal::{get_conn, get_url};
use rocket::http::{Status, ContentType};
use crate::jwt::{JWTKeys, JWTAuth, JWTNewAccount};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo<'a> {
	username: &'a str,
	password: &'a str
}

#[cfg(feature = "debug")]
#[rocket::post("/signup", data = "<user_info>")]
pub fn signup(user_info: Json<UserInfo<'_>>) {
	use crate::models::NewUser;
	use sha3::{Sha3_512, Digest};
	use crate::schema::users;

	// yes I know, and the answer is "not in my threat model"
	let hashed_password = format!("{:x}", Sha3_512::digest(user_info.password));
	let mut conn = get_conn();
	let new_user = NewUser {
		username: user_info.username,
		hashed_password
	};

	diesel::insert_into(users::table)
		.values(&new_user)
		.execute(&mut conn).unwrap();
}

#[cfg(not(feature = "debug"))]
#[rocket::post("/signup")]
pub fn signup() -> (Status, &'static str) {
	(Status::ImATeapot, "signup is only enabled in debug mode, try the invite system instead")
}

#[rocket::post("/login", data = "<user_info>")]
pub fn login(user_info: Json<UserInfo<'_>>, keyring: &rocket::State<JWTKeys>) -> (Status, String) {
	use sha3::{Sha3_512, Digest};
	use crate::schema::users::dsl::*;

	let provided_hashed_password = format!("{:x}", Sha3_512::digest(user_info.password));
	let provided_bytes = provided_hashed_password.as_bytes();

	let mut conn = get_conn();
	let (user_hashed_password, user_id): (String, i32) = users.select((hashed_password, id)).filter(username.eq(user_info.username)).first(&mut conn).unwrap();
	let retrieved_bytes = user_hashed_password.as_bytes();

	let password_correct = constant_time_eq::constant_time_eq(provided_bytes, retrieved_bytes);

	match password_correct {
		true => {
			let user_auth = JWTAuth {
				user_id
			};
			let claims = Claims::with_custom_claims(user_auth, Duration::from_mins(5));
			(Status::Ok, keyring.user_key.authenticate(claims).unwrap())
		},
		false => (Status::Forbidden, String::from("incorrect password"))
	}
}

#[rocket::post("/invite", data = "<username>")]
pub fn invite(jwt: JWTAuth, username: String, keyring: &rocket::State<JWTKeys>) -> (Status, (ContentType, String)) {
	use fast_qr::ECL;
	use fast_qr::qr::QRBuilder;
	use fast_qr::convert::svg::SvgBuilder;

	// TODO: check admin field
	if jwt.user_id != 1 {
		return (Status::Forbidden, (ContentType::Plain, String::from("you are not permitted to invite users")));
	}

	let key = &keyring.new_account_key;

	let invitation = JWTNewAccount {
		username
	};
	let claims = Claims::with_custom_claims(invitation, Duration::from_mins(2));
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
pub fn redeem_invite(jwt: JWTNewAccount, password: String) {
	use crate::models::NewUser;
	use sha3::{Sha3_512, Digest};
	use crate::schema::users;

	// yes I'm aware, but probably not that big of a deal tbh
	let hashed_password = format!("{:x}", Sha3_512::digest(password));

	let new_user = NewUser {
		username: &jwt.username,
		hashed_password
	};

	let mut conn = get_conn();

	diesel::insert_into(users::table)
		.values(&new_user)
		.execute(&mut conn).unwrap();
}
