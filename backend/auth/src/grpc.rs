tonic::include_proto!("auth");

use crate::db::ShoplistDbAuth;
use tonic::{Request, Response, Status};
use auth_service_server::AuthService;

pub use auth_service_server::AuthServiceServer;

pub struct Auth {
	db: ShoplistDbAuth
}

impl Auth {
	pub fn new(db: ShoplistDbAuth) -> Self {
		Self { db }
	}
}

#[tonic::async_trait]
impl AuthService for Auth {
	async fn basic_login(
		&self,
		request: Request<LoginRequest>,
	) -> Result<Response<AuthResponse>, Status> {
		let addr = request.remote_addr().unwrap();
		let LoginRequest { username, password } = request.into_inner();
		println!("Login request from {:?}: {:?}", addr, &username);
		match self.db.basic_login(username, password).await {
			Ok(token) => Ok(Response::new(AuthResponse {
				token
			})),
			Err(_) => Err(Status::unauthenticated("Invalid credentials"))
		}
	}
}
