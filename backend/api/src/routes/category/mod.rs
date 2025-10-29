use super::*;

#[put("/<category_id>")] // TODO data
async fn update_category(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	category_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[delete("/<category_id>")]
async fn delete_category(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	category_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> {
	Err(route_error::not_implemented()) // TODO
}

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/category",
		routes![
			update_category,
			delete_category,
		],
		catchers![],
		vec![],
	)
}
