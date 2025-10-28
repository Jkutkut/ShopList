use super::*;

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/list",
		routes![
		],
		catchers![],
		vec![],
	)
}
