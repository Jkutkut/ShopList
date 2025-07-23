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

pub struct ShoplistDbAuth {
	db_client: Client,
}

mod model {
	use uuid::Uuid;

	#[derive(Debug)]
	pub struct BasicLogin {
		pub id: Uuid,
		pub user_id: Uuid,
		pub email: String,
		pub password: String,
	}
}
use model::{
	BasicLogin,
};

impl ShoplistDbAuth {
	pub fn new(db_client: Client) -> Self {
		Self {
			db_client,
		}
	}

	pub async fn basic_login(&self, username: String, password: String) -> Result<String, ()> {
		let credentials = self.get_user_credentials(&username).await.ok_or(())?;
		let ok = self.validate_password(password.clone(), credentials.password);
		println!("Password ok: {}", ok);
		Ok(format!("OK: {}", ok)) // TODO
	}
}

impl ShoplistDbAuth {
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
