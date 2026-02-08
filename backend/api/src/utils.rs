pub fn env_var(var: &str) -> Result<String, String> {
	Ok(std::env::var(var).map_err(|_| {
		format!("{} not defined as environment variable or in .env file", var)
	})?)
}

pub fn is_valid_team_name(s: &str) -> bool {
	let len = s.len();
	if len < 4 || len > 50 {
		return false;
	}
	s.chars().all(|c| {
		c.is_ascii_alphanumeric() || c == '_' || c == '-'
	})
}
