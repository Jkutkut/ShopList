use serde::{Deserialize};

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
