use super::*;

struct Test {
	pub db: ShoplistDbAuth,
}

async fn setup() -> Test {
	let _ = env_logger::Builder::from_env(
		env_logger::Env::default()
			.default_filter_or("auth=debug")
	).is_test(true).format_timestamp(None).try_init();
	info!("Loading env vars from .env file");
	dotenv::from_path(std::env::var("ENV_PATH")
		.expect("ENV_PATH not defined as environment variable or in .env file")
	).unwrap();
	info!("Connecting to db...");
	let (client, db_connection) = db_handler().await.unwrap();
	tokio::spawn(async move {
		if let Err(e) = db_connection.await {
			error!("connection error: {}", e);
			error!("Stopping the server...");
			std::process::exit(1);
		}
	});
	info!("Setup complete");
	Test {
		db: client,
	}
}

async fn ensure_users_deleted(test: &Test, names: &[&str]) {
	info!("ensure_users_deleted({:?})", names);
	let query = "DELETE FROM users where name = $1";
	let stmt = test.db.db_client.prepare(query).await.unwrap();
	for user in names {
		test.db.db_client.execute(&stmt, &[user]).await.unwrap();
	}
}

async fn create_user_basic_credentials(test: &Test, name: &str, email: &str, password: &str) -> UserToken {
	let r = test.db.register_user_basic_login(
		name.into(), email.into(), password.into()
	).await;
	assert!(r.is_ok(), "Register should succeed");
	let token = r.unwrap();
	assert!(!token.token.is_empty(), "Token should not be empty");
	assert!(!token.user_id.is_empty(), "User ID should not be empty");
	assert!(!token.expires_at.is_empty(), "Expires at should not be empty");
	token
}

#[allow(dead_code)]
async fn create_superuser_basic_credentials(test: &Test, name: &str, email: &str, password: &str) -> UserToken {
	let user_token = create_user_basic_credentials(&test, name, email, password).await;
	let super_user_id: Uuid = user_token.user_id.parse().unwrap();
	let query = "SELECT set_superuser($1);";
	let stmt = test.db.db_client.prepare(query).await.unwrap();
	test.db.db_client.execute(&stmt, &[&super_user_id]).await.unwrap();
	user_token
}

#[tokio::test]
#[ntest::timeout(4000)]
async fn db_test() {
	info!("Running test db_test...");
	let test = setup().await;

	let user = "marvin_db_test";
	let password = "marvin-password";
	let email = "marvin-db_test@marvin.com";

	ensure_users_deleted(&test, &[user]).await;

	info!("invalid login attempt...");
	assert!(test.db.basic_login(
		email.into(), password.into()
	).await.is_err(), "Login should fail (No user)");

	info!("Registering user...");
	let token = create_user_basic_credentials(&test, user, email, password).await.token;

	info!("Obtaining user form token...");
	let me = test.db.me(&token).await;
	assert!(me.is_ok(), "Me should succeed");
	let me = me.unwrap();
	info!("Me: {:?}", me);
	assert!(me.name == user, "User should be {}", user);
	assert!(!me.is_superuser, "User should not be superuser");

	info!("Register again should fail...");
	assert!(test.db.register_user_basic_login(
		user.into(), email.into(), password.into()
	).await.is_err(), "Register should fail (User already exists)");

	info!("Register with same email should fail...");
	assert!(test.db.register_user_basic_login(
		user.to_string() + "2", email.into(), password.into()
	).await.is_err(), "Register should fail (User already exists 2)");

	info!("Register with same name should fail...");
	assert!(test.db.register_user_basic_login(
		user.into(), "other-email".to_string() + email, password.into()
	).await.is_err(), "Register should fail (User already exists 3)");

	info!("Logging in again...");
	let r = test.db.basic_login(
		email.into(), password.into()
	).await;
	assert!(r.is_ok(), "Login should succeed");
	let token2 = r.unwrap().token;
	assert!(!token2.is_empty(), "Token should not be empty");
	assert!(token != token2, "Tokens should be different");

	let query = "WITH user_to_delete AS (
		SELECT user_id FROM credentials WHERE token = $1 LIMIT 1
	) DELETE FROM users WHERE id = (SELECT user_id FROM user_to_delete)";
	let stmt = test.db.db_client.prepare(query).await.unwrap();
	assert!(test.db.db_client.execute(&stmt, &[&token]).await.is_ok(), "Delete user should succeed");

	info!("Logging in again after deletion...");
	assert!(test.db.basic_login(
		email.into(), password.into()
	).await.is_err(), "Login should fail (User deleted)");

	info!("Me should not work anymore...");
	assert!(test.db.me(&token).await.is_err(), "Me should fail (User deleted)");
	assert!(test.db.me(&token2).await.is_err(), "Me should fail (User deleted)");
}

