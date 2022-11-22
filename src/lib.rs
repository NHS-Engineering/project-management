use diesel::sqlite::SqliteConnection;
use diesel::{Connection, RunQueryDsl};
use diesel::dsl::sql_query;

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
