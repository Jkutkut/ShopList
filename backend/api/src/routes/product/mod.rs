use super::*;

#[get("/<_>/products")]
async fn team_products(
	team: guards::Team,
	db: &State<DB>,
) -> Result<Json<Vec<Product>>, InvalidResponse> {
	info!("Get team products");
	debug!("Team: {:#?}", team);
	match db.get_products(&team.id).await {
		Ok(products) => Ok(Json(products)),
		Err(err) => Err(InvalidResponse::new(Status::BadRequest, &err))
	}
}

#[post("/<_>/product", data = "<product_request>")]
async fn team_product_create(
	user: guards::User,
	team: guards::Team,
	product_request: Json<ProductRequest>,
	db: &State<DB>,
) -> Result<Json<Product>, InvalidResponse> {
	info!("Create team product");
	let user_id = user.id.get().unwrap();
	debug!("Team: {:#?}, Product: {:#?}, user_id: {:#?}", team, product_request, user_id);
	match db.create_product(&team.id, &user_id, &product_request).await {
		Ok(product) => Ok(Json(product)),
		Err(err) => Err(InvalidResponse::new(Status::BadRequest, &err))
	}
}

#[put("/team/<_>/product/<product_id>", data = "<product_request>")]
async fn product_update(
	team: guards::Team,
	product_id: UuidWrapper,
	product_request: Json<ProductRequest>,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	info!("Update team product");
	debug!("Team: {:#?}, Product: {:#?}, Product request: {:#?}", team, product_id, product_request);
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
			product_update_tags,
			product_delete_tag,
		],
		catchers![],
		vec![
			RouteHandlerBuilder::new(
				"/team",
				routes![
					team_products,
					team_product_create,
					product_update,
					product_delete,
				],
				catchers![],
				vec![],
			)
		],
	)
}
