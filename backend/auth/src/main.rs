use tonic::transport::Server;
use tokio_postgres::NoTls;

mod db;
mod grpc;

use model::grpc::auth::auth_service_server::AuthServiceServer;
use grpc::Auth;
use model::jwt::JWTHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	dotenv::from_path(
		std::env::var("ENV_PATH").unwrap_or("../.env".to_string())
	).ok();

	let jwt_secret = std::env::var("JWT_SECRET")
		.expect("JWT_SECRET not defined as environment variable or in .env file");

	let db_properties = format!(
		"host={} port={} dbname={} user={} password={}",
		"shoplist-db", "5432",
		std::env::var("DB_NAME").expect("DB_NAME not defined as environment variable or in .env file"),
		std::env::var("DB_USER").expect("DB_USER not defined as environment variable or in .env file"),
		std::env::var("DB_USER_PASSWORD").expect("DB_USER_PASSWORD not defined as environment variable or in .env file")
	);

	let (client, connection) = match tokio_postgres::connect(&db_properties, NoTls).await {
		Ok((client, connection)) => (client, connection),
		Err(e) => {
			eprintln!("Not able to connect with DB:\n{}", e);
			eprintln!("Is the DB running?");
			std::process::exit(1);
		}
	};

	let client = db::ShoplistDbAuth::new(client, JWTHandler::new(&jwt_secret));
	let auth_server = AuthServiceServer::new(Auth::new(client));
	let addr = "0.0.0.0:50051".parse().unwrap();
	println!("Auth server listening on {addr}");

	Server::builder()
		.add_service(auth_server)
		.serve_with_shutdown(addr, async {
			tokio::select! {
				_ = tokio::signal::ctrl_c() => {
					eprintln!("Received signal to end execution");
				},
				r = connection => {
					eprintln!("DB connection closed");
					if let Err(e) = r {
						eprintln!("connection error: {}", e);
					}
				},
			};
			println!("Shutting down...");
		})
		.await?;

	Ok(())
}
