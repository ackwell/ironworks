use std::{ops::Deref, sync::Arc};

#[derive(Debug)]
pub enum Borrowed<'a, T> {
	Reference(&'a T),
	Arc(Arc<T>),
}

impl<'a, T> From<&'a T> for Borrowed<'a, T> {
	fn from(value: &'a T) -> Self {
		Self::Reference(value)
	}
}

impl<T> From<Arc<T>> for Borrowed<'_, T> {
	fn from(value: Arc<T>) -> Self {
		Self::Arc(value)
	}
}

impl<T> Deref for Borrowed<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Reference(reference) => reference,
			Self::Arc(arc) => Deref::deref(arc),
		}
	}
}

impl<T> Clone for Borrowed<'_, T> {
	fn clone(&self) -> Self {
		match self {
			Self::Reference(reference) => Self::Reference(*reference),
			Self::Arc(arc) => Self::Arc(arc.clone()),
		}
	}
}
