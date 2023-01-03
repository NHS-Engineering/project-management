use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::Status;

use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;
use temp_file::TempFile;

lazy_static! {
	pub static ref LOCK: Mutex<()> = Mutex::new(());
}

struct IsolatedClient<'a> {
	client: Client,

	// compiler generated Drop gives us RAII on a Client
	#[allow(dead_code)]
	temp_db: TempFile,
	#[allow(dead_code)]
	lock: MutexGuard<'a, ()>
}

impl<'a> std::default::Default for IsolatedClient<'a> {
	fn default() -> Self {
		let lock = LOCK.lock().unwrap();
		let temp_db = TempFile::new().unwrap();
		std::env::set_var("OVERRIDE_DB", temp_db.path().as_os_str());

		Self {
			client: Client::tracked(rocket()).unwrap(),
			temp_db,
			lock
		}
	}
}

#[test]
fn tasks() {
	let instance = IsolatedClient::default();
	let response = instance.client.get("/api/projects/list").dispatch();
	assert_eq!(response.status(), Status::Ok);
}
