use super::*;

#[get("/roles")]
async fn user_team_roles(
	#[allow(unused_variables)]
	user: guards::User,
) -> Result<Json<Vec<()>>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[post("/")] // TODO data
async fn team_create(
	#[allow(unused_variables)]
	user: guards::User,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[get("/<team_id>")]
async fn team_get(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	team_id: String, // TODO input
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[put("/<team_id>")]
async fn team_update(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	team_id: String, // TODO input
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[delete("/<team_id>")]
async fn team_delete(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	team_id: String, // TODO input
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[get("/<team_id>/members")]
async fn team_members(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	team_id: String, // TODO input
) -> Result<Json<Vec<()>>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[put("/<team_id>/members")]
async fn team_members_update(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	team_id: String, // TODO input
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[delete("/<team_id>/members/<user_id>")]
async fn team_member_delete(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	team_id: String, // TODO input
	#[allow(unused_variables)]
	user_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

// Tags

#[get("/<team_id>/tags")]
async fn team_tags(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	team_id: String, // TODO input
) -> Result<Json<Vec<()>>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[post("/<team_id>/tags")] // TODO data
async fn team_tags_update(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	team_id: String, // TODO input
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

// Product

#[get("/<team_id>/products")]
async fn team_products(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	team_id: String, // TODO input
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[post("/<team_id>/product")] // TODO data
async fn team_product_create(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	team_id: String, // TODO input
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

// List

#[get("/<team_id>/lists")]
async fn team_lists(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	team_id: String, // TODO input
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

#[post("/<team_id>/list")] // TODO data
async fn team_list_create(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	team_id: String, // TODO input
) -> Result<Json<()>, InvalidResponse> { // TODO output
	Err(route_error::not_implemented()) // TODO
}

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/team",
		routes![
			user_team_roles,
			team_create,
			team_get,
			team_update,
			team_delete,
			team_members,
			team_members_update,
			team_member_delete,
			// Tags
			team_tags,
			team_tags_update,
			// Product
			team_products,
			team_product_create,
			// List
			team_lists,
			team_list_create,
		],
		catchers![],
		vec![],
	)
}
