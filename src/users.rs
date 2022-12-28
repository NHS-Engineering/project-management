use diesel::prelude::*;
use rocket::serde::json::Json;
use crate::{get_conn, jwt::JWTAuth};
use crate::models::UserInfo;

#[rocket::get("/info/<id>")]
pub fn info(id: i32) -> Json<UserInfo> {
	use crate::schema::users::dsl;

	let mut conn = get_conn();

	Json(dsl::users.find(id).select((dsl::id, dsl::username, dsl::is_admin)).first(&mut conn).unwrap())
}

#[rocket::get("/list")]
pub fn all_users(_jwt: JWTAuth) -> Json<Vec<UserInfo>> {
	use crate::schema::users::dsl;

	let mut conn = get_conn();

	Json(dsl::users.select((dsl::id, dsl::username, dsl::is_admin)).load(&mut conn).unwrap())
}
