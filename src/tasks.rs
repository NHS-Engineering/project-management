use diesel::prelude::*;
use rocket::serde::{Serialize, json::Json};
use rocket::http::Status;
use crate::jwt::JWTAuth;
use crate::models::{NewTask, Task};
use crate::pool::Conn;

#[rocket::post("/new/<project_id>", data = "<name>")]
pub fn new(jwt: JWTAuth, mut conn: Conn, project_id: i32, name: String) -> (Status, &'static str) {
	use crate::schema::projects;
	use crate::schema::tasks;

	let new_task = NewTask {
		name,
		project_id,
		assignee_id: jwt.user_id
	};

	let owner_id: i32 = projects::table.find(project_id)
		.select(projects::dsl::owner_id).first(&mut *conn).unwrap();

	match jwt.user_id == owner_id {
		true => {
			diesel::insert_into(tasks::table)
				.values(&new_task)
				.execute(&mut *conn).unwrap();

			(Status::Ok, "created task")
		},
		false => (Status::Forbidden, "you must own a project to create tasks for it")
	}
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TaskList {
	tasks: Vec<Task>
}

#[rocket::get("/list/<project_id>")]
pub fn list(mut conn: Conn, project_id: i32) -> Json<TaskList> {
	use crate::schema::tasks::dsl;

	let project_tasks: Vec<Task> = dsl::tasks.filter(dsl::project_id.eq(project_id))
		.load(&mut *conn)
		.unwrap();

	Json(TaskList {
		tasks: project_tasks
	})
}

#[rocket::delete("/delete/<id>")]
pub fn delete(jwt: JWTAuth, mut conn: Conn, id: i32) -> (Status, &'static str) {
	use crate::schema::tasks;
	use crate::schema::projects;

	// I think I deserve the Nobel Peace Prize for writing this query
	// deletes if the user is the owner of the project which the task is a part of
	let deleted = diesel::delete(tasks::dsl::tasks.filter(tasks::dsl::id.eq(id).and(
		tasks::dsl::project_id.eq_any(projects::dsl::projects.select(projects::dsl::id)
		.filter(projects::dsl::owner_id.eq(jwt.user_id)))
	))).execute(&mut *conn);

	match deleted.unwrap() {
		0 => (Status::BadRequest, "no tasks deleted"),
		1 => (Status::Ok, "task deleted"),
		_ => unreachable!("multiple tasks deleted?")
	}
}

#[rocket::post("/set_done/<id>/<state>")]
pub fn set_done(jwt: JWTAuth, mut conn: Conn, id: i32, state: bool) -> (Status, &'static str) {
	use crate::schema::tasks;

	let changed = diesel::update(tasks::dsl::tasks.find(id).filter(tasks::dsl::assignee_id.eq(jwt.user_id)))
		.set(tasks::dsl::done.eq(state))
		.execute(&mut *conn).unwrap();

	match changed {
		0 => (Status::BadRequest, "no tasks changed"),
		1 => (Status::Ok, "task changed"),
		_ => unreachable!("multiple tasks marked done?")
	}
}

#[rocket::post("/assign/<id>/<assignee_id>")]
pub fn assign(jwt: JWTAuth, mut conn: Conn, id: i32, assignee_id: i32) -> (Status, &'static str) {
	use crate::schema::projects;
	use crate::schema::tasks;
	use crate::schema::users;

	let changed = conn.transaction(|conn| {
		let task = tasks::dsl::tasks.find(id);

		let owned_projects: i64 = projects::dsl::projects.filter(projects::dsl::id.eq_any(
				task.select(tasks::dsl::project_id)
			).and(projects::dsl::owner_id.eq(jwt.user_id))).count().first(conn).unwrap();

		if owned_projects != 1 { return Ok(0); }

		let valid_assignees: i64 = users::dsl::users.find(assignee_id).count().first(conn).unwrap();

		if valid_assignees != 1 { return Ok(0); }

		diesel::update(task).set(tasks::dsl::assignee_id.eq(assignee_id)).execute(conn)
	}).unwrap();

	match changed {
		0 => (Status::BadRequest, "task not assigned"),
		1 => (Status::Ok, "task assigned"),
		_ => unreachable!("multiple assignees set?")
	}
}
