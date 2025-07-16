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
	fn new(status: Status, message: String) -> Self {
		Self { status, message }
	}
}

impl<'r> Responder<'r, 'static> for InvalidResponse {
	fn respond_to(self, _req: &'r Request<'_>) -> rocket::response::Result<'static> {
		rocket::response::status::Custom(self.status, Json(self)).respond_to(_req)
	}
}

fn msg(status: Status, msg: &str) -> InvalidResponse {
	InvalidResponse::new(status, msg.to_string())
}

#[catch(401)]
pub fn unauthorized() -> InvalidResponse {
	msg(Status::Unauthorized, "Unauthorized")
}

#[catch(404)]
pub fn not_found() -> InvalidResponse {
	msg(Status::NotFound, "Not Found")
}

pub fn invalid_api(reason: &str) -> InvalidResponse {
	msg(Status::BadRequest, reason)
}

#[catch(500)]
pub fn internal_server_error() -> InvalidResponse {
	msg(Status::InternalServerError, "Internal Server Error")
}

#[catch(501)]
pub fn not_implemented() -> InvalidResponse {
	msg(Status::NotImplemented, "Not Implemented")
}
