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
use ::model::{
	grpc::auth::{
		User,
		UserToken,
		UserTeamRoles,
		TeamRole,
	},
};
use crate::{
	guards::Team,
	model::{
		UserRole,
		Product,
	},
};
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

async fn delete_self_user(_: &Test, user_token: &UserToken) {
	let UserToken { user_id, token, .. } = user_token;
	let endpoint = format!("/api/v1/user/{user_id}");
	let client = new_client().await;
	let res = client.delete(&endpoint)
		.header(auth_header(&token))
		.dispatch().await;
	assert_eq!(res.status(), Status::Ok);
}

async fn fetch_me(_: &Test, user_token: &UserToken) -> User {
	let UserToken { token, .. } = user_token;
	let client = new_client().await;
	let res = client.get("/api/v1/user/me")
		.header(auth_header(&token))
		.dispatch().await;
	check_json_response(&res).await;
	res.into_json().await.unwrap()
}

async fn fail_fetch_me(_: &Test, user_token: &UserToken) {
	let UserToken { token, .. } = user_token;
	let client = new_client().await;
	let res = client.get("/api/v1/user/me")
		.header(auth_header(&token))
		.dispatch().await;
	check_response(&res, Status::Unauthorized, "application/json").await;
}

#[allow(dead_code)]
async fn fetch_user(_: &Test, user_token: &UserToken, user_id: &str) -> User {
	let UserToken { token, .. } = user_token;
	let endpoint = format!("/api/v1/user/{}", user_id);
	let client = new_client().await;
	let res = client.get(&endpoint)
		.header(auth_header(&token))
		.dispatch().await;
	check_json_response(&res).await;
	res.into_json().await.unwrap()
}

async fn login_user(_: &Test, key: &str) -> UserToken {
	sleep(Duration::from_secs(1)); // Ensure the JWT token is different
	let credentials = create_user_credentials(key);
	let client = new_client().await;
	let req = client.post("/api/v1/user/login/basic").json(&credentials);
	let res = req.dispatch().await;
	check_json_response(&res).await;
	res.into_json().await.unwrap()
}

