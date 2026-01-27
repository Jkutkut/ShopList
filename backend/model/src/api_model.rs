use serde::{
	Deserialize,
};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ApiBasicCredentials {
	pub email: String,
	pub password: String
}

#[derive(Debug, Deserialize)]
pub struct ApiRegisterBasicCredentials {
	pub name: String,
	pub email: String,
	pub password: String
}

#[derive(Debug, Deserialize)]
pub struct TeamRequest {
	pub name: String,
	pub description: Option<String>,
	pub image: Option<String>,
}
