use tonic::{transport::Server, Request, Response, Status};

pub mod auth {
	tonic::include_proto!("auth");
}

use crate::auth::auth_service_server::{
	AuthService,
	AuthServiceServer
};
use crate::auth::{
	LoginRequest,
	AuthResponse
};

#[derive(Default)]
pub struct Auth {}

#[tonic::async_trait]
impl AuthService for Auth {
	async fn basic_login(
		&self,
		request: Request<LoginRequest>,
	) -> Result<Response<AuthResponse>, Status> {
		let addr = request.remote_addr().unwrap();
		let LoginRequest { username, password } = request.into_inner();
		println!("Got a login request from {:?}: {:?}", addr, [&username, &password]);
		if username.is_empty() || password.is_empty() {
			return Err(Status::invalid_argument("Empty username or password"));
		}
		if &username == "jkutkut" {
			return Ok(Response::new(AuthResponse {
				token: "secret".to_string(),
			}))
		}
		Err(Status::unauthenticated("Invalid credentials"))
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("Hello auth!");
	let addr = "0.0.0.0:50051".parse().unwrap();

	println!("GreeterServer listening on {addr}");

	Server::builder()
		.add_service(AuthServiceServer::new(Auth::default()))
		.serve(addr)
		.await?;

	Ok(())
}