#[tokio::test]
#[ntest::timeout(4000)]
async fn db_test_logout() {
	let test = setup().await;

	let user = "marvin_db_test_logout";
	let password = "marvin-password";
	let email = "marvin-db_test_logout@marvin.com";

	ensure_users_deleted(&test, &[user]).await;

	info!("Registering users...");
	let token = create_user_basic_credentials(&test, user, email, password).await.token;

	// Logout
	assert!(test.db.me(&token).await.is_ok(), "Me should succeed");
	test.db.logout(&token).await.unwrap();
	assert!(test.db.me(&token).await.is_err(), "Me should fail (User logged out)");
}

#[tokio::test]
#[ntest::timeout(5000)]
async fn db_test_change_password() {
	let test = setup().await;

	let password = "marvin-password";
	let user_base = "db_test_change_password_";
	let email_base = "marvin-db_test_change_password_";
	let new_password = "new-password";

	ensure_users_deleted(&test, &[
		&(user_base.to_string() + "1"),
		&(user_base.to_string() + "2"),
		&(user_base.to_string() + "superuser"),
	]).await;

	info!("Registering users...");
	let super_user_token = create_superuser_basic_credentials(
		&test,
		&(user_base.to_string() + "superuser"),
		&(email_base.to_string() + "superuser@marvin.com"),
		"superuser-password"
	).await.token;
	let UserToken { token: user1_token, user_id, .. } = create_user_basic_credentials(
		&test,
		&(user_base.to_string() + "1"),
		&(email_base.to_string() + "1@marvin.com"),
		password
	).await;
	let user1_id: Uuid = user_id.parse().unwrap();
	let user2_token = create_user_basic_credentials(
		&test,
		&(user_base.to_string() + "2"),
		&(email_base.to_string() + "2@marvin.com"),
		password
	).await.token;

	assert!(test.db.basic_login(
		email_base.to_string() + "1@marvin.com", password.to_string()
	).await.is_ok(), "Login should succeed");
	assert!(test.db.basic_change_password(
		user2_token, &user1_id, new_password.to_string()
	).await.is_err(), "Other users should not be able to change password");
	assert!(test.db.basic_login(
		email_base.to_string() + "1@marvin.com", password.to_string()
	).await.is_ok(), "Login should still succeed");

	assert!(test.db.basic_change_password(
		user1_token, &user1_id, new_password.to_string()
	).await.is_ok(), "Each user can change their own password");
	assert!(test.db.basic_login(
		email_base.to_string() + "1@marvin.com", password.to_string()
	).await.is_err(), "Login should fail (Password changed)");
	assert!(test.db.basic_login(
		email_base.to_string() + "1@marvin.com", new_password.to_string()
	).await.is_ok(), "Login should succeed (Password changed)");

	assert!(test.db.basic_change_password(
		super_user_token, &user1_id, new_password.to_string() + "superuser"
	).await.is_ok(), "Superuser can change password");
	assert!(test.db.basic_login(
		email_base.to_string() + "1@marvin.com", new_password.to_string()
	).await.is_err(), "Login should fail (Password changed by superuser)");
	assert!(test.db.basic_login(
		email_base.to_string() + "1@marvin.com", new_password.to_string() + "superuser"
	).await.is_ok(), "Login should succeed (Password changed by superuser)");
}

