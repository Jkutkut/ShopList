use super::*;

#[post("/basic", data = "<credentials>")]
pub async fn basic(
	credentials: Json<ApiRegisterBasicCredentials>,
	cache_client: &State<Cache>,
) -> Result<ApiUserToken<UserToken>, InvalidResponse> {
	info!("Registering user");
	debug!("Credentials: {:?}", credentials);

	let mut auth_grpc_client = grpc::connect_auth().await.unwrap();
	let ApiRegisterBasicCredentials { name, email, password } = credentials.into_inner();
	let auth_request = tonic::Request::new(RegisterBasicUserRequest {
		name, email, password
	});

	let response = auth_grpc_client.register_user_basic_login(auth_request).await;
	if let Err(e) = response {
		return Err(invalid_api(&format!("GRPC error: {:?}", e)));
	}
	let response: UserToken = response.unwrap().into_inner();
	cache_client.cache_user_token(&response).await.map_err(|e| invalid_api(&e))?;
	Ok(ApiUserToken::new(response.token.clone(), response))
}

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/register",
		routes![
			basic,
		],
		catchers![],
		vec![],
	)
}
