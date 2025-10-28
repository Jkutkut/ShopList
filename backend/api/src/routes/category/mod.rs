use super::*;

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/category",
		routes![
		],
		catchers![],
		vec![],
	)
}