#[tokio::test]
#[ntest::timeout(2000)]
async fn db_test_logout_user() {
	let test = setup().await;
	
	let user = "marvin_db_test_logout_user";
	let password = "marvin-password";
	let email = "marvin-db_test_logout_user@marvin.com";

	let super_user = "marvin_db_test_logout_user_superuser";
	let super_password = "marvin-password";
	let super_email = "marvin-db_test_logout_user_superuser@marvin.com";

	ensure_users_deleted(&test, &[user, super_user]).await;

	info!("Registering users...");
	let UserToken {
		token: super_token,
		user_id: super_user_id,
		..
	} = create_superuser_basic_credentials(&test, super_user, super_email, super_password).await;
	let super_id: Uuid = super_user_id.parse().unwrap();
	let UserToken { token, user_id, .. } = create_user_basic_credentials(&test, user, email, password).await;
	let user_id: Uuid = user_id.parse().unwrap();
	assert!(test.db.me(&token).await.is_ok(), "User info should succeed");

	assert!(test.db.logout_user(
		&token,
		&user_id
	).await.is_ok(), "User should be able to logout itself");
	assert!(test.db.me(&token).await.is_err(), "Me should fail (User logged out)");
	assert!(test.db.logout_user(
		&token,
		&user_id
	).await.is_err(), "User should not be able to logout itself again with invalid token");

	let token = test.db.basic_login(
		email.to_string(), password.to_string()
	).await.unwrap().token;

	assert!(test.db.logout_user(
		&super_token,
		&user_id
	).await.is_ok(), "Superuser should be able to logout user");
	assert!(test.db.me(&token).await.is_err(), "Me should fail (User logged out)");
	assert!(test.db.me(&super_token).await.is_ok(), "Me should succeed (Superuser logged in)");
	assert!(test.db.logout_user(
		&super_token,
		&super_id
	).await.is_ok(), "Superuser should be able to logout itself");
	assert!(test.db.me(&super_token).await.is_err(), "Me should fail (Superuser logged out)");

	ensure_users_deleted(&test, &[user, super_user]).await;
}

#[tokio::test]
#[ntest::timeout(5000)]
async fn db_test_logout_everyone() {
	let test = setup().await;

	let mut users = Vec::new();
	for i in 0..10 {
		let user = format!("marvin_db_test_logout_everyone_{}", i);
		ensure_users_deleted(&test, &[&user]).await;
		let password = "marvin-password";
		let email = format!("marvin-db_test_logout_everyone_{}@marvin.com", i);

		let user = create_user_basic_credentials(&test, &user, &email, password).await.token;
		users.push(user);
	}

	let super_user = "marvin_db_test_logout_everyone_superuser";
	ensure_users_deleted(&test, &[super_user]).await;
	let super_password = "marvin-password";
	let super_email = "marvin-db_test_logout_everyone_superuser@marvin.com";

	let super_token = create_superuser_basic_credentials(&test, super_user, super_email, super_password).await.token;

	for i in 0..10 {
		assert!(test.db.me(&users[i]).await.is_ok(), "Me should succeed");
	}
	assert!(test.db.logout_everyone(&users[0]).await.is_err(), "Normal users should not be able to logout everyone");
	for i in 0..10 {
		assert!(test.db.me(&users[i]).await.is_ok(), "Me should succeed");
	}
	assert!(test.db.logout_everyone(&super_token).await.is_ok(), "Superuser should be able to logout everyone");
	for i in 0..10 {
		assert!(test.db.me(&users[i]).await.is_err(), "Me should fail (User logged out)");
	}
	assert!(test.db.me(&super_token).await.is_err(), "Me should fail (Superuser logged out)");

	users.push(super_token);
	let users = users.iter().map(|x| x.as_ref()).collect::<Vec<&str>>();

	ensure_users_deleted(&test, &users).await;
}

// TODO refresh token testing

