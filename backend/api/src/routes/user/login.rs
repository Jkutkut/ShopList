use super::*;

#[post("/basic", data = "<credentials>")]
pub async fn basic(
	credentials: Json<ApiBasicCredentials>,
	cache_client: &State<Cache>,
) -> Result<ApiUserToken<UserToken>, InvalidResponse> {
	info!("Login request");
	debug!("Credentials: {:?}", credentials);

	let mut auth_grpc_client = grpc::connect_auth().await.unwrap();
	let auth_request = tonic::Request::new(BasicLoginRequest {
		email: credentials.email.clone(),
		password: credentials.password.clone(),
	});
	debug!("grpc login");
	let user_data = match auth_grpc_client.basic_login(auth_request).await {
		Ok(response) => response.into_inner(),
		Err(e) => return Err(invalid_api(&format!("GRPC error: {:?}", e))),
	};

	let expires_at = DateTime::parse_from_rfc3339(&user_data.expires_at).map_err(|_| {
		error!("Failed to parse as datetime expires_at: {}", &user_data.expires_at);
		invalid_api("Failed to parse as datetime expires_at")
	})?;
	let now = Utc::now();
	let expiration = cache::Expiration::EX(expires_at.signed_duration_since(now).num_seconds());
	cache_client.set(
		&user_data.token,
		&user_data,
		Some(expiration),
	).await;
	cache_client.sadd(format!("user_token:{}", user_data.user_id).as_str(), &user_data.token).await;
	Ok(ApiUserToken::new(user_data.token.clone(), user_data))
}

#[post("/basic/password", data = "<credentials>")]
pub async fn basic_change_password(
	#[allow(unused_variables)]
	credentials: Json<ApiBasicCredentials>,
) -> Result<ApiUserToken<UserToken>, InvalidResponse> {
	Err(route_error::not_implemented()) // TODO
}

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/login",
		routes![
			basic,
			basic_change_password,
		],
		catchers![],
		vec![],
	)
}
