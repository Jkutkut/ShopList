use rocket::{
	Request,
	request::{
		FromRequest,
		Outcome,
	},
	http::Status,
};
use serde::{
	Deserialize, Serialize,
};
use log::*;
use crate::{
	grpc,
	cache,
};
use model::UuidWrapper;
use model::grpc::auth::{
	UserToken,
	User as GrpcUser
};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
	uuid: UuidWrapper,
	name: String,
	created_at: String,
	updated_at: String,
	is_superuser: bool,
	image: Option<String>
}

impl TryFrom<GrpcUser> for User {
	type Error = ();
	fn try_from(value: GrpcUser) -> Result<Self, Self::Error> {
		let uuid = UuidWrapper::try_from(value.uuid.as_str()).map_err(|_| ())?;
		Ok(User {
			uuid,
			name: value.name,
			created_at: value.created_at,
			updated_at: value.updated_at,
			is_superuser: value.is_superuser,
			image: value.image,
		})
	}
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
	type Error = ();

	async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let invalid = || Outcome::Error((Status::Unauthorized, ()));
		let expiration = Some(cache::Expiration::EX(15 * 60)); // TODO
		info!("User guard");
		let authorization = req.headers().get_one("Authorization");
		let token = match authorization {
			Some(token) => token.split_once("Bearer ").unwrap().1.to_string(),
			_ => return invalid(),
		};
		debug!("Authentication token: {}", &token);
		let cache_client = req.rocket().state::<cache::Client>().unwrap();
		let cache_key = token.clone();
		let try_get_user = || async {
			info!("Attempt to get user from grpc");
			let mut auth_grpc_client = grpc::connect_auth().await.unwrap();
			let auth_request = tonic::Request::new(UserToken { token });
			let user = match auth_grpc_client.me(auth_request).await {
				Ok(response) => {
					let response = response.into_inner();
					debug!("user: {:#?}", response);
					match Self::try_from(response) {
						Ok(user) => user,
						Err(_) => return Err(()),
					}
				}
				Err(_) => return Err(()),
			};
			Ok(user)
		};
		match cache::cached_value(
			&cache_client, &cache_key, expiration,
			try_get_user
		).await {
			Ok(user) => Outcome::Success(user),
			Err(_) => invalid(),
		}
	}
}
