use crate::*;
use serde_json::{
	json,
	Value as JsonValue,
};
use rocket::{
	http::{
		Status,
		Header,
	},
	local::asynchronous::{
		Client,
		LocalResponse,
	},
};
use model::{
	grpc::auth::*,
};

mod openapi;

struct Test {
	client: Client,
}

async fn setup() -> Test {
	let _ = env_logger::Builder::from_env(
		env_logger::Env::default()
			.default_filter_or("api=debug")
	).is_test(true).format_timestamp(None).try_init();

	let rocket_instance = rocket().await;

	let mut grpc_connect_attempts = 10;
	while grpc_connect_attempts > 0 {
		match grpc::connect_auth().await {
			Ok(_) => break,
			Err(e) => {
				error!("Failed to connect to auth grpc: {}", e);
				std::thread::sleep(std::time::Duration::from_secs(1));
			}
		}
		grpc_connect_attempts -= 1;
	}

	let client = Client::tracked(rocket_instance).await.unwrap();
	Test {
		client,
	}
}

async fn check_response(
	res: &LocalResponse<'_>,
	status: Status,
	content_type: &str,
) {
	assert_eq!(res.status(), status, "Response status is {} but should be {}", res.status(), status);
	assert_eq!(
		res.content_type(), Some(content_type.parse().unwrap()),
		"Response content type is {} but should be {}", res.content_type().unwrap(), content_type
	);
}

async fn check_json_response(
	res: &LocalResponse<'_>,
) {
	check_response(res, Status::Ok, "application/json").await;
}

fn create_user_credentials(key: &str) -> JsonValue {
	json!({
		"name": format!("test-{}", key),
		"email": format!("{}-test@test.com", key),
		"password": format!("test-{}-password", key),
	})
}

async fn create_user(test: &Test, key: &str) -> UserToken {
	let credentials = create_user_credentials(key);
	let req = test.client.post("/api/v1/user/register/basic").json(&credentials);
	let res = req.dispatch().await;
	check_json_response(&res).await;
	res.into_json().await.unwrap()
}

async fn delete_self_user(test: &Test, user_token: &UserToken) {
	let UserToken { user_id, token, .. } = user_token;
	let endpoint = format!("/api/v1/user/{user_id}");
	let auth = format!("Bearer {}", token);
	let res = test.client.delete(&endpoint)
		.header(Header::new("Authorization", auth))
		.dispatch().await;
	assert_eq!(res.status(), Status::Ok);
}

// GET /api
#[tokio::test]
async fn ping() {
	let test = setup().await;

	let req = test.client.get("/api");
	let res = req.dispatch().await;

	check_json_response(&res).await;

	let res_str = res.into_string().await.unwrap();
	for message in ["api", "is up", "running"] {
		assert!(res_str.contains(message), "Response body is {} but should contain {}", res_str, message);
	}
}

// POST /api/v1/user/register/basic
// DELETE /api/v1/user/<user_id>
#[tokio::test]
async fn basic_register() {
	let test = setup().await;
	let test = &test;
	let user_token = create_user(test, "basic_register").await;
	delete_self_user(test, &user_token).await;
}
