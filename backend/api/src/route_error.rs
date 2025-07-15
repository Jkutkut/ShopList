use rocket::{catch};
use rocket::serde::json::Json;

#[catch(404)]
pub fn not_found() -> Json<&'static str> {
  Json("Resource not found.")
}

#[catch(501)]
pub fn not_implemented() -> Json<&'static str> {
  Json("This feature is not implemented yet.")
}

#[catch(500)]
pub fn internal_server_error() -> Json<&'static str> {
  Json("Something went wrong on our side.")
}
