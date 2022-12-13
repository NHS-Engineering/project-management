mod schema;
mod models;
mod jwt;
mod users;
mod auth;
mod projects;
mod tasks;

use engineering_web_portal::{get_conn, run_migrations, copyright_message};

#[rocket::launch]
fn rocket() -> _ {
	copyright_message();

	run_migrations();

	rocket::build()
		.manage(jwt::JWTKeys::generate())
		.mount("/api/users", rocket::routes![users::info])
		.mount("/api/auth", rocket::routes![auth::signup, auth::login, auth::invite, auth::redeem_invite, auth::change_password])
		.mount("/api/projects", rocket::routes![projects::new, projects::list, projects::delete, projects::set_color])
		.mount("/api/tasks", rocket::routes![tasks::new, tasks::list, tasks::delete, tasks::set_done])
		.mount("/", rocket::fs::FileServer::from(match std::env::var("OVERRIDE_STATIC") {
			Ok(path) => path,
			Err(_) => String::from("frontend/dist")
		}))
}
