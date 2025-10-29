mod login;
mod register;

use super::*;

#[delete("/<user_id>")]
pub async fn delete_user(
	user_id: UuidWrapper,
	#[allow(unused_variables)]
	user: guards::User // TODO
) -> Result<(), InvalidResponse> {
	info!("Delete request: {:?}", user_id);
	let user_id: Uuid = match user_id.get() {
		Ok(id) => id,
		Err(_) => return Err(InvalidResponse::new(Status::BadRequest, "Invalid user id"))
	};
	let mut auth_grpc_client = grpc::connect_auth().await.unwrap();
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

#[get("/me")]
pub fn me(user: guards::User) -> Json<guards::User> {
	info!("user request: {:#?}", &user);
	Json(user)
}

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/user",
		routes![
			delete_user,
			me,
		],
		catchers![],
		vec![
			login::routes(),
			register::routes(),
		],
	)
}
