use rocket::{
	Build, Rocket,
	launch,
};

mod cors;
mod route_handler;
mod route_error;
mod api_user_token;
mod routes;
mod guards;

#[launch]
async fn rocket() -> Rocket<Build> {
	let mut r = rocket::build()
		.attach(cors::CORS);
	let api = routes::routes().build();
	for (path, routes) in api.routes {
		r = r.mount(&path, routes);
	}
	for (path, catcher) in api.catchers {
		r = r.register(&path, catcher);
	}
	r
}
