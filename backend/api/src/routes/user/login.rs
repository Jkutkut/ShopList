use super::*;

#[post("/basic", data = "<credentials>")]
pub async fn basic(
	credentials: Json<ApiBasicCredentials>,
) -> Result<ApiUserToken<UserToken>, InvalidResponse> {
	println!("Credentials: {:?}", credentials);

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
	Ok(ApiUserToken::new(response.token.clone(), response))
}

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/login",
		routes![
			basic,
		],
		catchers![],
		vec![],
	)
}
