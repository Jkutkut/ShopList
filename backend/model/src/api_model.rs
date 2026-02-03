use serde::{
	Deserialize,
	Serialize,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
	pub id: Uuid,
	pub name: String,
	pub description: Option<String>,
	pub image: Option<String>,
	pub created_at: String,
	pub created_by: Uuid,
	pub updated_at: String,
	pub updated_by: Uuid,
}