#[tokio::test]
#[ntest::timeout(4000)]
async fn db_test_user_team_roles() {
	let test = setup().await;

	let user = "marvin_db_test_user_team_roles";
	let password = "marvin-password";
	let email = "marvin-db_test_user_team_roles@marvin.com";

	let other_user = "marvin_db_test_user_team_roles_other";
	let other_password = "marvin-password";
	let other_email = "marvin-db_test_user_team_roles_other@marvin.com";

	ensure_users_deleted(&test, &[user]).await;
	let user_id = create_user_basic_credentials(&test, user, email, password).await.user_id;
	let user_id: Uuid = user_id.parse().unwrap();
	ensure_users_deleted(&test, &[other_user]).await;
	let other_user_id = create_user_basic_credentials(&test, other_user, other_email, other_password).await.user_id;
	let other_user_id: Uuid = other_user_id.parse().unwrap();

	let user_team_roles = test.db.team_roles(&user_id).await;
	assert!(user_team_roles.is_ok(), "Team roles should succeed");
	let user_team_roles = user_team_roles.unwrap().team_roles;
	assert_eq!(user_team_roles.len(), 0);

	let teams = [
		"marvin_db_test_user_team_roles_team1",
		"marvin_db_test_user_team_roles_team2",
		"marvin_db_test_user_team_roles_team3",
	];

	let query_new_team = "SELECT new_team($1, $2, $3, $4, $5)";
	let stmt_new_team = test.db.db_client.prepare(query_new_team).await.unwrap();
	let delete_team = "DELETE FROM teams WHERE name = $1";
	let delete_team = test.db.db_client.prepare(delete_team).await.unwrap();
	let mut team_ids = Vec::new();
	for team in teams.iter() {
		assert!(test.db.db_client.execute(&delete_team, &[team]).await.is_ok(), "Delete team {} should succeed", team);
		let result = test.db.db_client.query_one(&stmt_new_team, &[
			&user_id,
			team,
			team,
			&format!("This is the description for the team with name {}", team),
			&"url://2-image.png"
		]).await;
		assert!(result.is_ok(), "Create team {} should succeed", team);
		let team_id: Uuid = result.unwrap().get(0);
		team_ids.push(team_id);
	}

	let user_team_roles = test.db.team_roles(&user_id).await;
	assert!(user_team_roles.is_ok(), "Team roles should succeed");
	let user_team_roles = user_team_roles.unwrap().team_roles;
	assert_eq!(user_team_roles.len(), 3);

	let join_team = "INSERT INTO user_roles (user_id, team_id, role) VALUES ($1, $2, $3)";
	let join_team = test.db.db_client.prepare(join_team).await.unwrap();
	for (idx, team) in teams.iter().enumerate() {
		let role = match *team {
			"marvin_db_test_user_team_roles_team2" => "admin",
			_ => "member",
		};
		let result = test.db.db_client.execute(&join_team, &[
			&other_user_id,
			&team_ids[idx],
			&role
		]).await;
		assert!(result.is_ok(), "Join team {} should succeed, but got: {:?}", team, result);
	}

	let user_team_roles = test.db.team_roles(&other_user_id).await;
	assert!(user_team_roles.is_ok(), "Team roles should succeed");
	let user_team_roles = user_team_roles.unwrap().team_roles;
	assert_eq!(user_team_roles.len(), 3);
	for team_role in user_team_roles.iter() {
		assert!(team_role.team.is_some(), "Team should be Some");
		let team = team_role.team.as_ref().unwrap();
		assert!(!team.name.is_empty(), "Team name should not be empty");
		let team_name = team.name.as_str();
		let expected_role = match team_name {
			"marvin_db_test_user_team_roles_team1" => "member",
			"marvin_db_test_user_team_roles_team2" => "admin",
			"marvin_db_test_user_team_roles_team3" => "member",
			_ => panic!("Unexpected team name: {}", team_name),
		};
		assert_eq!(team_role.role, expected_role, "Role should be {} in team {}", expected_role, team_name);
	}

	ensure_users_deleted(&test, &[user]).await;
	let user_team_roles = test.db.team_roles(&user_id).await;
	debug!("user_team_roles (deleted user): {:#?}", user_team_roles);
	assert!(user_team_roles.is_err(), "Team roles should fail (User deleted)");
}
