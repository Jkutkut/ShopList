use rocket::{
	Config, Build, Rocket,
	launch, routes, catchers,
	get,
};
use std::net::Ipv4Addr;
use rocket::serde::json::Json;

mod route_error;

pub mod auth {
	tonic::include_proto!("auth");
}

#[get("/")]
fn ping() -> Json<&'static str> {
	Json(concat!("shoplist-", env!("CARGO_BIN_NAME"), " is up and running"))
}

mod login {
	use rocket::serde::json::Json;
	use rocket::post;
	use serde::Deserialize;

	use crate::route_error::{InvalidResponse, invalid_api};
	use crate::auth::auth_service_client::AuthServiceClient;
	use crate::auth::AuthResponse;
	use crate::auth::LoginRequest;

	#[derive(Debug, Deserialize)]
	pub struct BasicCredentials {
		username: String,
		password: String
	}

	#[post("/login/basic", data = "<credentials>")]
	pub async fn basic(
		credentials: Json<BasicCredentials>,
	) -> Result<Json<String>, InvalidResponse> {
		println!("Credentials: {:?}", credentials);

		let mut auth_grpc_client = AuthServiceClient::connect("http://0.0.0.0:50051").await.unwrap();
		let auth_request = tonic::Request::new(LoginRequest {
			username: credentials.username.clone(),
			password: credentials.password.clone(),
		});

		let response = auth_grpc_client.basic_login(auth_request).await;
		if let Err(e) = response {
			return Err(invalid_api(&format!("GRPC error: {:?}", e)));
		}
		let response: AuthResponse = response.unwrap().into_inner();

		println!("Response: {:#?}", response);

		Err(invalid_api("WIP: Not implemented"))
	}
}

fn config() -> Config {
	#[cfg(debug_assertions)]
	{
		Config::debug_default()
	}
	#[cfg(not(debug_assertions))]
	{
		Config::release_default()
	}
}

#[launch]
async fn rocket() -> Rocket<Build> {
	let port = 80;

	let config = Config {
		address: Ipv4Addr::new(0, 0, 0, 0).into(),
		port,
		..config()
	};
	rocket::custom(&config)
		.mount("/", routes![
			ping,
			login::basic
		])
		.register("/", catchers![
			route_error::not_implemented,
			route_error::not_found,
			route_error::internal_server_error,
		])
}
