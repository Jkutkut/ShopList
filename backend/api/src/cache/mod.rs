use std::time::Duration;
use serde::{
	Serialize,
	de::DeserializeOwned,
};
use log::*;
use crate::utils::env_var;

pub use fred::prelude::*;

pub async fn init_or(url: &str) -> Result<Client, Error> {
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
	Ok(client)
}

pub async fn init() -> Result<Client, String> {
	let url = format!("redis://{}:{}",
		env_var("VALKEY_HOST")?,
		env_var("VALKEY_PORT")?
	);
	let client = match init_or(&url).await {
		Ok(client) => client,
		Err(e) => return Err(format!("{}", e)),
	};
	client.on_error(|(error, server)| async move {
		error!("{:?}: Connection error: {:?}", server, error);
		Ok(())
	});
	Ok(client)
}

/// Obtains the given value from the given function.
///
/// The value is cached with the given key and expiration.
pub async fn cached_value<T, E, Ft, F>(
	cache_client: &Client,
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
	match try_get_cached_value(cache_client, key).await {
		Some(obj) => return Ok(obj),
		_ => info!("Not found in cache"),
	}
	match ft_fetch_value().await {
		Ok(obj) => {
			info!("Obj obtained from fetch");
			cache_value(cache_client, key, &obj, expiration).await;
			Ok(obj)
		},
		Err(e) => Err(e),
	}
}

// TODO use binary for cache

async fn try_get_cached_value<T>(
	cache_client: &Client,
	key: &str,
) -> Option<T>
where
	T: DeserializeOwned,
{
	match cache_client.get::<Option<String>, _>(key).await {
		Ok(Some(obj)) => {
			let obj: T = serde_json::from_str(&obj).unwrap();
			debug!("Obj obtained from cache");
			return Some(obj);
		},
		Err(e) => error!("Failed to get from cache: {}", e),
		_ => (),
	};
	None
}

async fn cache_value<T>(
	cache_client: &Client,
	key: &str,
	value: &T,
	expiration: Option<Expiration>,
) where
	T: Serialize,
{
	info!("Caching {key}");
	let json = serde_json::to_string(&value).unwrap(); // TODO why can this fail?
	debug!("JSON: {}", json);
	match cache_client.set::<String, _, _>(
		key, json,
		expiration,
		None, // Options
		false, // Get
	).await {
		Ok(_) => debug!("Obj set in cache"),
		Err(e) => error!("Failed to set obj in cache: {}", e),
	}
}
