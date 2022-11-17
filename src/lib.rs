use diesel::sqlite::SqliteConnection;
use diesel::{Connection, RunQueryDsl};
use diesel::dsl::sql_query;

#[cfg(feature = "prod_db")]
const DB: &'static str = "file:/home/engineer/db.sqlite";

#[cfg(not(feature = "prod_db"))]
const DB: &'static str = "file:db.sqlite";

pub fn get_conn() -> SqliteConnection {
	let mut conn = SqliteConnection::establish(DB).unwrap();
	sql_query("PRAGMA foreign_keys = ON;").execute(&mut conn).unwrap();
	conn
}

pub fn run_migrations() {
	use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
	const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

	let mut conn = get_conn();
	conn.run_pending_migrations(MIGRATIONS).unwrap();
}
