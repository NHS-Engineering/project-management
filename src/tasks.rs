use diesel::prelude::*;
use rocket::serde::{Serialize, json::Json};
use rocket::http::Status;
use crate::{get_conn, jwt::JWTAuth};
use crate::models::{NewTask, Task};

#[rocket::post("/new/<project_id>/<name>")]
pub fn new(jwt: JWTAuth, project_id: i32, name: String) {
	use crate::schema::tasks;

	let mut conn = get_conn();

	let new_project = NewTask {
		name,
		project_id,
		assignee_id: jwt.user_id
	};

	diesel::insert_into(tasks::table)
		.values(&new_project)
		.execute(&mut conn).unwrap();
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TaskList {
	tasks: Vec<Task>
}

#[rocket::get("/list/<project_id>")]
pub fn list(project_id: i32) -> Json<TaskList> {
	use crate::schema::tasks::dsl;

	let mut conn = get_conn();

	let project_tasks: Vec<Task> = dsl::tasks.filter(dsl::project_id.eq(project_id))
		.load(&mut conn)
		.unwrap();

	Json(TaskList {
		tasks: project_tasks
	})
}

#[rocket::delete("/delete/<id>")]
pub fn delete(jwt: JWTAuth, id: i32) -> (Status, &'static str) {
	use crate::schema::tasks;
	use crate::schema::projects;

	let mut conn = get_conn();

	// I think I deserve the Nobel Peace Prize for writing this query
	// deletes if the user is the owner of the project which the task is a part of
	let deleted = diesel::delete(tasks::dsl::tasks::filter(tasks::table, tasks::dsl::id.eq(id).and(
		tasks::dsl::project_id.eq_any(projects::dsl::projects.select(projects::dsl::id)
		.filter(projects::dsl::owner_id.eq(jwt.user_id)))
	))).execute(&mut conn);

	match deleted {
		Ok(n) if n == 1 => (Status::Ok, "task deleted"),
		Err(_) => (Status::BadRequest, "task not deleted"),
		_ => unreachable!("multiple tasks deleted?")
	}
}