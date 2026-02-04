use crate::*;
use std::time::Duration;
use std::thread::sleep;
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
	grpc::auth::{
		User,
		UserToken,
		UserTeamRoles,
		TeamRole,
	},
};
use crate::guards::Team;
use uuid::Uuid;

mod openapi;

struct Test {
	client: Client,
}

async fn new_client() -> Client {
	Client::tracked(rocket().await).await.unwrap()
}

async fn setup() -> Test {
	let _ = env_logger::Builder::from_env(
		env_logger::Env::default()
			.default_filter_or("api=debug")
	).is_test(true).format_timestamp(None).try_init();

	let mut grpc_connect_attempts = 50;
	while grpc_connect_attempts > 0 {
		match grpc::connect_auth().await {
			Ok(_) => break,
			Err(e) => {
				error!("Failed to connect to auth grpc: {}", e);
				sleep(Duration::from_millis(100));
			}
		}
		grpc_connect_attempts -= 1;
	}

	let client = new_client().await;
	Test {
		client,
	}
}

fn auth_header(token: &str) -> Header<'static> {
	Header::new("Authorization", format!("Bearer {}", token))
}

fn check_content_type(res: &LocalResponse<'_>, content_type: &str) {
	assert_eq!(
		res.content_type(), Some(content_type.parse().unwrap()),
		"Response content type is {} but should be {}", res.content_type().unwrap(), content_type
	);
}

fn check_json_content_type(res: &LocalResponse<'_>) {
	check_content_type(res, "application/json");
}

fn check_status(res: &LocalResponse<'_>, status: Status) {
	assert_eq!(res.status(), status, "Response status is {} but should be {}", res.status(), status);
}

async fn check_response(
	res: &LocalResponse<'_>,
	status: Status,
	content_type: &str,
) {
	check_status(res, status);
	check_content_type(res, content_type);
}

async fn check_json_response(
	res: &LocalResponse<'_>,
) {
	check_status(res, Status::Ok);
	check_json_content_type(res);
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
	{
		debug!("Ensure user does not exist");
		let db = test.client.rocket().state::<db::DB>().unwrap().client();
		let query = "DELETE FROM users WHERE name = $1";
		let user_name: String = credentials["name"].as_str().unwrap().to_string();
		let stmt = db.prepare(query).await.unwrap();
		db.execute(&stmt, &[&user_name]).await.unwrap();
	}
	let req = test.client.post("/api/v1/user/register/basic").json(&credentials);
	let res = req.dispatch().await;
	check_json_response(&res).await;
	let user_token = res.into_json().await.unwrap();
	debug!("Created user named {}: {:#?}", credentials["name"], user_token);
	user_token
}

async fn delete_self_user(test: &Test, user_token: &UserToken) {
	let UserToken { user_id, token, .. } = user_token;
	let endpoint = format!("/api/v1/user/{user_id}");
	let res = test.client.delete(&endpoint)
		.header(auth_header(&token))
		.dispatch().await;
	assert_eq!(res.status(), Status::Ok);
}

async fn fetch_me(test: &Test, user_token: &UserToken) -> User {
	let UserToken { token, .. } = user_token;
	let res = test.client.get("/api/v1/user/me")
		.header(auth_header(&token))
		.dispatch().await;
	check_json_response(&res).await;
	res.into_json().await.unwrap()
}

async fn fail_fetch_me(test: &Test, user_token: &UserToken) {
	let UserToken { token, .. } = user_token;
	let res = test.client.get("/api/v1/user/me")
		.header(auth_header(&token))
		.dispatch().await;
	check_response(&res, Status::Unauthorized, "application/json").await;
}

#[allow(dead_code)]
async fn fetch_user(test: &Test, user_token: &UserToken, user_id: &str) -> User {
	let UserToken { token, .. } = user_token;
	let endpoint = format!("/api/v1/user/{}", user_id);
	let res = test.client.get(&endpoint)
		.header(auth_header(&token))
		.dispatch().await;
	check_json_response(&res).await;
	res.into_json().await.unwrap()
}

async fn login_user(test: &Test, key: &str) -> UserToken {
	sleep(Duration::from_secs(1)); // Ensure the JWT token is different
	let credentials = create_user_credentials(key);
	let req = test.client.post("/api/v1/user/login/basic").json(&credentials);
	let res = req.dispatch().await;
	check_json_response(&res).await;
	res.into_json().await.unwrap()
}

async fn logout_user(test: &Test, user_token: &UserToken) {
	let UserToken { token, .. } = user_token;
	let res = test.client.post("/api/v1/user/logout")
		.header(auth_header(&token))
		.dispatch().await;
	check_status(&res, Status::Ok);
}

async fn create_team(test: &Test, user_token: &UserToken, team_name: &str) -> Uuid {
	{
		debug!("Ensure team does not exist");
		let db = test.client.rocket().state::<db::DB>().unwrap().client();
		let query = "DELETE FROM teams WHERE name = $1";
		let stmt = db.prepare(query).await.unwrap();
		db.execute(&stmt, &[&team_name]).await.unwrap();
	}
	let UserToken { token, .. } = user_token;
	let req = test.client.post("/api/v1/team")
		.header(auth_header(&token))
		.json(&json!({
			"name": team_name
		}));
	let res = req.dispatch().await;
	check_json_response(&res).await;
	res.into_json().await.unwrap()
}

