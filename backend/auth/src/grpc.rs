use crate::db::ShoplistDbAuth;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use model::grpc::auth::{
	auth_service_server::{
		AuthService,
	},
	UserToken,
	UserTokenRequest,
	User,
	LoginRequest,
	RegisterBasicUserRequest,
	DeleteUserRequest,
	Empty,
	LogoutUserRequest,
	BasicChangePasswordRequest,
	UserTeamRoles,
	UserId,
};
use log::*;

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
		info!("Login request from {:?}: {:?}", addr, &username);
		match self.db.basic_login(username, password).await {
			Ok(user_token) => Ok(Response::new(user_token)),
			Err(_) => Err(Status::unauthenticated("Invalid credentials"))
		}
	}

	async fn register_user_basic_login(
		&self,
		request: Request<RegisterBasicUserRequest>,
	) -> Result<Response<UserToken>, Status> {
		let addr = request.remote_addr().unwrap();
		let RegisterBasicUserRequest { name, email, password } = request.into_inner();
		info!("Register request from {:?}: {}", addr, &email);
		match self.db.register_user_basic_login(name, email, password).await {
			Ok(user_token) => Ok(Response::new(user_token)),
			Err(_) => Err(Status::unauthenticated("Invalid credentials"))
		}
	}

	async fn basic_change_password(
		&self,
		request: Request<BasicChangePasswordRequest>,
	) -> Result<Response<Empty>, Status> {
		let addr = request.remote_addr().unwrap();
		let BasicChangePasswordRequest { token, user_id, new_password } = request.into_inner();
		info!("Change password request from {:?}: {}", addr, &user_id);
		let user_id: Uuid = match user_id.parse() {
			Ok(id) => id,
			Err(_) => return Err(Status::invalid_argument("Invalid user_id"))
		};
		self.db.basic_change_password(token, &user_id, new_password).await?;
		Ok(Response::new(Empty {}))
	}

	async fn delete_user(
		&self,
		request: Request<DeleteUserRequest>,
	) -> Result<Response<Empty>, Status> { // TODO protect
		let addr = request.remote_addr().unwrap();
		let DeleteUserRequest { user_id } = request.into_inner();
		info!("Delete request from {:?}: {}", addr, &user_id);
		let user_id: Uuid = match user_id.parse() {
			Ok(id) => id,
			Err(_) => return Err(Status::invalid_argument("Invalid user_id"))
		};
		self.db.delete_user(&user_id, &user_id).await?;
		Ok(Response::new(Empty {}))
	}

	async fn me(
		&self,
		request: Request<UserTokenRequest>,
	) -> Result<Response<User>, Status> {
		Ok(Response::new(self.db.me(&request.into_inner().token).await?))
	}

	async fn logout(
		&self,
		request: Request<UserTokenRequest>,
	) -> Result<Response<Empty>, Status> {
		self.db.logout(&request.into_inner().token).await?;
		Ok(Response::new(Empty {}))
	}

	async fn logout_user(
		&self,
		request: Request<LogoutUserRequest>,
	) -> Result<Response<Empty>, Status> {
		let request = request.into_inner();
		let user_id: Uuid = match request.user_id.parse() {
			Ok(id) => id,
			Err(_) => return Err(Status::invalid_argument("Invalid user_id"))
		};
		self.db.logout_user(&request.token, &user_id).await?;
		Ok(Response::new(Empty {}))
	}

	async fn logout_everyone(
		&self,
		request: Request<UserTokenRequest>,
	) -> Result<Response<Empty>, Status> {
		self.db.logout_everyone(&request.into_inner().token).await?;
		Ok(Response::new(Empty {}))
	}

	async fn refresh_token(
		&self,
		request: Request<UserTokenRequest>,
	) -> Result<Response<UserToken>, Status> {
		Ok(Response::new(self.db.refresh_token(&request.into_inner().token).await?))
	}

	async fn team_roles(
		&self,
		request: Request<UserId>,
	) -> Result<Response<UserTeamRoles>, Status> {
		let user_id: Uuid = match request.into_inner().id.parse() {
			Ok(id) => id,
			Err(_) => return Err(Status::invalid_argument("Invalid user_id"))
		};
		Ok(Response::new(self.db.team_roles(&user_id).await?))
	}
}
