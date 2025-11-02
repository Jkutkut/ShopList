use tokio_postgres::{
	Client,
};
use uuid::Uuid;
use argon2::{
	password_hash::{
		rand_core::OsRng,
		PasswordHasher, SaltString,
		PasswordHash,
	},
	Argon2,
	PasswordVerifier,
};
use tonic::Status;
use model::{
	jwt::JWTHandler,
	BasicLogin,
	grpc::auth::{
		User,
		UserToken,
		Team,
		TeamRole,
		UserTeamRoles,
	}
};
use log::*;

pub struct ShoplistDbAuth {
	#[cfg(not(test))]
	db_client: Client,
	#[cfg(test)]
	pub db_client: Client,
	jwt: JWTHandler
}

impl ShoplistDbAuth {
	pub fn new(db_client: Client, jwt: JWTHandler) -> Self {
		Self {
			db_client,
			jwt
		}
	}

	pub async fn basic_login(&self, email: String, password: String) -> Result<UserToken, ()> {
		info!("Login with email: {}", email);
		debug!("Password: {}", password);
		let credentials = self.get_user_credentials(&email).await.ok_or(())?;
		match self.validate_password(password.clone(), credentials.password) {
			true => {
				info!("Password ok");
				Ok(self.new_jwt(&credentials.user_id).await?)
			},
			false => {
				info!("Password not ok");
				Err(())
			}
		}
	}

	pub async fn register_user_basic_login(&self, name: String, email: String, password: String) -> Result<UserToken, ()> {
		let password_hash = self.encrypt_password(password);
		let query = "SELECT create_user_basic_credentials($1, $2, $3)";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.query_one(&stmt, &[&name, &email, &password_hash]).await {
			Ok(r) => Ok(self.new_jwt(&r.get(0)).await?),
			Err(e) => {
				warn!("Error creating user: {}", e);
				Err(())
			}
		}
	}

	pub async fn basic_change_password(&self, token: String, user_id: &Uuid, password: String) -> Result<(), Status> {
		info!("Change password for user: {}", user_id);
		if !self.can_modify_user(user_id, &token).await {
			warn!("Invalid credentials to change password");
			return Err(Status::permission_denied("Invalid credentials"));
		}
		let password_hash = self.encrypt_password(password);
		let query = "UPDATE basic_login SET password = $1 WHERE user_id = $2";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.execute(&stmt, &[&password_hash, &user_id]).await {
			Ok(r) if r == 1 => {
				info!("Password changed for user: {}", user_id);
				Ok(())
			},
			Ok(r) if r == 0 => {
				warn!("User not found");
				Err(Status::not_found("User not found"))
			},
			Ok(_) => unreachable!(),
			Err(e) => {
				warn!("Error changing password");
				debug!("Error: {}", e);
				Err(Status::unauthenticated("Invalid credentials"))
			}
		}
	}

	pub async fn delete_user(&self, auth_user_id: &Uuid, user_id: &Uuid) -> Result<(), Status> {
		info!("Delete user {} by being logged in as {}", user_id, auth_user_id);
		if user_id != auth_user_id {
			let query = "SELECT user_id FROM superusers WHERE user_id = $1";
			let stmt = self.db_client.prepare(query).await.unwrap();
			match self.db_client.query_one(&stmt, &[&auth_user_id]).await {
				Ok(_) => (),
				Err(e) => {
					warn!("Error deleting user: {}", e);
					return Err(Status::permission_denied("Invalid credentials"))
				}
			};
			debug!("User {} is a superuser", auth_user_id);
		}
		let query = "DELETE FROM basic_login WHERE user_id = $1";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.execute(&stmt, &[&user_id]).await {
			Ok(r) if r == 1 => {
				info!("Deleted user {}", user_id);
				Ok(())
			},
			e => {
				error!("Error deleting user: {:?}", e);
				Err(Status::not_found("User not found"))
			}
		}
	}

