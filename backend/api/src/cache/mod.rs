use fred::prelude::*;
use std::time::Duration;
use log::*;
use crate::utils::env_var;

pub use fred::prelude::Client;

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
