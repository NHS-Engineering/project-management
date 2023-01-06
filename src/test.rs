use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::{Status, Header};
use rocket::serde::json::{Value, json};

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

static TEST_USERNAME: &'static str = "test_user";
static TEST_PASSWORD: &'static str = "test_password";

impl<'c> IsolatedClient<'c> {
	fn create_test_account<'a>(&self) -> Header<'a> {
		let user_info: Value = json!({
			"username": TEST_USERNAME,
			"password": TEST_PASSWORD
		});

		let signup_response = self.client.post("/api/auth/signup").json(&user_info).dispatch();
		assert_eq!(signup_response.status(), Status::Ok);

		let password_complaint: Value = json!({
			"BasicRequirement": "password must contain at least one uppercase character"
		});

		assert_eq!(signup_response.into_json().as_ref(), Some(&password_complaint));

		let login_response = self.client.post("/api/auth/login").json(&user_info).dispatch();
		assert_eq!(login_response.status(), Status::Ok);

		let login_response_json: Value = login_response.into_json().unwrap();
		assert_eq!(login_response_json["weak_hint"], password_complaint);

		Header::new("jwt", login_response_json["jwt"].as_str().unwrap().to_string())
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

	let unauth_new_response = instance.client.post("/api/projects/new")
		.body(PROJECT_NAME).dispatch();
	assert_eq!(unauth_new_response.status(), Status::Unauthorized);

	let jwt = instance.create_test_account();

	let new_response = instance.client.post("/api/projects/new")
		.body(PROJECT_NAME).header(jwt).dispatch();
	assert_eq!(new_response.status(), Status::Ok);

	let list_response = instance.client.get("/api/projects/list").dispatch();
	assert_eq!(list_response.status(), Status::Ok);
	assert_eq!(list_response.into_json(), Some(json!({
		"projects": [{
			"id": 1,
			"name": PROJECT_NAME,
			"owner_id": 1,
			"color": null
		}]
	})));
}
