use crate::*;
use rocket::{
	http::Status,
	local::asynchronous::{
		Client,
		LocalResponse,
	},
};

struct Test {
	client: Client,
}

async fn setup() -> Test {
	let _ = env_logger::Builder::from_env(
		env_logger::Env::default()
			.default_filter_or("api=debug")
	).is_test(true).format_timestamp(None).try_init();

	let rocket_instance = rocket().await;
	let client = Client::tracked(rocket_instance).await.unwrap();
	Test {
		client,
	}
}

async fn check_response(res: LocalResponse<'_>, status: Status, content_type: &str, message_contains: &[&str]) {
	assert_eq!(res.status(), status, "Response status is {} but should be {}", res.status(), status);
	assert_eq!(res.content_type(), Some(content_type.parse().unwrap()), "Response content type is {} but should be {}", res.content_type().unwrap(), content_type);
	let res_str = res.into_string().await.unwrap();
	for message in message_contains {
		assert!(res_str.contains(message), "Response body is {} but should contain {}", res_str, message);
	}
}

async fn check_json_response(res: LocalResponse<'_>, message_contains: &[&str]) {
	check_response(res, Status::Ok, "application/json", message_contains).await;
}

#[tokio::test]
async fn ping() {
	let test = setup().await;

	let req = test.client.get("/api");
	let res = req.dispatch().await;
	check_json_response(res, &["api", "is up", "running"]).await;
}
