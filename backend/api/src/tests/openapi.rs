use super::*;
use std::fmt::Display;
use std::collections::HashMap;
use rocket::{
	Route,
};

struct Endpoint {
	method: String,
	path: String,
}

impl Display for Endpoint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:6} {}", self.method, self.path)
	}
}

impl From<&Route> for Endpoint {
	fn from(route: &Route) -> Self {
		Self {
			method: route.method.to_string(),
			path: route.uri.path().to_string(),
		}
	}
}

struct EndpointValidator {
	endpoint: Endpoint,
	is_in_api: bool,
	is_in_openapi: bool,
}

impl EndpointValidator {
	fn new_api(endpoint: Endpoint) -> Self {
		Self {
			endpoint,
			is_in_api: true,
			is_in_openapi: false,
		}
	}

	fn new_openapi(endpoint: Endpoint) -> Self {
		Self {
			endpoint,
			is_in_api: false,
			is_in_openapi: true,
		}
	}

	fn is_valid(&self) -> bool {
		self.is_in_api == self.is_in_openapi && self.is_in_api == true
	}
}

impl Display for EndpointValidator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.is_valid() {
			true  => write!(f, "✅ 🖥️ 📖 {}", self.endpoint),
			false => write!(
				f, "❌ {} {} {}",
				if self.is_in_api { "🖥️" } else { "  " },
				if self.is_in_openapi { "📖" } else { "  " },
				self.endpoint,
			),
		}
	}
}

struct ApiValidator {
	endpoints: HashMap<String, EndpointValidator>,
}

impl ApiValidator {
	fn new() -> Self {
		Self {
			endpoints: HashMap::new(),
		}
	}

	fn add_api(&mut self, endpoint: Endpoint) {
		let key = endpoint.to_string();
		if self.endpoints.contains_key(&key) {
			let endpoint_validator = self.endpoints.get_mut(&key).unwrap();
			endpoint_validator.is_in_api = true;
			return;
		}
		self.endpoints.insert(endpoint.to_string(), EndpointValidator::new_api(endpoint));
	}

	fn add_openapi(&mut self, endpoint: Endpoint) {
		let key = endpoint.to_string();
		if self.endpoints.contains_key(&key) {
			let endpoint_validator = self.endpoints.get_mut(&key).unwrap();
			endpoint_validator.is_in_openapi = true;
			return;
		}
		self.endpoints.insert(endpoint.to_string(), EndpointValidator::new_openapi(endpoint));
	}

	fn validate(&self) -> bool {
		let mut is_valid = true;
		let mut valids: usize = 0;
		let mut invalids: usize = 0;
		debug!("Validating {} endpoints", self.endpoints.len());
		debug!(" - usage: api = 🖥️, openapi = 📖");
		let mut values = self.endpoints.values().collect::<Vec<_>>();
		values.sort_by(|a, b| {
			let key_a = format!("{}{}", &a.endpoint.path, &a.endpoint.method);
			let key_b = format!("{}{}", &b.endpoint.path, &b.endpoint.method);
			key_a.cmp(&key_b)
		});
		for endpoint_validator in values {
			debug!("{}", endpoint_validator);
			match endpoint_validator.is_valid() {
				true  => valids += 1,
				false => {
					invalids += 1;
					is_valid = false;
				}
			}
		}
		debug!(" - {} valid, {} invalid", valids, invalids);
		is_valid
	}
}

#[tokio::test]
async fn openapi() {
	let openapi: Result<serde_yaml::Value, _> = serde_yaml::from_str(include_str!("/shoplist/api/openapi.yaml"));
	assert!(openapi.is_ok(), "Failed to find / parse openapi.yaml");
	let openapi = openapi.unwrap();
	let paths = openapi["paths"].as_mapping().unwrap();
	let rocket = rocket().await;
	let mut validator = ApiValidator::new();

	for route in rocket.routes() {
		validator.add_api(Endpoint::from(route));
	}
	for path in paths.keys() {
		let path_str = path.as_str().unwrap()
			.replace("{", "<")
			.replace("}", ">");
		let methods = paths[path].as_mapping().unwrap();
		for method in methods.keys() {
			let method_str = method.as_str().unwrap().to_uppercase();
			let endpoint = Endpoint {
				method: method_str,
				path: path_str.clone(),
			};
			validator.add_openapi(endpoint);
		}
	}
	assert!(validator.validate());
}
