use rocket::{
	http::{
		Cookie,
		CookieJar,
	},
	time::{
		OffsetDateTime,
	}
};
use chrono::{
	DateTime,
};
use model::grpc::auth::UserToken;
use log::*;

pub fn add_user_token_cookie(
	cookie_jar: &CookieJar<'_>,
	user_data: &UserToken,
) {
	let datetime = match DateTime::parse_from_rfc3339(&user_data.expires_at) {
		Ok(datetime) => datetime,
		Err(e) => {
			error!("{}", e);
			return
		}
	};
	let token_expiration = match OffsetDateTime::from_unix_timestamp(datetime.timestamp()) {
		Ok(token_expiration) => token_expiration,
		Err(e) => {
			error!("{}", e);
			return
		}
	};
	let jwt = Cookie::build(("jwt", user_data.token.clone()))
		.expires(token_expiration)
		.build();
	cookie_jar.add(jwt);
}
