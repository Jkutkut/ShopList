use super::*;

#[get("/roles")]
async fn user_team_roles(
	user: guards::User,
) -> Result<Json<UserTeamRoles>, InvalidResponse> {
	info!("Get user team roles");
	debug!("User: {:#?}", user);
	let user_uuid: Uuid = user.uuid.get().unwrap();
	let mut auth_grpc_client = grpc::connect_auth().await.unwrap();
	let request = UserId { id: user_uuid.to_string() };
	debug!("Request: {:#?}", request);
	let team_roles = match auth_grpc_client.team_roles(request).await {
		Ok(response) => response.into_inner(),
		Err(e) => {
			error!("GRPC error: {:#?}", e);
			return Err(invalid_api(&format!("GRPC error: {:?}", e)))
		}
	};
	debug!("Team roles: {:#?}", team_roles);
	Ok(Json(team_roles))
}

#[post("/", data = "<team_request>")]
async fn team_create(
	user: guards::User,
	team_request: Json<TeamRequest>,
	db: &State<DB>,
) -> Result<Json<UuidWrapper>, InvalidResponse> {
	info!("Team create");
	let user_id: Uuid = match user.uuid.get() {
		Ok(id) => id,
		Err(_) => return Err(InvalidResponse::new(Status::BadRequest, "Invalid user id"))
	};
	match db.create_team(&user_id, &team_request).await {
		Ok(team_id) => Ok(Json(team_id.into())),
		Err(err) => Err(InvalidResponse::new(Status::InternalServerError, &err))
	}
}

#[get("/<_>")]
async fn team_get(
	team: guards::Team,
) -> Json<guards::Team> {
	info!("Team get");
	Json(team)
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
	user: guards::User,
	team_id: UuidWrapper,
	db: &State<DB>,
) -> Result<Json<()>, InvalidResponse> {
	let user_id: Uuid = match user.uuid.get() {
		Ok(id) => id,
		Err(_) => return Err(InvalidResponse::new(Status::BadRequest, "Invalid user id"))
	};
	let team_id: Uuid = match team_id.get() {
		Ok(id) => id,
		Err(_) => return Err(InvalidResponse::new(Status::BadRequest, "Invalid team id"))
	};
	match db.delete_team(&user_id, &team_id).await {
		Ok(_) => Ok(Json(())),
		Err(err) => Err(InvalidResponse::new(Status::BadRequest, &err))
	}
}

#[get("/<_>/members")]
async fn team_members(
	team: guards::Team,
) -> Result<Json<Vec<()>>, InvalidResponse> { // TODO output
	info!("Get team members");
	debug!("Team: {:#?}", team);
	Err(route_error::not_implemented()) // TODO
}

#[put("/<_>/members")]
async fn team_members_update(
	team: guards::Team,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	info!("Update team members");
	debug!("Team: {:#?}", team);
	Err(route_error::not_implemented()) // TODO
}

#[delete("/<_>/members/<user_id>")]
async fn team_member_delete(
	#[allow(unused_variables)]
	user: guards::User,
	team: guards::Team,
	#[allow(unused_variables)]
	user_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	info!("Delete team member");
	debug!("Team: {:#?}", team);
	Err(route_error::not_implemented()) // TODO
}

// Tags

#[get("/<_>/tags")]
async fn team_tags(
	team: guards::Team,
) -> Result<Json<Vec<()>>, InvalidResponse> { // TODO output
	info!("Get team tags");
	debug!("Team: {:#?}", team);
	Err(route_error::not_implemented()) // TODO
}

#[post("/<_>/tags")] // TODO data
async fn team_tags_update(
	team: guards::Team,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	info!("Update team tags");
	debug!("Team: {:#?}", team);
	Err(route_error::not_implemented()) // TODO
}

// Product

#[get("/<_>/products")]
async fn team_products(
	team: guards::Team,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	info!("Get team products");
	debug!("Team: {:#?}", team);
	Err(route_error::not_implemented()) // TODO
}

#[post("/<_>/product")] // TODO data
async fn team_product_create(
	team: guards::Team,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	info!("Create team product");
	debug!("Team: {:#?}", team);
	Err(route_error::not_implemented()) // TODO
}

// List

#[get("/<_>/lists")]
async fn team_lists(
	team: guards::Team,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	info!("Get team lists");
	debug!("Team: {:#?}", team);
	Err(route_error::not_implemented()) // TODO
}

#[post("/<_>/list")] // TODO data
async fn team_list_create(
	team: guards::Team,
) -> Result<Json<()>, InvalidResponse> { // TODO output
	info!("Create team list");
	debug!("Team: {:#?}", team);
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
