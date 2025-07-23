use tokio_postgres::Client;
use uuid::Uuid;
use argon2::{
	password_hash::{
		rand_core::OsRng,
		PasswordHasher, SaltString,
	},
	Argon2
};

pub struct ShoplistDbAuth {
	db_client: Client,
}

impl ShoplistDbAuth {
	pub fn new(db_client: Client) -> Self {
		Self {
			db_client,
		}
	}

	pub async fn basic_login(&self, username: String, password: String) -> Result<String, ()> {
		println!("Searching for {} with password '{}'", &username, &password);
		let user_id = self.login_basic_user(username, password).await?;
		Ok("OK".to_string()) // TODO
	}
}

impl ShoplistDbAuth {
	fn encrypt_password(&self, password: String) -> String {
		let salt = SaltString::generate(&mut OsRng);
		let argon2 = Argon2::default();
		let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
		hash.to_string()
	}

	async fn login_basic_user(&self, username: String, password: String) -> Result<Uuid, ()> {
		let query = "SELECT user_id FROM basic_login WHERE email = $1 AND password = $2";
		let stmt = self.db_client.prepare(query).await.unwrap();
		match self.db_client.query(&stmt, &[&username, &password]).await {
			Ok(rows) if rows.is_empty() => Err(()),
			Ok(rows) => Ok(rows[0].get(0)),
			Err(_) => Err(())
		}
	}
}
