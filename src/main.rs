use engineering_web_portal::construct_rocket;

#[rocket::launch]
fn rocket() -> _ {
	let db = match std::env::var("OVERRIDE_DB") {
		Ok(overridden_db) => overridden_db,
		Err(_) => String::from("file:db.sqlite")
	};

	construct_rocket(&db)
}
