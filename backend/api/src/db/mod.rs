use tokio_postgres::{
	Client,
	NoTls,
};
use log::*;
use crate::utils::env_var;
use model::{
	TeamRequest,
};
use uuid::Uuid;

pub struct DB {
	client: Client,
}

impl DB {
	fn new(client: Client) -> Self {
		Self {
			client,
		}
	}

	#[cfg(test)]
	pub fn client(&self) -> &Client {
		&self.client
	}

	#[cfg(not(test))]
	fn client(&self) -> &Client {
		&self.client
	}

	pub async fn connect_to_db_or_end() -> Result<Self, String> {
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
		Ok(DB::new(db_client))
	}
}

// Teams
impl DB {
	pub async fn create_team(&self, creator_uuid: &Uuid, team: &TeamRequest) -> Result<Uuid, String> {
		info!("Creating team \"{}\" by user {}", team.name, creator_uuid);
		debug!("Team request: {:#?}", team);
		let query = "SELECT new_team($1, $2, $3, $4)";
		let stmt = self.client().prepare(query).await.unwrap();
		match self.client().query_one(&stmt, &[
			creator_uuid, &team.name, &team.description, &team.image
		]).await {
			Ok(r) => {
				debug!("Team created: {:#?}", r);
				Ok(r.get::<'_, usize, Uuid>(0))
			},
			Err(e) => {
				warn!("Error creating team: {}", e);
				Err(e.to_string()) // TODO
			}
		}
	}
}
