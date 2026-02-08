use super::*;

#[get("/team/<_>/products")]
async fn team_products(
	team: guards::Team,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	info!("Get team products");
	debug!("Team: {:#?}", team);
	Err(route_error::not_implemented()) // TODO
}

#[post("/team/<_>/product")] // TODO data
async fn team_product_create(
	team: guards::Team,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	info!("Create team product");
	debug!("Team: {:#?}", team);
	Err(route_error::not_implemented()) // TODO
}

#[put("/team/<_>/product/<product_id>")] // TODO data
async fn product_update(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	product_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[delete("/team/<_>/product/<product_id>")]
async fn product_delete(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	product_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> {
	Err(route_error::not_implemented()) // TODO
}

#[post("/product/<product_id>/tags")] // TODO data
async fn product_update_tags(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	product_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[delete("/product/<product_id>/tag/<tag_id>")]
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

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/",
		routes![
			team_products,
			team_product_create,
			product_update,
			product_delete,
			product_update_tags,
			product_delete_tag,
		],
		catchers![],
		vec![],
	)
}
