fn main() -> Result<(), Box<dyn std::error::Error>> {
	#[cfg(feature = "auth")]
	tonic_build::compile_protos("../../proto/auth.proto")?;
	#[cfg(feature = "api")]
	tonic_build::configure()
		.type_attribute("AuthResponse", "#[derive(serde::Serialize)]")
		.compile_protos(&["../../proto/auth.proto"], &["../../proto"])
		.expect("Unable to compile protos");
	Ok(())
}
