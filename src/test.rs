use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::Status;
use rocket::serde::json::json;

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
		let temp_db_path = temp_db.path();
		std::env::set_var("OVERRIDE_DB", temp_db_path.as_os_str());

		// makes tests working directory agnostic, don't expect testing static files to work correctly...
		std::env::set_var("OVERRIDE_STATIC", temp_db_path.ancestors().nth(2).unwrap());

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
	let list_response = instance.client.get("/api/projects/list").dispatch();
	assert_eq!(list_response.status(), Status::Ok);
	assert_eq!(list_response.into_json(), Some(json!({
		"projects": []
	})));

	const PROJECT_NAME: &'static str = "test project";

	let new_response = instance.client.post("/api/projects/new")
		.body(PROJECT_NAME).dispatch();
	assert_eq!(new_response.status(), Status::Unauthorized);
}
