use rocket::{
	Request,
	request::{
		FromRequest,
		Outcome,
	},
	http::Status,
};
use model::grpc::auth::{
	UserToken,
	User as GrpcUser
};
use model::UuidWrapper;
use crate::grpc;
use log::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
		let authorization = req.headers().get_one("Authorization");
		debug!("User from request token {:#?}", &authorization);
		let token = match authorization {
			Some(token) => token.split_once("Bearer ").unwrap().1.to_string(),
			_ => return Outcome::Error((Status::Unauthorized, ())),
		};
		let mut auth_grpc_client = grpc::connect_auth().await.unwrap();
		let auth_request = tonic::Request::new(UserToken { token });
		match auth_grpc_client.me(auth_request).await {
			Ok(response) => {
				let response = response.into_inner();
				debug!("user: {:#?}", response);
				match Self::try_from(response).map_err(|_| ()) {
					Ok(user) => Outcome::Success(user),
					Err(_) => Outcome::Error((Status::Unauthorized, ())),
				}
			}
			Err(_) => Outcome::Error((Status::Unauthorized, ())),
		}
	}
}
