fn main() -> Result<(), Box<dyn std::error::Error>> {
	#[cfg(feature = "auth")]
	tonic_build::compile_protos("../../proto/auth.proto")?;
	#[cfg(feature = "api")]
	tonic_build::configure()
		.type_attribute("UserToken", "#[derive(serde::Serialize, serde::Deserialize)]")
		.type_attribute("User", "#[derive(serde::Deserialize)]")
		.type_attribute("UserTeamRoles", "#[derive(serde::Serialize, serde::Deserialize)]")
		.type_attribute("TeamRole", "#[derive(serde::Serialize, serde::Deserialize)]")
		.type_attribute("Team", "#[derive(serde::Serialize, serde::Deserialize)]")
		.compile_protos(&["../../proto/auth.proto"], &["../../proto"])
		.expect("Unable to compile protos");
	Ok(())
}
