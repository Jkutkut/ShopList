use rocket::{
	Build, Rocket,
	launch, routes, catchers,
	get,
};
use rocket::serde::json::Json;

mod cors;
mod route_error;
mod api_auth_response;

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
		AuthResponse,
		LoginRequest,
	};
	use crate::api_auth_response::ApiAuthResponse;

	#[post("/user/login/basic", data = "<credentials>")]
	pub async fn basic(
		credentials: Json<ApiBasicCredentials>,
	) -> Result<ApiAuthResponse<AuthResponse>, InvalidResponse> {
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
		let response: AuthResponse = response.unwrap().into_inner();
		Ok(ApiAuthResponse::new(response.token.clone(), response))
	}
}

mod register {
	use rocket::serde::json::Json;
	use rocket::post;
	use rocket::http::{Cookie, CookieJar};

	use crate::route_error::{InvalidResponse, invalid_api};
	use model::grpc::auth::{
		auth_service_client::AuthServiceClient,
		AuthResponse,
		RegisterBasicUserRequest,
	};
	use model::ApiRegisterBasicCredentials;

	#[post("/register/basic", data = "<credentials>")]
	pub async fn basic(
		credentials: Json<ApiRegisterBasicCredentials>,
		cookies: &CookieJar<'_>,
	) -> Result<Json<String>, InvalidResponse> {
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
		let response: AuthResponse = response.unwrap().into_inner();

		cookies.add_private(Cookie::new("bearer", response.token.clone()));

		println!("Response: {:#?}", response);

		Err(invalid_api(&format!(
			"WIP: Not implemented. response: {:#?}",
			response
		)))
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

use rocket::{
	Request,
	request,
	request::FromRequest,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct User {
	token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
	type Error = ();

	async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
		match req.headers().get_one("Authorization") {
			Some(token) => request::Outcome::Success(User { // TODO
				token: token.to_string(),
			}),
			_ => request::Outcome::Error((rocket::http::Status::Unauthorized, ())),
		}
	}
}

#[get("/me")]
fn me(user: User) -> Json<User> {
	println!("User: {:#?}", &user);
	Json(user)
}

#[launch]
async fn rocket() -> Rocket<Build> {
	rocket::build()
		.attach(cors::CORS).mount("/", routes![cors::options])
		// .manage(auth_grpc_client)
		.mount("/api", routes![
			ping,
		])
		.mount("/api/v1", routes![
			me,
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
		// .mount("/", routes![ping, cors::options])
		// .mount("/", rocket::fs::FileServer::from(PUBLIC_DIR))
		// .mount("/api/v1", api::get_v1_routes())
		// .register("/api", catchers![route_error::api_not_implemented])
}
