use diesel::prelude::*;
use rocket::serde::{Serialize, json::Json};
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
