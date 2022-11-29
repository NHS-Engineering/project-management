use diesel::sqlite::SqliteConnection;
use diesel::{Connection, RunQueryDsl};
use diesel::dsl::sql_query;

pub fn copyright_message() {
	println!("Copyright 2022 NHS Engineering Club.");
	println!("Licensed under the GNU Affero General Public License version 3 (https://www.gnu.org/licenses/agpl-3.0.en.html).");
	println!("Source code may be found at https://github.com/NHS-Engineering/project-management.");
}

fn _get_conn() -> SqliteConnection {
	let db = match std::env::var("OVERRIDE_DB") {
		Ok(overridden_db) => overridden_db,
		Err(_) => String::from("file:db.sqlite")
	};

	SqliteConnection::establish(&db).unwrap()
}

pub fn get_conn() -> SqliteConnection {
	let mut conn = _get_conn();
	sql_query("PRAGMA foreign_keys = ON;").execute(&mut conn).unwrap();
	conn
}

pub fn run_migrations() {
	use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
	const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

	let mut conn = _get_conn();
	conn.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn get_url() -> String {
	match std::env::var("OVERRIDE_URL") {
		Ok(overridden_url) => overridden_url,
		Err(_) => String::from("http://127.0.0.1:8000")
	}
}
