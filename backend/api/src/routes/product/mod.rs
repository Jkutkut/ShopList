use super::*;

#[post("/<product_id>/tags")] // TODO data
async fn product_update_tags(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	product_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[delete("/<product_id>/tag/<tag_id>")]
async fn product_delete_tag(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	product_id: UuidWrapper,
	#[allow(unused_variables)]
	tag_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> {
	Err(route_error::not_implemented()) // TODO
}

#[put("/<product_id>")] // TODO data
async fn product_update(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	product_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[delete("/<product_id>")]
async fn product_delete(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	product_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> {
	Err(route_error::not_implemented()) // TODO
}


pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/product",
		routes![
			product_update_tags,
			product_delete_tag,
			product_update,
			product_delete,
		],
		catchers![],
		vec![],
	)
}
