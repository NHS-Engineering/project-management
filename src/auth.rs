use rocket::serde::{Deserialize, json::Json};
use diesel::prelude::*;
use jwt_simple::prelude::*;
use engineering_web_portal::get_conn;
use rocket::http::Status;
use crate::models::NewUser;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo<'a> {
	username: &'a str,
	password: &'a str
}

#[rocket::post("/signup", data = "<user_info>")]
pub fn signup(user_info: Json<UserInfo<'_>>) {
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

#[rocket::post("/login", data = "<user_info>")]
pub fn login(user_info: Json<UserInfo<'_>>, key: &rocket::State<HS256Key>) -> (Status, String) {
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
			let user_auth = crate::jwt::JWTAuth {
				user_id
			};
			let claims = Claims::with_custom_claims(user_auth, Duration::from_mins(5));
			(Status::Ok, key.authenticate(claims).unwrap())
		},
		false => (Status::Forbidden, String::from("incorrect password"))
	}
}