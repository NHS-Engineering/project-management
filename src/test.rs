use crate::construct_rocket;
use rocket::local::blocking::Client;
use rocket::http::{Status, Header};
use rocket::serde::json::{Value, json};
use temp_dir::TempDir;

struct IsolatedClient {
	client: Client,

	// compiler generated Drop gives us RAII on temporary database
	#[allow(dead_code)]
	tmp_dir: TempDir
}

impl std::default::Default for IsolatedClient {
	fn default() -> Self {
		// makes tests working directory agnostic, don't expect testing static files to work correctly...
		let tmp_dir = TempDir::new().unwrap();
		let tmp_path = tmp_dir.path();
		std::env::set_var("OVERRIDE_STATIC", tmp_path.as_os_str());

		let rocket = construct_rocket(tmp_path.join("db.sqlite").to_str().unwrap());

		Self {
			client: Client::tracked(rocket).unwrap(),
			tmp_dir
		}
	}
}

static TEST_USERNAME: &'static str = "test_user";
static TEST_PASSWORD: &'static str = "test_password";

impl IsolatedClient {
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
