use tokio_postgres::{
	NoTls,
};
use log::*;
use crate::utils::env_var;

pub use tokio_postgres::{
	Client,
};

pub async fn connect_to_db_or_end() -> Result<Client, String> {
	let db_properties = format!(
		"host={} port={} dbname={} user={} password={}",
		env_var("DB_HOST")?,
		env_var("DB_PORT")?,
		env_var("DB_NAME")?,
		env_var("DB_USER")?,
		env_var("DB_USER_PASSWORD")?
	);
	info!("Connecting to db... Config: {}", &db_properties);
	let (db_client, db_connection) = match tokio_postgres::connect(&db_properties, NoTls).await {
		Ok(r) => r,
		Err(e) => {
			debug!("Failed to connect to DB: {}", e);
			return Err("Is the DB running?".into());
		}
	};
	tokio::spawn(async move {
		if let Err(e) = db_connection.await {
			error!("connection error: {}", e);
			error!("Stopping the server...");
			std::process::exit(1);
		}
	});
	Ok(db_client)
}
