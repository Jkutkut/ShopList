use tonic::transport::Server;
use tokio_postgres::NoTls;

mod db;
mod grpc;

use model::grpc::auth::auth_service_server::AuthServiceServer;
use grpc::Auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	dotenv::from_path(
		std::env::var("ENV_PATH").unwrap_or("../.env".to_string())
	).ok();

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

	tokio::spawn(async move {
		if let Err(e) = connection.await {
			eprintln!("connection error: {}", e);
			eprintln!("Stopping the server...");
			std::process::exit(1);
		}
	});

	let client = db::ShoplistDbAuth::new(client);
	let auth_server = AuthServiceServer::new(Auth::new(client));
	let addr = "0.0.0.0:50051".parse().unwrap();
	println!("Auth server listening on {addr}");
	Server::builder()
		.add_service(auth_server)
		.serve(addr)
		.await?;

	Ok(())
}
