use diesel::sqlite::SqliteConnection;
use diesel::{Connection, RunQueryDsl};
use diesel::dsl::sql_query;

#[cfg(feature = "prod_db")]
pub fn get_conn() -> SqliteConnection {
	let mut conn = SqliteConnection::establish("file:/home/engineer/db.sqlite").unwrap();
	sql_query("PRAGMA foreign_keys = ON;").execute(&mut conn);
	conn
}

#[cfg(not(feature = "prod_db"))]
pub fn get_conn() -> SqliteConnection {
	let mut conn = SqliteConnection::establish("file:db.sqlite").unwrap();
	sql_query("PRAGMA foreign_keys = ON;").execute(&mut conn);
	conn
}

pub fn run_migrations() {
	use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
	const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

	let mut conn = get_conn();
	conn.run_pending_migrations(MIGRATIONS).unwrap();
}
