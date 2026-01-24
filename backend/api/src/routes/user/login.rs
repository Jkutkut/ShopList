use super::*;

#[post("/basic", data = "<credentials>")]
pub async fn basic(
	credentials: Json<ApiBasicCredentials>,
	cache_client: &State<Cache>,
	cookie_jar: &CookieJar<'_>,
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
	cache_client.cache_user_token(&user_data).await.map_err(|e| invalid_api(&e))?;
	cookie::add_user_token_cookie(cookie_jar, &user_data);
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
