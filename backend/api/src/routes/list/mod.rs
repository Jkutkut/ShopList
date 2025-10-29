use super::*;

// PUT /<list_id>
// DELETE /<list_id>
// GET /<list_id>/products
// PUT /<list_id>/products
// DELETE /<list_id>/products/<product_id>

#[put("/<list_id>")] // TODO data
async fn list_update(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	list_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[delete("/<list_id>")]
async fn list_delete(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	list_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[get("/<list_id>/products")]
async fn list_products(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	list_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[put("/<list_id>/products/<product_id>")] // TODO data
async fn list_products_update(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	list_id: UuidWrapper,
	#[allow(unused_variables)]
	product_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[delete("/<list_id>/products/<product_id>")]
async fn list_product_delete(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	list_id: UuidWrapper,
	#[allow(unused_variables)]
	product_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> {
	Err(route_error::not_implemented()) // TODO
}

// Categories

#[get("/<list_id>/categories")]
async fn list_categories(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	list_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[post("/<list_id>/category")] // TODO data
async fn list_category_create(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	list_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/list",
		routes![
			list_update,
			list_delete,
			list_products,
			list_products_update,
			list_product_delete,
			// Categories
			list_categories,
			list_category_create,
		],
		catchers![],
		vec![],
	)
}
