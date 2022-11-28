use diesel::prelude::*;
use rocket::serde::{Serialize, json::Json};
use rocket::http::Status;
use crate::{get_conn, jwt::JWTAuth};
use crate::models::{NewProject, Project};

#[rocket::post("/new", data = "<name>")]
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

#[rocket::post("/set_color/<id>", data = "<color>")]
pub fn set_color(jwt: JWTAuth, id: i32, color: String) -> (Status, &'static str) {
	use crate::schema::projects::dsl;

	let mut color_chars = color.chars();

	if color_chars.next() != Some('#') {
		return (Status::BadRequest, "color expected to start with '#'");
	}

	let valid_chars = color_chars.map(|c| c.is_ascii_hexdigit() && c.is_ascii_lowercase());

	let mut char_count = 0;
	for valid in valid_chars {
		char_count += 1;

		if !valid {
			return (Status::BadRequest, "invalid color character");
		}
	}

	if char_count != 6 {
		return (Status::BadRequest, "invalid color length");
	}

	let mut conn = get_conn();

	let owned_project = dsl::id.eq(id).and(dsl::owner_id.eq(jwt.user_id));
	match diesel::update(dsl::projects.filter(owned_project)).set(dsl::color.eq(color)).execute(&mut conn) {
		Ok(n) if n == 1 => (Status::Ok, "color changed"),
		Err(_) => (Status::BadRequest, "color not changed"),
		_ => unreachable!("multiple colors changed?")
	}
}