use super::*;

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/team",
		routes![
		],
		catchers![],
		vec![],
	)
}
