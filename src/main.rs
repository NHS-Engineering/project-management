mod schema;
mod models;
mod jwt;

use jwt_simple::prelude::*;
use engineering_web_portal::{get_conn, run_migrations};

#[cfg(feature = "debug")]
#[rocket::get("/users")]
fn get_users() -> String {
	use diesel::prelude::*;
	use schema::users::dsl::*;
	use models::User;

	let mut conn = get_conn();
	let results: Vec<User> = users.load(&mut conn).unwrap();
	format!("{:?}", results)
}

mod users;
mod auth;
mod projects;
mod tasks;

#[rocket::launch]
fn rocket() -> _ {
	run_migrations();

	let application = rocket::build()
		.manage(HS256Key::generate())
		.mount("/api/users", rocket::routes![users::info])
		.mount("/api/auth", rocket::routes![auth::signup, auth::login])
		.mount("/api/projects", rocket::routes![projects::new, projects::list, projects::delete])
		.mount("/api/tasks", rocket::routes![tasks::new, tasks::list]);

	#[cfg(feature = "static")]
	let application = application.mount("/", rocket::fs::FileServer::from("frontend/dist"));

	#[cfg(feature = "debug")]
	let application = application.mount("/api/debug", rocket::routes![get_users]);

	application
}
