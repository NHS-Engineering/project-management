use diesel::{sql_query, RunQueryDsl};
use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use r2d2::{ManageConnection, PooledConnection, Pool};

#[derive(Default)]
pub struct DbManager;

impl ManageConnection for DbManager {
	type Connection = SqliteConnection;
	type Error = std::convert::Infallible; // yeah this is a blatent lie, I just don't feel like implementing Error

	fn connect(&self) -> Result<Self::Connection, Self::Error> {
		let mut conn = engineering_web_portal::get_raw_conn();
		sql_query("PRAGMA foreign_keys = ON;").execute(&mut conn).unwrap();
		Ok(conn)
	}

	fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
		conn.test_transaction(|conn| {
			sql_query("SELECT 1").execute(conn)
		});
		Ok(())
	}

	fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
		false
	}
}

use core::ops::{Deref, DerefMut};

pub struct Conn {
	pooled_conn: PooledConnection<DbManager>
}

impl Deref for Conn {
	type Target = SqliteConnection;

	fn deref(&self) -> &Self::Target {
		self.pooled_conn.deref()
	}
}

impl DerefMut for Conn {
	fn deref_mut(&mut self) -> &mut SqliteConnection {
		self.pooled_conn.deref_mut()
	}
}

impl std::convert::From<PooledConnection<DbManager>> for Conn {
	fn from(pooled_conn: PooledConnection<DbManager>) -> Self {
		Self {
			pooled_conn
		}
	}
}

use rocket::http::Status;
use rocket::request::{FromRequest, Request, Outcome};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Conn {
	type Error = r2d2::Error;

	async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let pool = &req.rocket().state::<Pool<DbManager>>().unwrap();
		match pool.get() {
			Ok(conn) => Outcome::Success(Conn::from(conn)),
			Err(err) => Outcome::Failure((Status::InternalServerError, err))
		}
	}
}
