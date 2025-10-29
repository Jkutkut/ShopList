use tonic::transport::Server;
use tokio_postgres::{
	NoTls,
	Connection, Client,
	Socket,
	tls::NoTlsStream,
};
use std::io::{
	Error,
	ErrorKind,
};
use log::*;

use model::grpc::auth::auth_service_server::AuthServiceServer;
use grpc::Auth;
use model::jwt::JWTHandler;
use db::ShoplistDbAuth;

mod db;
mod grpc;
#[cfg(test)]
mod tests;

fn env_var_or_error(var: &str, error_msg: &str) -> Result<String, Error> {
	match std::env::var(var) {
		Ok(var) => Ok(var),
		Err(_) => Err(Error::new(ErrorKind::Other, error_msg)),
	}
}

struct Main {
	server: AuthServiceServer<Auth>,
	db_connection: Connection<Socket, NoTlsStream>
}

async fn connect_to_db() -> Result<(Client, Connection<Socket, NoTlsStream>), Error> {
	let db_properties = format!(
		"host={} port={} dbname={} user={} password={}",
		env_var_or_error("DB_HOST", "DB_HOST not defined as environment variable or in .env file")?,
		env_var_or_error("DB_PORT", "DB_PORT not defined as environment variable or in .env file")?,
		env_var_or_error("DB_NAME", "DB_NAME not defined as environment variable or in .env file")?,
		env_var_or_error("DB_USER", "DB_USER not defined as environment variable or in .env file")?,
		env_var_or_error("DB_USER_PASSWORD", "DB_USER_PASSWORD not defined as environment variable or in .env file")?
	);
	info!("Connecting to db... Config: {}", &db_properties);
	tokio_postgres::connect(&db_properties, NoTls).await
		.map_err(|_| Error::new(
			ErrorKind::Other,
			"Not able to connect with DB. Is the DB running?"
		))
}

async fn db_handler() -> Result<(ShoplistDbAuth, Connection<Socket, NoTlsStream>), Error> {
	let jwt_secret = env_var_or_error("AUTH_JWT_SECRET", "AUTH_JWT_SECRET not defined as environment variable or in .env file")?;
	let (db_client, db_connection) = connect_to_db().await?;
	let client = ShoplistDbAuth::new(db_client, JWTHandler::new(&jwt_secret));
	Ok((client, db_connection))
}

async fn main_builder() -> Result<Main, Error> {
	let env_path = std::env::var("ENV_PATH");
	if let Ok(env_path) = env_path {
		dotenv::from_path(env_path).ok();
	}

	let (client, db_connection) = db_handler().await?;
	let server = AuthServiceServer::new(Auth::new(client));
	Ok(Main { server, db_connection })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::init();
	let addr = "0.0.0.0:50051".parse().unwrap();
	let main = main_builder().await?;
	info!("Auth server listening on {addr}");
	Server::builder()
		.add_service(main.server)
		.serve_with_shutdown(addr, async {
			tokio::select! {
				_ = tokio::signal::ctrl_c() => {
					warn!("Received signal to end execution");
				},
				r = main.db_connection => {
					warn!("DB connection closed");
					if let Err(e) = r {
						error!("connection error: {}", e);
					}
				},
			};
			info!("Shutting down...");
		}).await?;
	Ok(())
}
