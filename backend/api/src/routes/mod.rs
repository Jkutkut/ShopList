mod category;
mod list;
mod product;
mod tag;
mod team;
mod user;

use uuid::Uuid;
use log::*;
use rocket::{
	get, post, delete, put,
	http::Status,
	routes,
	catchers,
	serde::json::Json,
	State,
};
use chrono::{
	Utc,
	DateTime,
};
use model::*;
use model::{
	grpc::auth::*,
};
use crate::{
	cache,
	cache::{
		Cache,
	},
	cors,
	guards,
	grpc,
	api_user_token::ApiUserToken,
	route_error,
	route_error::{InvalidResponse, invalid_api},
	route_handler::RouteHandlerBuilder,
};

#[get("/")]
fn ping() -> Json<&'static str> {
	Json(concat!(
		"shoplist-", env!("CARGO_BIN_NAME"),
		" is up and running v", env!("CARGO_PKG_VERSION")
	))
}

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/",
		routes![cors::options],
		catchers![
			route_error::not_implemented,
			route_error::unauthorized,
			route_error::not_found,
			route_error::internal_server_error,
		],
		vec![
			RouteHandlerBuilder::new(
				"/api",
				routes![ping],
				catchers![],
				vec![]
			),
			RouteHandlerBuilder::new(
				"/api/v1",
				routes![],
				catchers![],
				vec![
					category::routes(),
					list::routes(),
					product::routes(),
					tag::routes(),
					team::routes(),
					user::routes(),
				]
			),
		],
	)
}
