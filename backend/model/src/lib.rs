mod uuid_wrapper;

#[cfg(feature = "api")]
mod api_model;
#[cfg(feature = "auth")]
mod auth_model;

pub use uuid_wrapper::*;

#[cfg(feature = "api")]
pub use api_model::*;
// #[cfg(feature = "auth")]
// pub use auth_model::*;

use uuid::Uuid;

#[derive(Debug)]
pub struct BasicLogin {
	pub id: Uuid,
	pub user_id: Uuid,
	pub email: String,
	pub password: String,
}

pub mod grpc {
	pub mod auth {
		tonic::include_proto!("auth");
	}
}
