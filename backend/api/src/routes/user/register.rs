use super::*;

#[post("/basic", data = "<credentials>")]
pub async fn basic(
	credentials: Json<ApiRegisterBasicCredentials>,
) -> Result<ApiUserToken<UserToken>, InvalidResponse> {
	println!("Credentials: {:?}", credentials);

	let mut auth_grpc_client = AuthServiceClient::connect("http://shoplist-auth:50051").await.unwrap();
	let auth_request = tonic::Request::new(RegisterBasicUserRequest {
		name: credentials.name.clone(),
		email: credentials.email.clone(),
		password: credentials.password.clone(),
	});

	let response = auth_grpc_client.register_user_basic_login(auth_request).await;
	if let Err(e) = response {
		return Err(invalid_api(&format!("GRPC error: {:?}", e)));
	}
	let response: UserToken = response.unwrap().into_inner();
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
