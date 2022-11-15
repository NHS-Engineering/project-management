use diesel::prelude::*;
use rocket::serde::{Serialize, json::Json};
use crate::get_conn;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo {
	username: String
}

#[rocket::get("/info/<id>")]
pub fn info(id: i32) -> Json<UserInfo> {
	use crate::schema::users::dsl;

	let mut conn = get_conn();
	let username = dsl::users.find(id).select(dsl::username).first(&mut conn).unwrap();

	Json(UserInfo {
		username
	})
}
