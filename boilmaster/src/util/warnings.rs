use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct Warnings<T> {
	value: T,
	warnings: Vec<String>,
}

impl<T> Warnings<T> {
	pub fn new(value: T) -> Self {
		Self {
			value,
			warnings: vec![],
		}
	}

	#[must_use]
	pub fn with_warning(mut self, warning: impl Into<String>) -> Self {
		self.warnings.push(warning.into());
		self
	}

	#[must_use]
	pub fn with_warnings(mut self, warnings: impl IntoIterator<Item = String>) -> Self {
		self.warnings.extend(warnings.into_iter());
		self
	}

	pub fn map<U, F>(self, function: F) -> Warnings<U>
	where
		F: FnOnce(T) -> U,
	{
		Warnings {
			value: function(self.value),
			warnings: self.warnings,
		}
	}

	pub fn and_then<U, F>(self, function: F) -> Warnings<U>
	where
		F: FnOnce(T) -> Warnings<U>,
	{
		function(self.value).with_warnings(self.warnings)
	}

	// Used primarily for tests at the moment but hey who knows
	#[allow(dead_code)]
	pub fn decompose(self) -> (T, Vec<String>) {
		(self.value, self.warnings)
	}
}

pub trait SoftDeserialize<'de>: Sized {
	fn deserialize<D>(deserializer: D) -> Result<Warnings<Self>, D::Error>
	where
		D: Deserializer<'de>;
}

impl<'de, T> Deserialize<'de> for Warnings<T>
where
	T: SoftDeserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		T::deserialize(deserializer)
	}
}
