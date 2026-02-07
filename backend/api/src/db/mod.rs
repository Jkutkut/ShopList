use tokio_postgres::{
	Client,
	NoTls,
};
use log::*;
use model::{
	TeamRequest,
};
use crate::{
	utils::env_var,
	guards::{
		Team,
		User
	},
	model::{
		UserRole,
	},
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
	pub async fn create_team(&self, creator_id: &Uuid, team: &TeamRequest) -> Result<Uuid, String> {
		info!("Creating team \"{}\" by user {}", team.name, creator_id);
		debug!("Team request: {:#?}", team);
		let query = "SELECT new_team($1, $2, $3, $4)";
		let stmt = self.client().prepare(query).await.unwrap();
		match self.client().query_one(&stmt, &[
			creator_id, &team.name, &team.description, &team.image
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

	pub async fn get_team(&self, team_id: &Uuid, user_id: &Uuid) -> Result<Team, String> {
		info!("Getting team \"{}\" by user {}", team_id, user_id);
		let query = "SELECT
				t.id, t.name, t.description, t.image,
				t.created_at, t.updated_at,
				t.created_by, t.updated_by
			FROM teams t, user_roles ur WHERE
				t.id = $1 AND 
				ur.user_id = $2 AND
				t.id = ur.team_id";
		let stmt = self.client().prepare(query).await.unwrap();
		match self.client().query_one(&stmt, &[team_id, user_id]).await {
			Ok(r) => {
				let team = Team {
					id: r.get::<'_, usize, Uuid>(0),
					name: r.get(1),
					description: r.get::<'_, usize, Option<String>>(2),
					image: r.get::<'_, usize, Option<String>>(3),
					created_at: r.get::<'_, usize, chrono::NaiveDateTime>(4).to_string(),
					updated_at: r.get::<'_, usize, chrono::NaiveDateTime>(5).to_string(),
					created_by: r.get::<'_, usize, Uuid>(6),
					updated_by: r.get::<'_, usize, Uuid>(7),
				};
				debug!("Team: {:#?}", team);
				Ok(team)
			}
			Err(e) => {
				warn!("Error getting team: {}", e);
				Err(e.to_string()) // TODO
			}
		}
	}

	pub async fn delete_team(&self, admin_id: &Uuid, team_id: &Uuid) -> Result<(), String> {
		info!("Deleting team \"{}\" by user {}", team_id, admin_id);
		let query = "SELECT delete_team($1, $2)";
		let stmt = self.client().prepare(query).await.unwrap();
		match self.client().execute(&stmt, &[admin_id, team_id]).await {
			Ok(_) => {
				debug!("Team deleted");
				Ok(())
			},
			Err(e) => {
				warn!("Error deleting team: {}", e);
				Err(e.to_string()) // TODO
			}
		}
	}

	pub async fn get_team_members(&self, team_id: &Uuid) -> Result<Vec<UserRole>, String> {
		info!("Getting team members");
		debug!("Team: {:#?}", team_id);
		let query = "SELECT
				u.id, u.name,
				u.created_at, u.updated_at,
				(SELECT true FROM superusers WHERE user_id = u.id) IS NOT NULL AS is_superuser,
				u.image,
				ur.role
			FROM users u, user_roles ur WHERE
				u.id = ur.user_id AND
				ur.team_id = $1";
		let stmt = self.client().prepare(query).await.unwrap();
		let rows = match self.client().query(&stmt, &[team_id]).await {
			Ok(r) => r,
			Err(e) => {
				warn!("Error getting team members: {}", e);
				return Err(e.to_string()); // TODO
			}
		};
		let mut result = Vec::new();
		for row in rows {
			result.push(UserRole {
				user: User {
					id: row.get::<'_, usize, Uuid>(0).into(),
					name: row.get(1),
					created_at: row.get::<'_, usize, chrono::NaiveDateTime>(2).to_string(),
					updated_at: row.get::<'_, usize, chrono::NaiveDateTime>(3).to_string(),
					is_superuser: row.get::<'_, usize, bool>(4),
					image: row.get::<'_, usize, Option<String>>(5),
				},
				role: row.get::<'_, usize, String>(6),
			});
		}
		debug!("Team members ({}): {:#?}", result.len(), result);
		Ok(result)
	}

	pub async fn add_user_to_team(
		&self,
		team_id: &Uuid,
		user_id: &Uuid,
		new_member: &Uuid,
		user_role: &String,
	) -> Result<(), String> {
		info!("Adding user to team");
		debug!("Adding user \"{}\" to team \"{}\" as {} by user {}", new_member, team_id, user_role, user_id);
		if user_id == new_member {
			return Err("Cannot add yourself to a team".into());
		}
		let query = "SELECT add_user_to_team($1, $2, $3, $4)";
		let stmt = self.client().prepare(query).await.unwrap();
		match self.client().execute(&stmt, &[user_id, team_id, new_member, user_role]).await {
			Ok(_) => {
				debug!("User added to team");
				Ok(())
			},
			Err(e) => {
				warn!("Error adding user to team: {}", e);
				Err(e.to_string()) // TODO
			}
		}
	}
}
