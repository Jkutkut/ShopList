mod category;
mod list;
mod product;
mod tag;
mod team;
mod user;

use uuid::Uuid;
use rocket::{
	get, post, delete,
	http::Status,
	routes,
	catchers,
	serde::json::Json,
};
use model::*;
use model::{
	grpc::auth::*,
	grpc::auth::{
		auth_service_client::AuthServiceClient,
	},
};
use crate::{
	User,
	cors,
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
	/*
	let users = RouteHandlerBuilder::new(
		"/user",
		routes![
			me::me,
			user::delete_user,
			login::basic,
			register::basic,
		],
		catchers![],
		vec![],
	);
	let api = RouteHandlerBuilder::new(
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
				vec![users]
			),
		],
	).build();
	*/
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