async fn fetch_team(_: &Test, user_token: &UserToken, team_id: &Uuid) -> Team {
	let UserToken { token, .. } = user_token;
	let endpoint = format!("/api/v1/team/{}", team_id);
	let client = new_client().await;
	let res = client.get(&endpoint)
		.header(auth_header(&token))
		.dispatch().await;
	check_json_response(&res).await;
	res.into_json().await.unwrap()
}

async fn fail_fetch_team(_: &Test, user_token: &UserToken, team_id: &Uuid) {
	let UserToken { token, .. } = user_token;
	let endpoint = format!("/api/v1/team/{}", team_id);
	let client = new_client().await;
	let res = client.get(&endpoint)
		.header(auth_header(&token))
		.dispatch().await;
	check_response(&res, Status::BadRequest, "application/json").await;
}

async fn delete_team(_: &Test, user_token: &UserToken, team_id: &Uuid, status: Status) {
	info!("Deleting team \"{}\" by user {}", team_id, user_token.user_id);
	let UserToken { token, .. } = user_token;
	debug!("token: {:#?}", token);
	let endpoint = format!("/api/v1/team/{}", team_id);
	let client = new_client().await;
	let res = client.delete(&endpoint)
		.header(auth_header(&token))
		.dispatch().await;
	check_status(&res, status);
}

async fn fetch_user_team_roles(_: &Test, user_token: &UserToken) -> UserTeamRoles {
	let UserToken { token, .. } = user_token;
	let client = new_client().await;
	let res = client.get("/api/v1/team/roles")
		.header(auth_header(&token))
		.dispatch().await;
	check_json_response(&res).await;
	res.into_json().await.unwrap()
}

// -------------------------------------------

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

// GET /api/v1/user/me
// GET TODO /api/v1/user/<user_id>
#[tokio::test]
async fn get_user() {
	let test = setup().await;
	let user_token = create_user(&test, "get_user_me").await;
	let user = fetch_me(&test, &user_token).await;
	debug!("user: {:#?}", user);
	// assert_eq!(user, fetch_user(&test, &user_token, &user.uuid).await);
	assert_eq!(user.uuid, user_token.user_id);
	delete_self_user(&test, &user_token).await;
}

// POST /api/v1/user/login/basic
#[tokio::test]
async fn basic_login() {
	let test = setup().await;
	let key = "basic_login";
	let user_token = create_user(&test, key).await;
	let login = login_user(&test, key).await;
	assert_eq!(login.user_id, user_token.user_id);
	assert!(user_token.token != login.token);
	delete_self_user(&test, &user_token).await;
}

// POST /api/v1/user/logout
#[tokio::test]
async fn logout() {
	let test = setup().await;
	let key = "logout";
	let user_token = create_user(&test, key).await;
	let _ = fetch_me(&test, &user_token).await;
	logout_user(&test, &user_token).await;
	fail_fetch_me(&test, &user_token).await;
	let user_token = login_user(&test, key).await;
	delete_self_user(&test, &user_token).await;
}

// POST /api/v1/team
// GET /api/v1/team/<team_id>
// DELETE /api/v1/team/<team_id>
#[tokio::test]
async fn test_create_team() {
	let test = setup().await;
	let random = create_user(&test, "create_team_2").await;
	let admin = create_user(&test, "create_team_admin").await;
	let team_uuid = create_team(&test, &admin, "test_create_team").await;
	let team = fetch_team(&test, &admin, &team_uuid).await;
	assert_eq!(team.name, "test_create_team");
	fail_fetch_team(&test, &random, &team_uuid).await;
	delete_team(&test, &random, &team_uuid, Status::BadRequest).await;
	delete_team(&test, &admin, &team_uuid, Status::Ok).await;
	delete_team(&test, &random, &team_uuid, Status::BadRequest).await;
	delete_team(&test, &admin, &team_uuid, Status::BadRequest).await;
	fail_fetch_team(&test, &admin, &team_uuid).await;
	fail_fetch_team(&test, &random, &team_uuid).await;
	delete_self_user(&test, &admin).await;
	delete_self_user(&test, &random).await;
}

// GET /api/v1/team/roles
#[tokio::test]
async fn test_user_team_roles() {
	let test = setup().await;
	let user = create_user(&test, "user_team_roles").await;
	let UserTeamRoles { team_roles } = fetch_user_team_roles(&test, &user).await;
	assert!(team_roles.is_empty());

	let team01 = create_team(&test, &user, "user_team_roles_team01").await;
	let team02 = create_team(&test, &user, "user_team_roles_team02").await;
	let team03 = create_team(&test, &user, "user_team_roles_team03").await;

	// TODO add as team member

	let UserTeamRoles { team_roles } = fetch_user_team_roles(&test, &user).await;
	assert_eq!(team_roles.len(), 3);

	let check_team = |tr: &TeamRole, uuid: &str| {
		match &tr.team {
			Some(team) => team.uuid == uuid,
			_ => panic!("team is not Some: {:#?}", tr),
		}
	};
	let check_team_admin = |tr: &TeamRole, uuid: &str| {
		tr.role == "admin" && check_team(tr, uuid)
	};
	assert!(team_roles.iter().any(|tr| check_team_admin(tr, &team01.to_string())));
	assert!(team_roles.iter().any(|tr| check_team_admin(tr, &team02.to_string())));
	assert!(team_roles.iter().any(|tr| check_team_admin(tr, &team03.to_string())));

	delete_self_user(&test, &user).await;
}
