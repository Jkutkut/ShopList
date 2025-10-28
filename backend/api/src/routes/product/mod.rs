use super::*;

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/product",
		routes![
		],
		catchers![],
		vec![],
	)
}
