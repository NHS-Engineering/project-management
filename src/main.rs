mod schema;
mod models;
mod jwt;
mod users;
mod auth;
mod projects;
mod tasks;

#[cfg(test)]
mod test;

use engineering_web_portal::{run_migrations, copyright_message};

mod pool;
use pool::DbManager;

#[rocket::launch]
fn rocket() -> _ {
	copyright_message();

	run_migrations();

	let manager = DbManager::default();
	let pool = r2d2::Pool::builder()
		.build(manager).unwrap();

	rocket::build()
		.manage(jwt::JWTKeys::generate())
		.manage(pool)
		.mount("/api/users", rocket::routes![users::info, users::all_users])
		.mount("/api/auth", rocket::routes![auth::signup, auth::login, auth::invite, auth::redeem_invite, auth::change_password])
		.mount("/api/projects", rocket::routes![projects::new, projects::list, projects::delete, projects::set_color])
		.mount("/api/tasks", rocket::routes![tasks::new, tasks::list, tasks::delete, tasks::set_done])
		.mount("/", rocket::fs::FileServer::from(match std::env::var("OVERRIDE_STATIC") {
			Ok(path) => path,
			Err(_) => String::from("frontend/dist")
		}))
}
