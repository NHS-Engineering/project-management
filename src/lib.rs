use diesel::sqlite::SqliteConnection;
use diesel::Connection;

#[cfg(feature = "debug")]
pub fn get_conn() -> SqliteConnection {
	SqliteConnection::establish("file:db.sqlite").unwrap()
}

#[cfg(not(feature = "debug"))]
pub fn get_conn() -> SqliteConnection {
	SqliteConnection::establish("file:/home/engineer/db.sqlite").unwrap()
}

pub fn run_migrations() {
	use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
	const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

	let mut conn = get_conn();
	conn.run_pending_migrations(MIGRATIONS).unwrap();
}
