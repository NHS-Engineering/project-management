use diesel::sqlite::SqliteConnection;
use diesel::Connection;

#[cfg(test)]
mod test;

mod pool;
use pool::DbManager;

pub fn copyright_message() {
	println!("Copyright 2022 NHS Engineering Club.");
	println!("Licensed under the GNU Affero General Public License version 3 (https://www.gnu.org/licenses/agpl-3.0.en.html).");
	println!("Source code may be found at https://github.com/NHS-Engineering/project-management.");
}

pub fn get_raw_conn(db: &str) -> SqliteConnection {
	SqliteConnection::establish(db).unwrap()
}

pub fn run_migrations(db: &str) {
	use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
	const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

	let mut conn = get_raw_conn(db);
	conn.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn get_url() -> String {
	match std::env::var("OVERRIDE_URL") {
		Ok(overridden_url) => overridden_url,
		Err(_) => String::from("http://127.0.0.1:8000")
	}
}

mod schema;
mod models;
mod jwt;
mod users;
mod auth;
mod projects;
mod tasks;

pub fn construct_rocket(db: &str) -> rocket::Rocket<rocket::Build> {
	copyright_message();

	run_migrations(db);

	let manager = DbManager::from_db(db);
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
