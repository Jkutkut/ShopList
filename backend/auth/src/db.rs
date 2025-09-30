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
	grpc::auth::User,
};

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

	// TODO logout_everyone
	// TODO logout_user_everywhere

	pub async fn basic_login(&self, email: String, password: String) -> Result<String, ()> {
		let credentials = self.get_user_credentials(&email).await.ok_or(())?;
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

	pub async fn me(&self, token: &str) -> Result<User, Status> {
		println!("me request with token");
		let query = "SELECT
			u.id, u.name,
			u.created_at, u.updated_at,
			(SELECT true FROM superusers WHERE user_id = u.id) IS NOT NULL AS is_superuser,
			u.image
		FROM users u JOIN credentials c
		ON u.id = c.user_id
		WHERE c.token = $1";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.query_one(&stmt, &[&token]).await {
			Ok(r) => Ok(User {
				uuid: r.get::<'_, usize, Uuid>(0).to_string(),
				name: r.get(1),
				created_at: r.get::<'_, usize, chrono::NaiveDateTime>(2).to_string(),
				updated_at: r.get::<'_, usize, chrono::NaiveDateTime>(3).to_string(),
				is_superuser: r.get::<'_, usize, bool>(4),
				image: r.get::<'_, usize, Option<String>>(5)
			}),
			Err(_) => Err(Status::unauthenticated("Invalid token"))
		}
	}

	pub async fn logout(&self, token: &str) -> Result<(), Status> {
		println!("logout request with token: {}", token);
		let query = "DELETE FROM credentials WHERE token = $1";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.execute(&stmt, &[&token.to_string()]).await {
			Ok(r) if r <= 1 => Ok(()),
			_ => Err(Status::unauthenticated("Invalid token"))
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

	async fn get_user_credentials(&self, email: &str) -> Option<BasicLogin> {
		let query = "SELECT id, user_id, email, password FROM basic_login WHERE email = $1";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.query(&stmt, &[&email]).await {
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