async fn logout_user(_: &Test, user_token: &UserToken) {
	let UserToken { token, .. } = user_token;
	let client = new_client().await;
	let res = client.post("/api/v1/user/logout")
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
	let client = new_client().await;
	let req = client.post("/api/v1/team")
		.header(auth_header(&token))
		.json(&json!({
			"name": team_name,
			"display_name": team_name
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

async fn fetch_team_members(_: &Test, team_id: &Uuid, user_token: &UserToken) -> Vec<UserRole> {
	let UserToken { token, .. } = user_token;
	let endpoint = format!("/api/v1/team/{team_id}/members");
	let client = new_client().await;
	let res = client.get(&endpoint)
		.header(auth_header(&token))
		.dispatch().await;
	check_json_response(&res).await;
	res.into_json().await.unwrap()
}

async fn add_user_to_team(_: &Test, user: &UserToken, team: &Uuid, user_to_add: &UserToken, role: &str, should_succeed: bool) {
	info!("Adding user \"{}\" to team \"{}\" as {} by user \"{}\"", user_to_add.user_id, team, role, user.user_id);
	let endpoint = format!("/api/v1/team/{}/members", team);
	let client = new_client().await;
	let response = client.put(&endpoint)
		.header(auth_header(&user.token))
		.json(&json!({
			"role": role,
			"user_id": user_to_add.user_id
		}))
		.dispatch().await;
	if should_succeed {
		check_json_response(&response).await;
	}
	else {
		check_response(&response, Status::BadRequest, "application/json").await;
	}
}

async fn try_delete_user_from_team(
	_: &Test,
	user: &UserToken,
	team: &Uuid,
	other_user: &str,
	should_succeed: bool
) {
	info!("Deleting user \"{}\" from team \"{}\" by user \"{}\"", other_user, team, user.user_id);
	let UserToken { token, .. } = user;
	let endpoint = format!("/api/v1/team/{}/members/{}", team, other_user);
	let client = new_client().await;
	let res = client.delete(&endpoint)
		.header(auth_header(&token))
		.dispatch().await;
	if should_succeed {
		check_json_response(&res).await;
	} else {
		check_json_content_type(&res);
	}
}

async fn create_product(test: &Test, user: &UserToken, team_id: &Uuid, product_name: &str) -> Product {
	info!("Creating product");
	debug!("Creating product {} for team {} by user {}", product_name, team_id, &user.user_id);
	{
		let db = test.client.rocket().state::<db::DB>().unwrap().client();
		let query = "DELETE FROM products WHERE name = $1";
		let stmt = db.prepare(query).await.unwrap();
		db.execute(&stmt, &[&product_name]).await.unwrap();
	}
	let UserToken { token, .. } = user;
	let client = new_client().await;
	let endpoint = format!("/api/v1/team/{}/product", team_id);
	let req = client.post(&endpoint)
		.header(auth_header(&token))
		.json(&json!({
			"name": product_name,
			"description": product_name
		}));
	let res = req.dispatch().await;
	check_json_response(&res).await;
	let product: Product = res.into_json().await.unwrap();
	assert_eq!(product.name, product_name);
	assert_eq!(product.description, Some(product_name.into()));
	product
}

async fn fetch_products(_: &Test, user: &UserToken, team_id: &Uuid) -> Vec<Product> {
	info!("Fetching products for team {}", team_id);
	let UserToken { token, .. } = user;
	let client = new_client().await;
	let endpoint = format!("/api/v1/team/{}/products", team_id);
	let res = client.get(&endpoint)
		.header(auth_header(&token))
		.dispatch().await;
	check_json_response(&res).await;
	res.into_json().await.unwrap()
}

async fn update_product(_: &Test, user: &UserToken, team: &Uuid, product_id: &Uuid, name: &str, description: Option<&str>) {
	let UserToken { token, .. } = user;
	let client = new_client().await;
	let endpoint = format!("/api/v1/team/{}/product/{}", team, product_id);
	let req = client.put(&endpoint)
		.header(auth_header(&token))
		.json(&json!({
			"name": name,
			"description": description
		}));
	let res = req.dispatch().await;
	check_json_response(&res).await;
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
	// assert_eq!(user, fetch_user(&test, &user_token, &user.id).await);
	assert_eq!(user.id, user_token.user_id);
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
	let team_id = create_team(&test, &admin, "test_create_team").await;
	let team = fetch_team(&test, &admin, &team_id).await;
	assert_eq!(team.name, "test_create_team");
	fail_fetch_team(&test, &random, &team_id).await;
	delete_team(&test, &random, &team_id, Status::BadRequest).await;
	delete_team(&test, &admin, &team_id, Status::Ok).await;
	delete_team(&test, &random, &team_id, Status::BadRequest).await;
	delete_team(&test, &admin, &team_id, Status::BadRequest).await;
	fail_fetch_team(&test, &admin, &team_id).await;
	fail_fetch_team(&test, &random, &team_id).await;
	delete_self_user(&test, &admin).await;
	delete_self_user(&test, &random).await;
}

// GET /api/v1/team/roles
// PUT /api/v1/team/<team_id>/members
// GET /api/v1/team/<team_id>/members
// DELETE /api/v1/team/<team_id>/members
#[tokio::test]
async fn test_user_team_roles() {
	let test = setup().await;
	let user = create_user(&test, "user_team_roles").await;
	let other_team_user = create_user(&test, "user_team_roles_other").await;
	let other_team_user_2 = create_user(&test, "user_team_roles_other_2").await;
	let random_user = create_user(&test, "user_team_roles_random").await;
	let random_user_2 = create_user(&test, "user_team_roles_random_2").await;
	let UserTeamRoles { team_roles } = fetch_user_team_roles(&test, &user).await;
	assert!(team_roles.is_empty());

	let team01 = create_team(&test, &user, "user_team_roles_team01").await;
	let team02 = create_team(&test, &user, "user_team_roles_team02").await;
	let other_team = create_team(&test, &other_team_user, "user_team_roles_other_team").await;

	add_user_to_team(&test, &user, &team01, &other_team_user, "member", true).await;
	add_user_to_team(&test, &user, &team01, &other_team_user_2, "member", true).await;
	add_user_to_team(&test, &other_team_user, &other_team, &user, "admin", true).await;

	add_user_to_team(&test, &random_user, &team01, &user, "member", false).await;
	add_user_to_team(&test, &random_user, &team01, &random_user_2, "member", false).await;
	add_user_to_team(&test, &other_team_user, &team01, &random_user, "member", false).await;
	add_user_to_team(&test, &other_team_user, &team01, &random_user, "admin", false).await;

	let check_team = |tr: &TeamRole, id: &str| {
		match &tr.team {
			Some(team) => team.id == id,
			_ => panic!("team is not Some: {:#?}", tr),
		}
	};
	let check_team_admin = |tr: &TeamRole, id: &str| {
		tr.role == "admin" && check_team(tr, id	)
	};
	let check_team_member = |tr: &TeamRole, id: &str| {
		tr.role == "member" && check_team(tr, id)
	};
	let UserTeamRoles { team_roles } = fetch_user_team_roles(&test, &user).await;
	assert_eq!(team_roles.len(), 3);
	assert!(team_roles.iter().any(|tr| check_team_admin(tr, &team01.to_string())));
	assert!(team_roles.iter().any(|tr| check_team_admin(tr, &team02.to_string())));
	assert!(team_roles.iter().any(|tr| check_team_admin(tr, &other_team.to_string())));
	let UserTeamRoles { team_roles } = fetch_user_team_roles(&test, &other_team_user).await;
	assert_eq!(team_roles.len(), 2);
	assert!(team_roles.iter().any(|tr| check_team_admin(tr, &other_team.to_string())));
	assert!(team_roles.iter().any(|tr| check_team_member(tr, &team01.to_string())));

	// Modify roles
	add_user_to_team(&test, &user, &team01, &other_team_user, "admin", true).await;
	let UserTeamRoles { team_roles } = fetch_user_team_roles(&test, &other_team_user).await;
	assert_eq!(team_roles.len(), 2);
	assert!(team_roles.iter().any(|tr| check_team_admin(tr, &other_team.to_string())));
	assert!(team_roles.iter().any(|tr| check_team_admin(tr, &team01.to_string())));

	// Fetch team members
	let team_members = fetch_team_members(&test, &team01, &user).await;
	assert_eq!(team_members.len(), 3);
	assert!(team_members.iter().any(|ur| ur.user.id.to_string() == user.user_id));
	assert!(team_members.iter().any(|ur| ur.user.id.to_string() == other_team_user.user_id));
	assert!(team_members.iter().any(|ur| ur.user.id.to_string() == other_team_user_2.user_id));
	let team_members = fetch_team_members(&test, &team01, &other_team_user).await;
	assert_eq!(team_members.len(), 3);
	assert!(team_members.iter().any(|ur| ur.user.id.to_string() == user.user_id));
	assert!(team_members.iter().any(|ur| ur.user.id.to_string() == other_team_user.user_id));
	assert!(team_members.iter().any(|ur| ur.user.id.to_string() == other_team_user_2.user_id));

	// Delete team members
	// random vs admin
	try_delete_user_from_team(&test, &random_user_2, &team01, &user.user_id, false).await;
	// admin vs member
	try_delete_user_from_team(&test, &user, &team01, &other_team_user.user_id, true).await;
	// admin vs deleted member
	try_delete_user_from_team(&test, &user, &team01, &other_team_user.user_id, false).await;
	// member vs admin
	try_delete_user_from_team(&test, &other_team_user, &team01, &user.user_id, false).await;
	// member vs member
	try_delete_user_from_team(&test, &other_team_user, &team01, &other_team_user_2.user_id, false).await;

	delete_self_user(&test, &user).await;
	delete_self_user(&test, &other_team_user).await;
	delete_self_user(&test, &random_user).await;
	delete_self_user(&test, &random_user_2).await;
}

// GET /api/v1/team/<team_id>/products
// POST /api/v1/team/<team_id>/product
// PUT /api/v1/team/<team_id>/product/<product_id>
#[tokio::test]
async fn test_products() {
	let test = setup().await;
	let user = create_user(&test, "test_products").await;
	let team = create_team(&test, &user, "team_test_products").await;

	let p01 = create_product(&test, &user, &team, "product_1").await;
	let p02 = create_product(&test, &user, &team, "product_2").await;
	let p03 = create_product(&test, &user, &team, "product_3").await;

	let products = fetch_products(&test, &user, &team).await;
	assert_eq!(products.len(), 3);
	assert!(products.iter().any(|p| p.id == p01.id));
	assert!(products.iter().any(|p| p.id == p02.id));
	assert!(products.iter().any(|p| p.id == p03.id));

	update_product(&test, &user, &team, &p01.id, "product_1_updated", None).await;
	update_product(&test, &user, &team, &p02.id, "product_2_updated", Some("product_2_description_updated")).await;
	let products = fetch_products(&test, &user, &team).await;
	assert_eq!(products.len(), 3);
	assert!(products.iter().any(|p| p.id == p01.id && p.name == "product_1_updated"));
	assert!(products.iter().any(|p| p.id == p02.id && p.name == "product_2_updated" && p.description == Some("product_2_description_updated".into())));

	delete_team(&test, &user, &team, Status::Ok).await;
	delete_self_user(&test, &user).await;
}
