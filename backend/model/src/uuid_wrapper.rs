#[cfg(feature = "api")]
use rocket::{
	form::{FromFormField, ValueField},
	serde::Deserialize,
	request::FromParam,
};
use uuid::Uuid;
use log::*;

pub struct UuidWrapper(Result<Uuid, String>);

impl UuidWrapper {
	pub fn get(self) -> Result<Uuid, String> {
		debug!("uuid get: {:?}", self.0);
		self.0
	}
}

#[cfg(feature =	"api")]
impl TryFrom<&str> for UuidWrapper {
	type Error = String;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		debug!("uuid from str: {}", value);
		Ok(match Uuid::parse_str(value) {
			Ok(uuid) => UuidWrapper(Ok(uuid)),
			Err(_) => UuidWrapper(Err("invalid uuid".to_string()))
		})
	}
}

#[cfg(feature = "api")]
impl<'v> FromFormField<'v> for UuidWrapper {
	fn from_value(field: ValueField<'v>) -> rocket::form::Result<'v, Self> {
		debug!("uuid from field: {}", field.value);
		Ok(match Uuid::parse_str(field.value) {
			Ok(uuid) => UuidWrapper(Ok(uuid)),
			Err(_) => UuidWrapper(Err("invalid uuid".to_string()))
		})
	}
}

#[cfg(feature = "api")]
impl<'r> FromParam<'r> for UuidWrapper {
	type Error = &'r str;

	fn from_param(param: &'r str) -> Result<Self, Self::Error> {
		debug!("uuid from param: {}", param);
		Ok(match Uuid::parse_str(param) {
			Ok(uuid) => UuidWrapper(Ok(uuid)),
			Err(_) => UuidWrapper(Err("invalid uuid".to_string()))
		})
	}
}

#[cfg(feature = "api")]
impl<'de> Deserialize<'de> for UuidWrapper {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		debug!("uuid from deserializer");
		Ok(match Uuid::deserialize(deserializer) {
			Ok(uuid) => UuidWrapper(Ok(uuid)),
			Err(_) => UuidWrapper(Err("invalid uuid".to_string()))
		})
	}
}

#[cfg(feature = "api")]
impl<'se> serde::Serialize for UuidWrapper {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		debug!("uuid to serializer");
		match &self.0 {
			Ok(uuid) => uuid.serialize(serializer),
			Err(err) => Err(serde::ser::Error::custom(err)),
		}
	}
}

impl std::fmt::Debug for UuidWrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.0 {
			Ok(uuid) => write!(f, "{:?}", uuid),
			Err(err) => Err(serde::ser::Error::custom(err)),
		}
	}
}

impl std::fmt::Display for UuidWrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.0 {
			Ok(uuid) => write!(f, "{}", uuid),
			Err(err) => Err(serde::ser::Error::custom(err)),
		}
	}
}
