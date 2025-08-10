use crate::db::ShoplistDbAuth;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use model::grpc::auth::{
	auth_service_server::{
		AuthService,
	},
	UserToken,
	LoginRequest,
	RegisterBasicUserRequest,
	DeleteUserRequest,
	Empty,
};

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
	) -> Result<Response<UserToken>, Status> {
		let addr = request.remote_addr().unwrap();
		let LoginRequest { username, password } = request.into_inner();
		println!("Login request from {:?}: {:?}", addr, &username);
		match self.db.basic_login(username, password).await {
			Ok(token) => Ok(Response::new(UserToken {
				token
			})),
			Err(_) => Err(Status::unauthenticated("Invalid credentials"))
		}
	}

	async fn register_user_basic_login(
		&self,
		request: Request<RegisterBasicUserRequest>,
	) -> Result<Response<UserToken>, Status> {
		let addr = request.remote_addr().unwrap();
		let RegisterBasicUserRequest { name, email, password } = request.into_inner();
		println!("Register request from {:?}: {}", addr, &email);
		match self.db.register_user_basic_login(name, email, password).await {
			Ok(token) => Ok(Response::new(UserToken {token})),
			Err(_) => Err(Status::unauthenticated("Invalid credentials"))
		}
	}

	async fn delete_user(
		&self,
		request: Request<DeleteUserRequest>,
	) -> Result<Response<Empty>, Status> { // TODO protect
		let addr = request.remote_addr().unwrap();
		let DeleteUserRequest { user_id } = request.into_inner();
		println!("Delete request from {:?}: {}", addr, &user_id);
		let user_id: Uuid = match user_id.parse() {
			Ok(id) => id,
			Err(_) => return Err(Status::invalid_argument("Invalid user_id"))
		};
		self.db.delete_user(&user_id, &user_id).await?;
		Ok(Response::new(Empty {}))
	}
}
