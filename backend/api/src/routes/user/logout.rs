use super::*;

#[post("/")]
async fn logout_current_user(
	#[allow(unused_variables)]
	user: guards::User,
	session_token: guards::SessionToken,
	cache_client: &State<Cache>,
	cookie_jar: &CookieJar<'_>,
) -> Result<(), InvalidResponse> {
	info!("logout request: {}", user.id);
	let session_token = session_token.to_string();
	let mut auth_grpc_client = grpc::connect_auth().await.unwrap();
	let auth_request = tonic::Request::new(UserTokenRequest {
		token: session_token.clone(),
	});
	match auth_grpc_client.logout(auth_request).await {
		Ok(_) => info!("Logout successful"),
		Err(e) => {
			error!("Logout failed: {}", e);
			return Err(invalid_api(&format!("GRPC error: {:?}", e)));
		}
	};
	cache_client.flush_user_token(
		&session_token, &user.id.get().unwrap()
	).await.map_err(|e| invalid_api(&e))?;
	cookie_jar.remove(Cookie::from("jwt"));
	Ok(())
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
