use rocket::{
	Build, Rocket,
	launch,
	fairing::AdHoc,
};
use fred::interfaces::ClientLike;
use log::*;

mod cache;
mod cors;
mod db;
mod route_handler;
mod route_error;
mod api_user_token;
mod routes;
mod guards;
mod grpc;
mod utils;

#[cfg(test)]
mod tests;

#[launch]
async fn rocket() -> Rocket<Build> {
	let env_path = std::env::var("ENV_PATH");
	if let Ok(env_path) = env_path {
		info!("Loading environment variables from {}", env_path);
		dotenv::from_path(env_path).ok();
	}
	env_logger::init();

	let db_client = db::connect_to_db_or_end().await.unwrap_or_else(|e| {
		error!("Failed to connect to DB: {}", e);
		std::process::exit(1);
	});
	let cache_client = cache::Cache::init().await.unwrap_or_else(|e| {
		error!("Failed to initialize cache: {}", e);
		std::process::exit(1);
	});

	let mut r = rocket::build()
		.manage(db_client)
		.manage(cache_client)
		.attach(cors::CORS)
		.attach(AdHoc::on_shutdown("Shutdown cache", |r| Box::pin(async move {
			info!("Shutting down cache connection...");
			let cache_client = r.state::<cache::Client>().unwrap();
			match cache_client.quit().await {
				Ok(_) => info!("Cache connection closed"),
				Err(e) => error!("Failed to close cache connection: {}", e),
			}
		})));
	let api = routes::routes().build();
	for (path, routes) in api.routes {
		r = r.mount(&path, routes);
	}
	for (path, catcher) in api.catchers {
		r = r.register(&path, catcher);
	}
	r
}
