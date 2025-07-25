use rocket::catch;
use rocket::serde::json::Json;
use rocket::response::Responder;
use rocket::Request;
use rocket::http::Status;

#[derive(serde::Serialize)]
pub struct InvalidResponse{
	status: Status,
	message: String,
}

impl InvalidResponse {
	pub fn new(status: Status, message: &str) -> Self {
		Self { status, message: message.to_string() }
	}
}

impl<'r> Responder<'r, 'static> for InvalidResponse {
	fn respond_to(self, _req: &'r Request<'_>) -> rocket::response::Result<'static> {
		rocket::response::status::Custom(self.status, Json(self)).respond_to(_req)
	}
}

#[catch(401)]
pub fn unauthorized() -> InvalidResponse {
	InvalidResponse::new(Status::Unauthorized, "Unauthorized")
}

#[catch(404)]
pub fn not_found() -> InvalidResponse {
	InvalidResponse::new(Status::NotFound, "Not Found")
}

pub fn invalid_api(reason: &str) -> InvalidResponse {
	InvalidResponse::new(Status::BadRequest, reason)
}

#[catch(500)]
pub fn internal_server_error() -> InvalidResponse {
	InvalidResponse::new(Status::InternalServerError, "Internal Server Error")
}

#[catch(501)]
pub fn not_implemented() -> InvalidResponse {
	InvalidResponse::new(Status::NotImplemented, "Not Implemented")
}
