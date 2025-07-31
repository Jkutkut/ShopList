use rocket::http::Header;
use rocket::response::{Responder, Response};
use rocket::http::ContentType;
use rocket::Request;
use serde::Serialize;

#[derive(Debug)]
pub struct ApiAuthResponse<T: Serialize> {
	token: String,
	response: T
}

impl<T> ApiAuthResponse<T>
where
	T: Serialize,
{
	pub fn new(token: String, response: T) -> ApiAuthResponse<T> {
		Self {
			token,
			response
		}
	}
}

#[rocket::async_trait]
impl<'r, T> Responder<'r, 'static> for ApiAuthResponse<T>
where
	T: Serialize,
{
	fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
		let json = serde_json::to_string(&self.response).unwrap();
		Ok(Response::build()
			.header(Header::new("Authorization", format!("Bearer {}", self.token)))
			.sized_body(json.len(), std::io::Cursor::new(json))
			.header(ContentType::JSON)
			.ok()?
		)
	}
}
