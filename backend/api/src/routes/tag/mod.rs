use super::*;

#[delete("/<tag_id>")]
async fn tag_delete(
	#[allow(unused_variables)]
	user: guards::User,
	#[allow(unused_variables)]
	tag_id: UuidWrapper,
) -> Result<Json<()>, InvalidResponse> {
	Err(route_error::not_implemented()) // TODO
}

pub fn routes() -> RouteHandlerBuilder {
	RouteHandlerBuilder::new(
		"/tag",
		routes![
			tag_delete,
		],
		catchers![],
		vec![],
	)
}
