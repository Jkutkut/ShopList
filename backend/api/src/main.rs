use rocket::{
	Config, Build, Rocket,
	launch, routes, catchers,
	get,
};
use std::net::Ipv4Addr;
use rocket::serde::json::Json;

mod route_error;

#[get("/")]
fn ping() -> Json<&'static str> {
	Json(concat!("shoplist-", env!("CARGO_BIN_NAME"), " is up and running"))
}

mod login {
	use rocket::serde::json::Json;
	use rocket::post;
	use serde::Deserialize;
	use crate::route_error::InvalidAPI;

	#[derive(Debug, Deserialize)]
	pub struct BasicCredentials {
		username: String,
		password: String
	}

	#[post("/login/basic", data = "<credentials>")]
	pub async fn basic(
		credentials: Json<BasicCredentials>
	) -> Result<Json<String>, InvalidAPI> {
		println!("Credentials: {:?}", credentials);
		Err("WIP: Not implemented".into())
	}
}

fn config() -> Config {
	#[cfg(debug_assertions)]
	{
		Config::debug_default()
	}
	#[cfg(not(debug_assertions))]
	{
		Config::release_default()
	}
}

#[launch]
async fn rocket() -> Rocket<Build> {
	let port = 80;

	let config = Config {
		address: Ipv4Addr::new(0, 0, 0, 0).into(),
		port,
		..config()
	};
	rocket::custom(&config)
		.mount("/", routes![
			ping,
			login::basic
		])
		.register("/", catchers![
			route_error::not_implemented,
			route_error::not_found,
			route_error::internal_server_error,
		])
}
