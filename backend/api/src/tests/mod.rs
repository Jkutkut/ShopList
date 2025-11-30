use crate::*;
use rocket::{http::Status, local::asynchronous::Client};

struct Test {
	client: Client,
}

async fn setup() -> Test {
	let rocket_instance = rocket().await;
	let client = Client::tracked(rocket_instance).await.unwrap();
	Test { client }
}

#[tokio::test]
async fn ping() {
	let test = setup().await;

	let req = test.client.get("/api");
	let res = req.dispatch().await;

	assert_eq!(res.status(), Status::Ok);
	assert_eq!(res.content_type(), Some("application/json".parse().unwrap()));
	let res_str = res.into_string().await.unwrap();
	assert!(res_str.contains("api"));
	assert!(res_str.contains("is up"));
	assert!(res_str.contains("running"));
}
