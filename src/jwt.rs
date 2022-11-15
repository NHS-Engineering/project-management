use jwt_simple::prelude::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Request, Outcome};

#[derive(Debug)]
pub enum JWTError {
	BadJWT,
	NoJWT
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct JWTAuth {
	pub user_id: i32
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWTAuth {
	type Error = JWTError;

	async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		match req.headers().get_one("jwt") {
			Some(jwt_header) => {
				let key = &req.rocket().state::<HS256Key>().unwrap();

				match key.verify_token::<Self>(&jwt_header, None).ok().map(|claims| claims.custom) {
					Some(claims) => Outcome::Success(claims),
					None => Outcome::Failure((Status::BadRequest, JWTError::BadJWT))
				}
			},
			None => Outcome::Failure((Status::Unauthorized, JWTError::NoJWT))
		}
	}
}