use std::time::Duration;
use serde::{
	Serialize,
	de::DeserializeOwned,
};
use log::*;
use chrono::{
	Utc,
	TimeZone,
};
use crate::utils::env_var;

pub use fred::prelude::*;

pub struct Cache {
	client: Client,
}

impl Cache {
	pub async fn init() -> Result<Self, String> {
		let url = format!("redis://{}:{}",
			env_var("VALKEY_HOST")?,
			env_var("VALKEY_PORT")?
		);
		let cache = match Self::init_or(&url).await {
			Ok(client) => client,
			Err(e) => return Err(format!("{}", e)),
		};
		cache.client.on_error(|(error, server)| async move {
			error!("{:?}: Connection error: {:?}", server, error);
			Ok(())
		});
		Ok(cache)
	}

	async fn init_or(url: &str) -> Result<Self, Error> {
		let config = Config::from_url(&url)?;
		let client = Builder::from_config(config)
			.with_connection_config(|config| {
				config.connection_timeout = Duration::from_secs(5);
				config.tcp = TcpConfig {
					nodelay: Some(true),
					..Default::default()
				};
			})
			.build()?;
		client.init().await?;
		Ok(Self { client: client })
	}

	/// Obtains the given value from the given function.
	///
	/// The value is cached with the given key and expiration.
	pub async fn cached_value<T, E, Ft, F>(
		&self,
		key: &str,
		expiration: Option<Expiration>,
		ft_fetch_value: Ft,
	) -> Result<T, E>
	where
		T: Serialize + DeserializeOwned,
		Ft: FnOnce() -> F,
		F: std::future::Future<Output = Result<T, E>>,
	{
		debug!("Obtaining obj for key {}", key);
		match self.try_get(key).await {
			Some(obj) => return Ok(obj),
			_ => info!("Not found in cache"),
		}
		info!("fetching value...");
		match ft_fetch_value().await {
			Ok(obj) => {
				info!("Successfully fetched");
				self.set(key, &obj, expiration).await;
				Ok(obj)
			},
			Err(e) => Err(e),
		}
	}

	// TODO use binary for cache

	pub async fn try_get<T>(
		&self,
		key: &str,
	) -> Option<T>
	where
		T: DeserializeOwned,
	{
		info!("try_get: {}", key);
		match self.client.get::<Option<String>, _>(key).await {
			Ok(Some(obj)) => {
				let obj: T = serde_json::from_str(&obj).unwrap();
				info!("cache hit!");
				return Some(obj);
			},
			Err(e) => error!("Failed to get from cache: {}", e),
			_ => (),
		};
		info!("cache miss");
		None
	}

	pub async fn set<T>(
		&self,
		key: &str,
		value: &T,
		expiration: Option<Expiration>,
	) where
		T: Serialize,
	{
		info!("Caching {key}");
		debug!("Caching for {}", Self::expiration_to_str(&expiration));
		let json = serde_json::to_string(&value).unwrap(); // TODO why can this fail?
		debug!("JSON: {}", json);
		match self.client.set::<String, _, _>(
			key, json,
			expiration,
			None, // Options
			false, // Get
		).await {
			Ok(_) => debug!("Obj set in cache"),
			Err(e) => error!("Failed to set obj in cache: {}", e),
		}
	}

	fn expiration_to_str(exp: &Option<Expiration>) -> String {
		match exp {
			None => "infinite".into(),
			Some(Expiration::EX(exp)) => format!("{} min", exp / 60),
			Some(Expiration::PX(exp)) => format!("{} s", exp / 1000),
			Some(Expiration::EXAT(exp)) => {
				let now = Utc::now();
				let time = Utc.timestamp_opt(*exp, 0).unwrap();
				let duration = time.signed_duration_since(now);
				format!("{} min", duration.num_seconds() / 60)
			},
			Some(Expiration::PXAT(exp)) => {
				let now = Utc::now();
				let time = Utc.timestamp_opt(*exp / 1000, 0).unwrap();
				let duration = time.signed_duration_since(now);
				format!("{} s", duration.num_seconds())
			}
			Some(Expiration::KEEPTTL) => "\"Do not reset the TTL\"".into(),
		}
	}
}
