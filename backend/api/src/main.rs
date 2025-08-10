use rocket::{
	Build, Rocket,
	launch, routes, catchers,
	get,
};
use rocket::serde::json::Json;

mod cors;
mod route_error;
mod api_user_token;

#[get("/")]
fn ping() -> Json<&'static str> {
	Json(concat!("shoplist-", env!("CARGO_BIN_NAME"), " is up and running"))
}

mod login {
	use rocket::serde::json::Json;
	use rocket::post;
	use model::ApiBasicCredentials;

	use crate::route_error::{InvalidResponse, invalid_api};
	use model::grpc::auth::{
		auth_service_client::AuthServiceClient,
		UserToken,
		LoginRequest,
	};
	use crate::api_user_token::ApiUserToken;

	#[post("/user/login/basic", data = "<credentials>")]
	pub async fn basic(
		credentials: Json<ApiBasicCredentials>,
	) -> Result<ApiUserToken<UserToken>, InvalidResponse> {
		println!("Credentials: {:?}", credentials);

		let mut auth_grpc_client = AuthServiceClient::connect("http://shoplist-auth:50051").await.unwrap();
		let auth_request = tonic::Request::new(LoginRequest {
			username: credentials.username.clone(),
			password: credentials.password.clone(),
		});

		let response = auth_grpc_client.basic_login(auth_request).await;
		if let Err(e) = response {
			return Err(invalid_api(&format!("GRPC error: {:?}", e)));
		}
		let response: UserToken = response.unwrap().into_inner();
		Ok(ApiUserToken::new(response.token.clone(), response))
	}
}

mod register {
	use rocket::serde::json::Json;
	use rocket::post;

	use crate::api_user_token::ApiUserToken;
	use crate::route_error::{InvalidResponse, invalid_api};
	use model::grpc::auth::{
		auth_service_client::AuthServiceClient,
		UserToken,
		RegisterBasicUserRequest,
	};
	use model::ApiRegisterBasicCredentials;

	#[post("/register/basic", data = "<credentials>")]
	pub async fn basic(
		credentials: Json<ApiRegisterBasicCredentials>,
	) -> Result<ApiUserToken<UserToken>, InvalidResponse> {
		println!("Credentials: {:?}", credentials);

		let mut auth_grpc_client = AuthServiceClient::connect("http://shoplist-auth:50051").await.unwrap();
		let auth_request = tonic::Request::new(RegisterBasicUserRequest {
			name: credentials.name.clone(),
			email: credentials.email.clone(),
			password: credentials.password.clone(),
		});

		let response = auth_grpc_client.register_user_basic_login(auth_request).await;
		if let Err(e) = response {
			return Err(invalid_api(&format!("GRPC error: {:?}", e)));
		}
		let response: UserToken = response.unwrap().into_inner();
		Ok(ApiUserToken::new(response.token.clone(), response))
	}
}

mod user {
	use uuid::Uuid;
	use rocket::{
		delete,
		http::Status,
	};
	use crate::{
		route_error::InvalidResponse,
		User,
	};
	use model::{
		UuidWrapper,
		grpc::auth::{
			auth_service_client::AuthServiceClient,
			DeleteUserRequest,
		},
	};

	#[delete("/user/<user_id>")]
	pub async fn delete_user(
		user_id: UuidWrapper,
		#[allow(unused_variables)]
		user: User // TODO
	) -> Result<(), InvalidResponse> {
		let user_id: Uuid = match user_id.get() {
			Ok(id) => id,
			Err(_) => return Err(InvalidResponse::new(Status::BadRequest, "Invalid user id"))
		};
		println!("Delete request: {:?}", user_id);
		let mut auth_grpc_client = AuthServiceClient::connect("http://shoplist-auth:50051").await.unwrap();
		let auth_request = tonic::Request::new(DeleteUserRequest {
			user_id: user_id.to_string()
		});
		match auth_grpc_client.delete_user(auth_request).await {
			Ok(_) => Ok(()),
			Err(e) => match e.code() {
				// Code::NotFound
				// Code::PermissionDenied
				_ => Err(InvalidResponse::new(Status::Unauthorized, "Invalid credentials"))
			}
		}
	}
}

use me::User; // TODO refactor
mod me {
	use rocket::{
		get,
		Request,
		request,
		request::FromRequest,
		serde::json::Json,
	};
	use model::grpc::auth::{
		auth_service_client::AuthServiceClient,
		UserToken,
	};
	use model::UuidWrapper;

	#[derive(Debug, serde::Serialize, serde::Deserialize)]
	pub struct User {
		uuid: UuidWrapper,
		name: String,
		created_at: String,
		updated_at: String,
		is_superuser: bool,
		image: Option<String>
	}

	#[rocket::async_trait]
	impl<'r> FromRequest<'r> for User {
		type Error = ();

		async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
			let token = match req.headers().get_one("Authorization") {
				Some(token) => token.split_once("Bearer ").unwrap().1.to_string(),
				_ => return request::Outcome::Error((rocket::http::Status::Unauthorized, ())),
			};
			println!("Token: {}", token);
			let mut auth_grpc_client = AuthServiceClient::connect("http://shoplist-auth:50051").await.unwrap();
			let auth_request = tonic::Request::new(UserToken { token: token.clone() });
			match auth_grpc_client.me(auth_request).await {
				Ok(response) => {
					let response = response.into_inner();
					println!("Response: {:#?}", response);
					request::Outcome::Success(Self { // TODO refactor
						uuid: UuidWrapper::try_from(response.uuid.as_str()).unwrap(),
						name: response.name,
						created_at: response.created_at,
						updated_at: response.updated_at,
						is_superuser: response.is_superuser,
						image: response.image
					})
				}
				Err(_) => request::Outcome::Error((rocket::http::Status::Unauthorized, ())),
			}
		}
	}

	#[get("/user/me")]
	pub fn me(user: User) -> Json<User> {
		println!("User: {:#?}", &user);
		Json(user)
	}
}

#[launch]
async fn rocket() -> Rocket<Build> {
	rocket::build()
		.attach(cors::CORS).mount("/", routes![cors::options])
		// .manage(auth_grpc_client) // TODO
		.mount("/api", routes![
			ping,
		])
		.mount("/api/v1", routes![
			me::me,
			login::basic,
			register::basic,
			user::delete_user
		])
		.register("/", catchers![
			route_error::not_implemented,
			route_error::unauthorized,
			route_error::not_found,
			route_error::internal_server_error,
		])
}