	pub async fn me(&self, token: &str) -> Result<User, Status> {
		info!("me request with token");
		let query = "SELECT
			u.id, u.name,
			u.created_at, u.updated_at,
			(SELECT true FROM superusers WHERE user_id = u.id) IS NOT NULL AS is_superuser,
			u.image
		FROM users u JOIN credentials c
		ON u.id = c.user_id
		WHERE c.token = $1 AND c.expires_at > now()";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.query_one(&stmt, &[&token]).await {
			Ok(r) => {
				let user = User {
					uuid: r.get::<'_, usize, Uuid>(0).to_string(),
					name: r.get(1),
					created_at: r.get::<'_, usize, chrono::NaiveDateTime>(2).to_string(),
					updated_at: r.get::<'_, usize, chrono::NaiveDateTime>(3).to_string(),
					is_superuser: r.get::<'_, usize, bool>(4),
					image: r.get::<'_, usize, Option<String>>(5)
				};
				debug!("User: {:#?}", user);
				Ok(user)
			},
			Err(e) => {
				error!("Error getting user: {:?}", e);
				Err(Status::unauthenticated("Invalid token"))
			}
		}
	}

	pub async fn logout(&self, token: &str) -> Result<(), Status> {
		info!("logout request with token: {}", token);
		let query = "DELETE FROM credentials WHERE token = $1";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.execute(&stmt, &[&token.to_string()]).await {
			Ok(r) if r <= 1 => {
				debug!("Logged out user");
				Ok(())
			},
			e => {
				error!("Error logging out: {:?}", e);
				Err(Status::unauthenticated("Invalid token"))
			}
		}
	}

	pub async fn logout_user(&self, auth_token: &str, user_id: &Uuid) -> Result<(), Status> {
		info!("logout_user request with auth_token: {}, user_id: {}", auth_token, user_id);
		if !self.can_modify_user(user_id, auth_token).await {
			warn!("Invalid credentials to logout user");
			return Err(Status::permission_denied("Invalid credentials"));
		}
		let query = "DELETE FROM credentials WHERE user_id = $1";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.execute(&stmt, &[&user_id]).await {
			Ok(r) if r >= 1 => {
				info!("User {} logged out", user_id);
				Ok(())
			},
			Ok(r) if r == 0 => {
				warn!("User {} not logged in", user_id);
				Err(Status::not_found("User not found"))
			},
			e => {
				error!("Error logging out: {:?}", e);
				Err(Status::unauthenticated("Invalid token"))
			}
		}
	}

	pub async fn logout_everyone(&self, token: &str) -> Result<(), Status> {
		if !self.is_superuser(token).await {
			warn!("Invalid credentials to logout everyone");
			return Err(Status::permission_denied("Invalid credentials"));
		}
		info!("logout_everyone request with token: {}", token);
		let query = "DELETE FROM credentials";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.execute(&stmt, &[]).await {
			Ok(r) => {
				info!("Logged out {} users", r);
				Ok(())
			},
			e => {
				error!("Error logging out: {:?}", e);
				Err(Status::unauthenticated("Invalid token"))
			}
		}
	}

	pub async fn refresh_token(&self, token: &str) -> Result<UserToken, Status> {
		info!("refresh_token request with token: {}", token);
		let user_id = match self.get_user_id(token).await {
			Ok(user_id) => user_id,
			Err(e) => {
				error!("Error refreshing token: {:?}", e);
				return Err(Status::unauthenticated("Invalid token"));
			}
		};
		let token = match self.new_jwt(&user_id).await {
			Ok(token) => token,
			Err(e) => {
				error!("Error refreshing token: {:?}", e);
				return Err(Status::internal("Internal error"));
			}
		};
		info!("Refreshed token for user: {}", user_id);
		Ok(token)
	}

	pub async fn team_roles(&self, user_id: &Uuid) -> Result<UserTeamRoles, Status> {
		info!("team_roles request with user_id: {}", user_id);
		let query = "SELECT * FROM team_roles($1);";
		let stmt = self.db_client.prepare(query).await.unwrap();
		let rows = match self.db_client.query(&stmt, &[&user_id]).await {
			Ok(rows) => rows,
			Err(e) => {
				if e.to_string().contains("not exists") {
					return Err(Status::not_found("User not found"));
				}
				error!("Error getting user roles: {}", e);
				return Err(Status::internal("Internal error"));
			}
		};
		let mut team_roles = Vec::new();
		for row in rows {
			let uuid: Uuid = row.get(1);
			let created_at: chrono::NaiveDateTime = row.get(5);
			let created_by: Uuid = row.get(6);
			let updated_at: chrono::NaiveDateTime = row.get(7);
			let updated_by: Uuid = row.get(8);
			let team = Team {
				uuid: uuid.to_string(),
				name: row.get(2),
				description: row.get(3),
				image: row.get(4),
				created_at: created_at.to_string(),
				created_by: created_by.to_string(),
				updated_at: updated_at.to_string(),
				updated_by: updated_by.to_string(),
			};
			let role = TeamRole {
				role: row.get(0),
				team: Some(team)
			};
			debug!("User role: {:#?}", role);
			team_roles.push(role);
		}
		Ok(UserTeamRoles { team_roles })
	}
}

