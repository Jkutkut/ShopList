use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use jsonwebtoken::{
	encode, decode,
	Header, Validation,
	Algorithm,
	EncodingKey, DecodingKey,
};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct JWT {
	user_id: String,
	exp: usize
}

impl JWT {
	pub fn new(user_id: &Uuid, exp: usize) -> Self {
		Self {
			user_id: user_id.to_string(),
			exp
		}
	}

	pub fn get_user_id(&self) -> Result<Uuid, ()> {
		Uuid::parse_str(&self.user_id).map_err(|_| ())
	}

	pub fn is_expired(&self) -> bool {
		chrono::Utc::now().timestamp() > self.exp as i64
	}

	pub fn expiration(&self) -> usize {
		self.exp
	}
}

pub struct JWTHandler {
	encoding_key: EncodingKey,
	encoding_header: Header,
	decoding_key: DecodingKey,
	decoding_validation: Validation
}

impl JWTHandler {
	pub fn new(secret: &str) -> Self {
		let secret = secret.as_bytes();
		let encoding_key = EncodingKey::from_secret(secret);
		let encoding_header = Header::new(Algorithm::HS512);
		let decoding_key = DecodingKey::from_secret(secret);
		let decoding_validation = Validation::new(Algorithm::HS512);
		Self {
			encoding_key,
			encoding_header,
			decoding_key,
			decoding_validation
		}
	}

	pub fn new_jwt(&self, user_id: &Uuid) -> JWT {
		// TODO expiration time
		let expiration = Utc::now().checked_add_signed(Duration::hours(2)).unwrap().timestamp();
		JWT::new(user_id, expiration as usize)
	}

	pub fn encode(&self, jwt: &JWT) -> Result<String, ()> {
		match encode(&self.encoding_header, jwt, &self.encoding_key) {
			Ok(token) => Ok(token),
			Err(e) => {
				eprintln!("Failed to create JWT: {}", e);
				Err(()) // TODO
			}
		}
	}

	pub fn decode(&self, token: &str) -> Result<JWT, ()> {
		match decode::<JWT>(token, &self.decoding_key, &self.decoding_validation) {
			Ok(token) => Ok(token.claims),
			Err(e) => {
				eprintln!("Failed to decode JWT: {}", e);
				Err(()) // TODO
			}
		}
	}
}
