use rocket::{
	Request,
	request::{
		FromRequest,
		Outcome,
	},
	http::{
		Status,
		Cookie,
	},
};
use serde::{
	Deserialize, Serialize,
};
use log::*;
use crate::{
	grpc,
	cache,
	db,
};
use model::UuidWrapper;
use model::grpc::auth::{
	UserTokenRequest,
	User as GrpcUser,
	UserToken,
};
use uuid::Uuid;

fn bearer_token_from(req: &Request) -> Option<String> {
	let token = match bearer_token_from_cookie(req) {
		Some(token) => Some(token),
		_ => bearer_token_from_header(req)
	};
	match &token {
		Some(token) => {
			#[cfg(debug_assertions)]
			debug!("Bearer token: {:?}", token);
		},
		_ => {
			warn!("Bearer token not found");
		}
	};
	token
}

fn bearer_token_from_cookie(req: &Request) -> Option<String> {
	debug!("bearer_token_from: cookie");
	match req.cookies().get("jwt") {
		Some(token) => {
			#[cfg(debug_assertions)]
			debug!("token found in cookie jwt: {}", token.value());
			Some(token.value().to_string())
		},
		_ => return None
	}
}

fn bearer_token_from_header(req: &Request) -> Option<String> {
	debug!("bearer_token_from_header");
	let authorization = match req.headers().get_one("Authorization") {
		Some(token) => token,
		_ => return None
	};
	match authorization.split_once("Bearer ") {
		Some((_, token)) => {
			#[cfg(debug_assertions)]
			debug!("token found in header Authorization: {}", token);
			Some(token.to_string())
		},
		_ => None
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
	pub id: UuidWrapper,
	pub name: String,
	pub created_at: String,
	pub updated_at: String,
	pub is_superuser: bool,
	pub image: Option<String>
}

impl TryFrom<GrpcUser> for User {
	type Error = ();
	fn try_from(value: GrpcUser) -> Result<Self, Self::Error> {
		let id = UuidWrapper::try_from(value.id.as_str()).map_err(|_| ())?;
		Ok(User {
			id,
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
		let invalid = || {
			let cookie_jar = req.cookies();
			cookie_jar.remove(Cookie::from("jwt"));
			Outcome::Error((Status::Unauthorized, ()))
		};
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
				debug!("cached token belongs to user {}", user_id);
				match cache_client.cached_value(
					&user_id, expiration,
					try_get_user,
				).await {
					Ok(user) => {
						#[cfg(debug_assertions)]
						debug!("user: {:#?}", user);
						Outcome::Success(user)
					},
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
				cache_client.set(user.id.to_string().as_str(), &user, expiration).await;
				Outcome::Success(user)
			}
		}
	}
}

#[derive(Debug)]
pub struct SessionToken(String);

impl SessionToken {
	pub fn new(token: String) -> Self {
		Self(token)
	}

	pub fn to_string(self) -> String {
		self.0
	}
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionToken {
	type Error = ();

	async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let token = match bearer_token_from(req) {
			Some(token) => token,
			_ => return Outcome::Error((Status::Unauthorized, ())),
		};
		Outcome::Success(Self::new(token))
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
	pub id: Uuid,
	pub name: String,
	pub display_name: Option<String>,
	pub description: Option<String>,
	pub image: Option<String>,
	pub created_at: String,
	pub created_by: Uuid,
	pub updated_at: String,
	pub updated_by: Uuid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Team {
	type Error = ();

	async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		info!("FromRequest Team");
		let user_id = match req.guard::<User>().await {
			Outcome::Success(user) => user.id.get().unwrap(),
			_ => return Outcome::Error((Status::BadRequest, ())),
		};
		// /api/v1/team/<team_id>,
		// but <team_id> is the first param in route def
		// TODO this may break in the future
		const PARAM_IDX: usize = 0;
		let team_id = match req.param::<UuidWrapper>(PARAM_IDX) {
			Some(Ok(team_id)) => match team_id.get() {
				Ok(team_id) => team_id,
				Err(_) => return Outcome::Error((Status::BadRequest, ())),
			},
			_ => return Outcome::Error((Status::BadRequest, ())),
		};
		debug!("team_id: {:?}", team_id);
		let db = req.rocket().state::<db::DB>().unwrap();
		let team = match db.get_team(&team_id, &user_id).await {
			Ok(team) => team,
			Err(_) => return Outcome::Error((Status::BadRequest, ())),
		};
		debug!("team: {:#?}", team);
		Outcome::Success(team)
	}
}
