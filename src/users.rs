use diesel::prelude::*;
use rocket::serde::json::Json;
use crate::jwt::JWTAuth;
use crate::models::{UserInfo, UserPreview};
use crate::pool::Conn;

#[rocket::get("/info/<id>")]
pub fn info(mut conn: Conn, id: i32) -> Json<UserInfo> {
	use crate::schema::users::dsl;

	Json(dsl::users.find(id).select((dsl::id, dsl::username, dsl::is_admin)).first(&mut *conn).unwrap())
}

#[rocket::get("/preview")]
pub fn preview(_jwt: JWTAuth, mut conn: Conn) -> Json<Vec<UserPreview>> {
	use crate::schema::users::dsl;

	Json(dsl::users.select((dsl::id, dsl::username)).load(&mut *conn).unwrap())
}
