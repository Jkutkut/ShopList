pub fn env_var(var: &str) -> Result<String, String> {
	Ok(std::env::var(var).map_err(|_| {
		format!("{} not defined as environment variable or in .env file", var)
	})?)
}
