use diesel::{QueryDsl, RunQueryDsl, Connection};
use sha3::{Sha3_512, Digest};
use r2d2::ManageConnection;
use crate::pool::DbManager;
use crate::models::NewUser;
use crate::schema::users;

const USERNAME: &'static str = "admin";
const PASSWORD: &'static str = "password";

pub fn setup(pool: &DbManager) {
	let mut conn = pool.connect().unwrap();

	conn.transaction(|conn| {
		let users: i64 = users::dsl::users.count().first(conn)?;

		if users != 0 {
			return Ok(0);
		}

		println!("automatically creating default user, username: {:?}, password: {:?}", USERNAME, PASSWORD);
		let hashed_password = format!("{:x}", Sha3_512::digest(PASSWORD));
		let new_user = NewUser {
			username: USERNAME,
			hashed_password,
			is_admin: true
		};

		diesel::insert_into(users::table)
			.values(&new_user)
			.execute(conn)
	}).unwrap();
}