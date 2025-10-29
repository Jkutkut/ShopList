use super::*;

#[post("/")]
async fn logout_current_user(
	#[allow(unused_variables)]
	user: guards::User
) -> Result<(), InvalidResponse> {
	Err(route_error::not_implemented()) // TODO
}

#[post("/<user_id>")]
async fn logout_user(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	user_id: UuidWrapper
) -> Result<(), InvalidResponse> {
	Err(route_error::not_implemented()) // TODO
}

#[post("/everyone")]
async fn logout_everyone(
	#[allow(unused_variables)]
	user: guards::User
) -> Result<(), InvalidResponse> {
	Err(route_error::not_implemented()) // TODO
}

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/logout",
		routes![
			logout_current_user,
			logout_user,
			logout_everyone,
		],
		catchers![],
		vec![],
	)
}
