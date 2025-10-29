mod login;
mod logout;
mod register;

use super::*;

#[get("/<user_id>")]
async fn get_user(
	#[allow(unused_variables)]
	user_id: UuidWrapper,
) -> Result<Json<guards::User>, InvalidResponse> {
	Err(route_error::not_implemented()) // TODO
}

#[delete("/<user_id>")]
async fn delete_user(
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
fn me(user: guards::User) -> Json<guards::User> {
	info!("user request: {:#?}", &user);
	Json(user)
}

#[post("/me/token")]
fn refresh_token(
	#[allow(unused_variables)]
	user: guards::User
) -> Result<Json<guards::User>, InvalidResponse> {
	Err(route_error::not_implemented()) // TODO
}

#[post("/superuser/<user_id>")]
fn set_user_as_superuser(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	user_id: UuidWrapper
) -> Result<Json<guards::User>, InvalidResponse> {
	Err(route_error::not_implemented()) // TODO
}

#[delete("/superuser/<user_id>")]
fn delete_as_superuser(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	user_id: UuidWrapper
) -> Result<Json<guards::User>, InvalidResponse> {
	Err(route_error::not_implemented()) // TODO
}

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/user",
		routes![
			get_user,
			delete_user,
			me,
			refresh_token,
			set_user_as_superuser,
			delete_as_superuser,
		],
		catchers![],
		vec![
			login::routes(),
			logout::routes(),
			register::routes(),
		],
	)
}
