use jwt_simple::prelude::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Request, Outcome};

pub struct JWTKeys {
	pub user_key: HS256Key,
	pub new_account_key: HS256Key
}

impl JWTKeys {
	pub fn generate() -> Self {
		Self {
			user_key: HS256Key::generate(),
			new_account_key: HS256Key::generate()
		}
	}
}

#[derive(Debug)]
pub enum JWTError {
	BadJWT,
	NoJWT
}

macro_rules! impl_jwt {
	($t: ty, $k: ident) => {
		#[rocket::async_trait]
		impl<'r> FromRequest<'r> for $t {
			type Error = JWTError;

			async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
				match req.headers().get_one("jwt") {
					Some(jwt_header) => {
						let keyring = &req.rocket().state::<JWTKeys>().unwrap();
						let key = &keyring.$k;

						match key.verify_token::<Self>(&jwt_header, None).ok().map(|claims| claims.custom) {
							Some(claims) => Outcome::Success(claims),
							None => Outcome::Failure((Status::BadRequest, JWTError::BadJWT))
						}
					},
					None => Outcome::Failure((Status::Unauthorized, JWTError::NoJWT))
				}
			}
		}
	}
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct JWTAuth {
	pub user_id: i32
}

impl_jwt!(JWTAuth, user_key);

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct JWTNewAccount {
	pub username: String
}

impl_jwt!(JWTNewAccount, new_account_key);
