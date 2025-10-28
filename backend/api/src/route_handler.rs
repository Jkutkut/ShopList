pub struct RouteHandler {
	pub routes: Vec<(String, Vec<rocket::Route>)>,
	pub catchers: Vec<(String, Vec<rocket::Catcher>)>,
}

pub struct RouteHandlerBuilder {
	path: String,
	routes: Vec<rocket::Route>,
	catchers: Vec<rocket::Catcher>,
	childs: Vec<RouteHandlerBuilder>,
}

impl RouteHandlerBuilder {
	pub fn new(
		path: &str,
		routes: Vec<rocket::Route>,
		catchers: Vec<rocket::Catcher>,
		childs: Vec<RouteHandlerBuilder>,
	) -> Self {
		Self {
			path: path.to_string(),
			routes,
			catchers,
			childs,
		}
	}

	pub fn build(self) -> RouteHandler {
		let mut all_routes: Vec<(String, Vec<rocket::Route>)> = Vec::new();
		let mut all_catchers: Vec<(String, Vec<rocket::Catcher>)> = Vec::new();
		all_routes.push((self.path.clone(), self.routes));
		all_catchers.push((self.path.clone(), self.catchers));
		for child in self.childs {
			let child = child.build();
			for (key, routes) in child.routes {
				all_routes.push((Self::concat_path(&self.path, &key), routes));
			}
			for (key, catchers) in child.catchers {
				all_catchers.push((Self::concat_path(&self.path, &key), catchers));
			}
		}
		RouteHandler {
			routes: all_routes,
			catchers: all_catchers,
		}
	}

	fn concat_path(path: &str, sub_path: &str) -> String {
		// TODO improve logic to prevent invalid values
		format!("{}{}", path, sub_path)
	}
}
