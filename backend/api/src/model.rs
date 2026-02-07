use serde::{
	Serialize,
	Deserialize,
};
use model::{
	UuidWrapper,
};
use crate::guards;

#[derive(Serialize)]
pub struct UserRole {
	user: guards::User,
	role: String,
}

#[derive(Debug, Deserialize)]
pub struct UserRoleRequest {
	pub user_id: UuidWrapper,
	pub role: String,
}
