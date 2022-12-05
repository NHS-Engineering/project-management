use diesel::prelude::*;
use rocket::serde::Serialize;

#[derive(Queryable, Debug)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub hashed_password: String,
	pub is_admin: bool
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
	pub username: &'a str,
	pub hashed_password: String
}

#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Project {
	pub id: i32,
	pub name: String,
	pub owner_id: i32,
	pub color: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::projects)]
pub struct NewProject {
	pub name: String,
	pub owner_id: i32
}

#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Task {
	pub id: i32,
	pub project_id: i32,
	pub assignee_id: Option<i32>,
	pub done: bool,
	pub name: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewTask {
	pub name: String,
	pub project_id: i32,
	pub assignee_id: i32
}
