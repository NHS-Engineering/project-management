use diesel::prelude::*;
use rocket::serde::{Serialize, json::Json};
use rocket::http::Status;
use crate::{get_conn, jwt::JWTAuth};
use crate::models::{NewProject, Project};

#[rocket::post("/new/<name>")]
pub fn new(jwt: JWTAuth, name: String) {
	use crate::schema::projects;

	let mut conn = get_conn();

	let new_project = NewProject {
		name,
		owner_id: jwt.user_id
	};

	diesel::insert_into(projects::table)
		.values(&new_project)
		.execute(&mut conn).unwrap();
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProjectList {
	projects: Vec<Project>
}

#[rocket::get("/list")]
pub fn list() -> Json<ProjectList> {
	use crate::schema::projects::dsl::*;

	let mut conn = get_conn();
	let projects_list: Vec<Project> = projects.load(&mut conn).unwrap();

	Json(ProjectList {
		projects: projects_list
	})
}

#[rocket::delete("/delete/<id>")]
pub fn delete(jwt: JWTAuth, id: i32) -> (Status, &'static str) {
	use crate::schema::projects::dsl;

	let mut conn = get_conn();

	match diesel::delete(dsl::projects.find(id).filter(dsl::owner_id.eq(jwt.user_id))).execute(&mut conn) {
		Ok(n) if n == 1 => (Status::Ok, "project deleted"),
		Err(_) => (Status::BadRequest, "project not deleted"),
		_ => unreachable!("multiple projects deleted?")
	}
}