impl ShoplistDbAuth {
	async fn new_jwt(&self, user_id: &Uuid) -> Result<UserToken, ()> {
		info!("New JWT for user: {}", user_id);
		let token = self.jwt.new_jwt(user_id);
		let token_str: String = self.jwt.encode(&token)?;
		debug!("New token for {}: {}", user_id, &token_str);
		let query = "SELECT create_credentials($1, $2, to_timestamp($3)::timestamp)";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.execute(&stmt, &[
			user_id, &token_str, &(token.expiration() as f64)
		]).await {
			Ok(r) if r == 1 => {
				debug!("Created credentials for user {}: credential_id {}", user_id, r);
				Ok(UserToken {
					token: token_str,
					user_id: user_id.to_string(),
					expires_at: token.expiration_date_str(),
				})
			},
			e => {
				error!("Error creating credentials: {:?}", e);
				Err(())
			}
		}
	}

	fn validate_password(&self, password: String, password_hash: String) -> bool {
		debug!("Validating password");
		let argon2 = Argon2::default();
		let password_hash = match PasswordHash::new(&password_hash) {
			Ok(hash) => hash,
			Err(_) => return false
		};
		let result = argon2.verify_password(password.as_bytes(), &password_hash).is_ok();
		debug!("Password valid: {}", result);
		result
	}

	fn encrypt_password(&self, password: String) -> String {
		debug!("Encrypting password");
		let salt = SaltString::generate(&mut OsRng);
		let argon2 = Argon2::default();
		let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
		let hash = hash.to_string();
		debug!("Password encrypted: {}", &hash);
		hash
	}

	async fn get_user_credentials(&self, email: &str) -> Option<BasicLogin> {
		info!("Getting user credentials for email: {}", email);
		let query = "SELECT id, user_id, email, password FROM basic_login WHERE email = $1";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.query(&stmt, &[&email]).await {
			Ok(rows) if rows.is_empty() => {
				info!("User not found");
				return None
			},
			Ok(rows) if rows.len() == 1 => {
				let login = BasicLogin {
					id: rows[0].get(0),
					user_id: rows[0].get(1),
					email: rows[0].get(2),
					password: rows[0].get(3),
				};
				info!("User found: {}", login.user_id);
				Some(login)
			},
			Ok(_) => unreachable!(),
			Err(e) => {
				error!("Error getting user credentials: {}", e);
				return None
			}
		}
	}

	async fn can_modify_user(&self, user_id: &Uuid, token: &str) -> bool {
		info!("Checking if can modify user: {}", user_id);
		let query = "SELECT EXISTS (
			SELECT 1 FROM credentials WHERE user_id = $1 AND token = $2 AND expires_at > now()
		) OR EXISTS (
			SELECT 1 FROM credentials, superusers
			WHERE credentials.token = $2 AND credentials.expires_at > now() AND credentials.user_id = superusers.user_id
		);";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.query_one(&stmt, &[&user_id, &token]).await {
			Ok(r) => {
				debug!("Can modify user: {:?}", r);
				let r = r.get::<'_, usize, bool>(0);
				info!("Can modify user: {:?}", r);
				r
			},
			Err(e) => {
				error!("Error checking if can modify user: {:?}", e);
				false
			}
		}
	}

	async fn is_superuser(&self, token: &str) -> bool {
		info!("Checking if is superuser");
		let query = "SELECT EXISTS (
			SELECT 1 FROM credentials, superusers
			WHERE credentials.token = $1 AND credentials.expires_at > now() AND credentials.user_id = superusers.user_id
		);";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.query_one(&stmt, &[&token]).await {
			Ok(r) => {
				debug!("Is superuser: {:?}", r);
				let r = r.get::<'_, usize, bool>(0);
				info!("Is superuser: {:?}", r);
				r
			},
			Err(e) => {
				error!("Error checking if is superuser: {:?}", e);
				false
			}
		}
	}

	async fn get_user_id(&self, token: &str) -> Result<Uuid, ()> {
		info!("Getting user id for token: {}", token);
		let query = "SELECT user_id FROM credentials WHERE token = $1 AND expires_at > now()";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.query_one(&stmt, &[&token]).await {
			Ok(r) => {
				let user_id = r.get::<'_, usize, Uuid>(0);
				debug!("User id: {}", &user_id);
				Ok(user_id)
			},
			Err(e) => {
				error!("Error getting user id: {:?}", e);
				Err(())
			}
		}
	}
}
