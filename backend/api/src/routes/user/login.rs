use super::*;

#[post("/basic", data = "<credentials>")]
pub async fn basic(
	credentials: Json<ApiBasicCredentials>,
	cache_client: &State<Cache>,
) -> Result<ApiUserToken<UserToken>, InvalidResponse> {
	info!("Login request");
	debug!("Credentials: {:?}", credentials);

	let mut auth_grpc_client = grpc::connect_auth().await.unwrap();
	let auth_request = tonic::Request::new(LoginRequest {
		username: credentials.username.clone(),
		password: credentials.password.clone(),
	});

	let response = auth_grpc_client.basic_login(auth_request).await;
	if let Err(e) = response {
		return Err(invalid_api(&format!("GRPC error: {:?}", e)));
	}
	let response: UserToken = response.unwrap().into_inner();

	let expires_at = DateTime::parse_from_rfc3339(&response.expires_at).map_err(|_| {
		error!("Failed to parse as datetime expires_at: {}", &response.expires_at);
		invalid_api("Failed to parse as datetime expires_at")
	})?;
	let now = Utc::now();
	let expiration = cache::Expiration::EX(expires_at.signed_duration_since(now).num_seconds());
	cache_client.set(
		&response.token,
		&response,
		Some(expiration),
	).await;
	Ok(ApiUserToken::new(response.token.clone(), response))
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
