use rocket::{
	Build, Rocket,
	launch, routes, catchers,
	get,
};
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
	use rocket::http::{Cookie, CookieJar};
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
		cookies: &CookieJar<'_>,
	) -> Result<Json<String>, InvalidResponse> {
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

		cookies.add_private(Cookie::new("bearer", response.token.clone()));

		println!("Response: {:#?}", response);

		Err(invalid_api("WIP: Not implemented"))
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
		match req.cookies().get_private("bearer") {
			Some(cookie) => request::Outcome::Success(User {
				token: cookie.value().to_string(),
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
		.mount("/", routes![
			ping,
			me,
			login::basic
		])
		.register("/", catchers![
			route_error::not_implemented,
			route_error::unauthorized,
			route_error::not_found,
			route_error::internal_server_error,
		])
}
