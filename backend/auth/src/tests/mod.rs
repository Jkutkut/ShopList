use crate::*;
use model::grpc::auth::*;
use uuid::Uuid;
#[allow(unused_imports)]
use macro_test::*;

mod db;

// The tests require a valid .env file configured
// and the ENV_PATH defined as an environment variable

#[derive(Debug)]
struct TestEnvVar<'a> {
	name: &'a str,
}

impl TestEnvVar<'_> {
	fn error_msg_test(&self) -> &str {
		self.name
	}
}

static ENV: &[TestEnvVar] = &[
	TestEnvVar { name: "DB_HOST" },
	TestEnvVar { name: "DB_PORT" },
	TestEnvVar { name: "AUTH_JWT_SECRET" },
	TestEnvVar { name: "DB_NAME" },
	TestEnvVar { name: "DB_USER" },
	TestEnvVar { name: "DB_USER_PASSWORD" },
];

async fn test_env_var(env_var: &TestEnvVar<'_>) {
	info!("Testing missing env var {}:\n{:?}", env_var.name, env_var);
	let value = std::env::var(env_var.name);
	info!("Variable exists in current context: {}", value.is_ok());
	assert!(value.is_ok());
	// Remove env var
	let value = value.unwrap();
	unsafe {
		std::env::remove_var(env_var.name);
	}
	// Test missing env var
	match main_builder().await {
		Err(e) => {
			let error = e.to_string();
			let error_piece = env_var.error_msg_test();
			info!("- Error generated: {}\n- Error piece: {}\n", &error, &error_piece);
			assert!(error.contains(error_piece));
		},
		err => assert!(err.is_err()),
	}
	// Restore env var
	unsafe {
		std::env::set_var(env_var.name, value);
	}
}

#[tokio::test]
async fn test_main() {
	tokio::spawn(async {
		dotenv::from_path(
			std::env::var("ENV_PATH").unwrap()
		).ok();
		unsafe {
			std::env::remove_var("ENV_PATH");
		}
		for env_var in ENV {
			test_env_var(env_var).await;
		}
		assert!(main_builder().await.is_ok());
	}).await.unwrap();
}

#[tokio::test]
async fn env_variable_test() {
	tokio::spawn(async {
		let env_path = std::env::var("ENV_PATH").unwrap();
		info!("ENV_PATH: {}\n", &env_path);
		#[allow(deprecated)]
		let env = dotenv::from_path_iter(&env_path).unwrap().map(|k| {
			let (k, v) = k.unwrap();
			info!("{}={}", k, v);
			k
		})
		.filter(|env_var|
			env_var.starts_with("AUTH_") ||
			(env_var.starts_with("DB_") && !env_var.starts_with("DB_CONTROLER_"))
		)
		.collect::<Vec<String>>();
		let not_in_env = ENV.iter()
			.filter(|env_var| env_var.name != "DB_HOST" && env_var.name != "DB_PORT")
			.filter(|env_var| !env.contains(&env_var.name.to_string()))
			.collect::<Vec<_>>();
		let not_in_tests = env.iter().filter(|env_var| {
			for test_env_var in ENV {
				if test_env_var.name == env_var.as_str() {
					return false;
				}
			}
			true
		}).collect::<Vec<_>>();
		if not_in_env.is_empty() && not_in_tests.is_empty() {
			info!("All env vars found");
			return;
		}
		info!("Env variables do not align between tests and current environment:");
		info!("Not in ENV: {:?}", not_in_env);
		info!("Not in tests: {:?}\n", not_in_tests);
		assert!(false);
	}).await.unwrap();
}
