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
	UserTokenRequest,
	User as GrpcUser,
	UserToken,
};

fn bearer_token_from(req: &Request) -> Option<String> {
	let authorization = match req.headers().get_one("Authorization") {
		Some(token) => token,
		_ => return None
	};
	match authorization.split_once("Bearer ") {
		Some((_, token)) => {
			#[cfg(debug_assertions)]
			debug!("Bearer token: {:?}", token);
			Some(token.to_string())
		},
		_ => {
			warn!("Bearer token not found");
			None
		}
	}
}

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
		let expiration = Some(cache::Expiration::EX(15 * 60)); // TODO use login as expiration handler
		info!("Authenticating user based on token...");
		let token = match bearer_token_from(req) {
			Some(token) => token,
			_ => return invalid(),
		};
		let cache_client = req.rocket().state::<cache::Cache>().unwrap();

		let try_get_user = || async {
			info!("Attempt to get user from grpc");
			let mut auth_grpc_client = grpc::connect_auth().await.unwrap();
			let auth_request = tonic::Request::new(UserTokenRequest { token: token.clone() });
			let user = match auth_grpc_client.me(auth_request).await {
				Ok(response) => {
					let response = response.into_inner();
					match Self::try_from(response) {
						Ok(user) => user,
						Err(e) => {
							error!("{:?}", e);
							return Err(())
						}
					}
				},
				Err(e) => {
					error!("{}", e);
					return Err(())
				}
			};
			Ok(user)
		};

		match cache_client.try_get::<UserToken>(token.as_str()).await {
			Some(user_token) => {
				let user_id = user_token.user_id;
				match cache_client.cached_value(
					&user_id, expiration,
					try_get_user,
				).await {
					Ok(user) => Outcome::Success(user),
					_ => invalid(),
				}
			},
			None => {
				let user = match try_get_user().await {
					Ok(user) => {
						#[cfg(debug_assertions)]
						debug!("user: {:#?}", user);
						user
					},
					Err(_) => return invalid(),
				};
				cache_client.set(user.uuid.to_string().as_str(), &user, expiration).await;
				Outcome::Success(user)
			}
		}
	}
}
