use tokio_postgres::Client;
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
use model::jwt::JWTHandler;

pub struct ShoplistDbAuth {
	db_client: Client,
	jwt: JWTHandler
}

use model::{
	BasicLogin,
};

impl ShoplistDbAuth {
	pub fn new(db_client: Client, jwt: JWTHandler) -> Self {
		Self {
			db_client,
			jwt
		}
	}

	// TODO logout_everyone
	// TODO logout_user_everywhere

	pub async fn basic_login(&self, username: String, password: String) -> Result<String, ()> {
		let credentials = self.get_user_credentials(&username).await.ok_or(())?;
		let ok = self.validate_password(password.clone(), credentials.password);
		println!("Password ok: {}", ok);
		Ok(self.new_jwt(&credentials.user_id).await?)
	}

	pub async fn register_user_basic_login(&self, name: String, email: String, password: String) -> Result<String, ()> {
		let password_hash = self.encrypt_password(password);
		let query = "SELECT create_user_basic_credentials($1, $2, $3)";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.query_one(&stmt, &[&name, &email, &password_hash]).await {
			Ok(r) => Ok(self.new_jwt(&r.get(0)).await?),
			Err(_) => Err(()) // TODO error?
		}
	}

	pub async fn delete_user(&self, auth_user_id: &Uuid, user_id: &Uuid) -> Result<(), Status> {
		if user_id != auth_user_id {
			let query = "SELECT user_id FROM superusers WHERE user_id = $1";
			let stmt = self.db_client.prepare(query).await.unwrap();
			match self.db_client.query_one(&stmt, &[&auth_user_id]).await {
				Ok(_) => (),
				Err(_) => return Err(Status::permission_denied("Invalid credentials"))
			};
		}
		let query = "DELETE FROM basic_login WHERE user_id = $1";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.execute(&stmt, &[&user_id]).await {
			Ok(r) if r == 1 => Ok(()),
			_ => Err(Status::not_found("User not found"))
		}
	}
}

impl ShoplistDbAuth {
	async fn new_jwt(&self, user_id: &Uuid) -> Result<String, ()> {
		let token = self.jwt.new_jwt(user_id);
		let token_str: String = self.jwt.encode(&token)?;
		println!("New token for {}: {}", user_id, &token_str);
		let query = "SELECT create_credentials($1, $2, to_timestamp($3)::timestamp)";
		let stmt = self.db_client.prepare(query).await.unwrap();
		self.db_client.execute(&stmt, &[
			user_id, &token_str, &(token.expiration() as f64)
		]).await.map_err(|_| ())?;
		Ok(token_str)
	}

	fn validate_password(&self, password: String, password_hash: String) -> bool {
		let argon2 = Argon2::default();
		let password_hash = match PasswordHash::new(&password_hash) {
			Ok(hash) => hash,
			Err(_) => return false
		};
		argon2.verify_password(password.as_bytes(), &password_hash).is_ok()
	}

	fn encrypt_password(&self, password: String) -> String {
		let salt = SaltString::generate(&mut OsRng);
		let argon2 = Argon2::default();
		let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
		hash.to_string()
	}

	async fn get_user_credentials(&self, username: &str) -> Option<BasicLogin> {
		let query = "SELECT id, user_id, email, password FROM basic_login WHERE email = $1";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.query(&stmt, &[&username]).await {
			Ok(rows) if rows.is_empty() => return None,
			Ok(rows) if rows.len() == 1 => Some(BasicLogin {
				id: rows[0].get(0),
				user_id: rows[0].get(1),
				email: rows[0].get(2),
				password: rows[0].get(3),
			}),
			Ok(_) => unreachable!(),
			Err(_) => return None
		}
	}
}
