use rocket::{
	Build, Rocket,
	launch,
};

mod cors;
mod route_handler;
mod route_error;
mod api_user_token;
mod routes;

use me::User; // TODO refactor
mod me {
	use rocket::{
		Request,
		request,
		request::FromRequest,
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
}

#[launch]
async fn rocket() -> Rocket<Build> {
	let mut r = rocket::build()
		.attach(cors::CORS);
	let api = routes::routes().build();
	for (path, routes) in api.routes {
		r = r.mount(&path, routes);
	}
	for (path, catcher) in api.catchers {
		r = r.register(&path, catcher);
	}
	r
}
