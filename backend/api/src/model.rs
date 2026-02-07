use serde::{
	Serialize,
	Deserialize,
};
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
