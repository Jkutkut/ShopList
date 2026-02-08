use serde::{
	Serialize,
	Deserialize,
};
use uuid::Uuid;
use model::{
	UuidWrapper,
};
use crate::guards;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
	pub user: guards::User,
	pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct UserRoleRequest {
	pub user_id: UuidWrapper,
	pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductRequest {
	pub name: String,
	pub description: Option<String>,
	pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
	pub id: Uuid,
	pub name: String,
	pub team_id: Uuid,
	pub description: Option<String>,
	pub image: Option<String>,
	pub created_at: String,
	pub created_by: Uuid,
	pub updated_at: String,
	pub updated_by: Uuid,
}
