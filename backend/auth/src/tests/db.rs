use super::*;

struct Test {
	pub db: ShoplistDbAuth,
}

async fn setup() -> Test {
	println!("Loading env vars from .env file");
	dotenv::from_path(std::env::var("ENV_PATH")
		.expect("ENV_PATH not defined as environment variable or in .env file")
	).unwrap();
	println!("Connecting to db...");
	let (client, db_connection) = db_handler().await.unwrap();
	tokio::spawn(async move {
		if let Err(e) = db_connection.await {
			eprintln!("connection error: {}", e);
			eprintln!("Stopping the server...");
			std::process::exit(1);
		}
	});
	println!("Setup complete");
	Test {
		db: client,
	}
}

#[tokio::test]
#[ntest::timeout(2000)]
async fn db_test() {
	let test = setup().await;

	let user = "marvin_db_test";
	let password = "marvin-password";
	let email = "marvin-db_test@marvin.com";

	let query = "DELETE FROM users where name = $1";
	let stmt = test.db.db_client.prepare(query).await.unwrap();
	test.db.db_client.execute(&stmt, &[&user]).await.unwrap();

	println!("invalid login attempt...");
	assert!(test.db.basic_login(
		email.into(), password.into()
	).await.is_err(), "Login should fail (No user)");

	println!("Registering user...");
	let r = test.db.register_user_basic_login(
		user.into(), email.into(), password.into()
	).await;
	assert!(r.is_ok(), "Register should succeed");
	let token = r.unwrap();
	assert!(!token.is_empty(), "Token should not be empty");

	println!("Register again should fail...");
	assert!(test.db.register_user_basic_login(
		user.into(), email.into(), password.into()
	).await.is_err(), "Register should fail (User already exists)");

	println!("Register with same email should fail...");
	assert!(test.db.register_user_basic_login(
		user.to_string() + "2", email.into(), password.into()
	).await.is_err(), "Register should fail (User already exists 2)");

	println!("Register with same name should fail...");
	assert!(test.db.register_user_basic_login(
		user.into(), "other-email".to_string() + email, password.into()
	).await.is_err(), "Register should fail (User already exists 3)");

	println!("Logging in again...");
	let r = test.db.basic_login(
		email.into(), password.into()
	).await;
	assert!(r.is_ok(), "Login should succeed");
	let token2 = r.unwrap();
	assert!(!token2.is_empty(), "Token should not be empty");
	assert!(token != token2, "Tokens should be different");

	let query = "WITH user_to_delete AS (
		SELECT user_id FROM credentials WHERE token = $1 LIMIT 1
	) DELETE FROM users WHERE id = (SELECT user_id FROM user_to_delete)";
	let stmt = test.db.db_client.prepare(query).await.unwrap();
	assert!(test.db.db_client.execute(&stmt, &[&token]).await.is_ok(), "Delete user should succeed");

	println!("Logging in again after deletion...");
	assert!(test.db.basic_login(
		email.into(), password.into()
	).await.is_err(), "Login should fail (User deleted)");
}
