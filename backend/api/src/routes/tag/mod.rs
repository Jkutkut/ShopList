use super::*;

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/tag",
		routes![
		],
		catchers![],
		vec![],
	)
}
