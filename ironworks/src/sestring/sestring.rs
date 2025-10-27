use std::{
	borrow::Borrow,
	collections::TryReserveError,
	fmt::{self},
	hash::{Hash, Hasher},
	ops::{Deref, DerefMut, Index, IndexMut, RangeFull},
};

use crate::sestring::sestr::SeStr;

use super::error::Result;

/// Square Enix rich text format.
///
/// SeString data consistes of a combination of UTF8 text and "macros" that
/// perform operations ranging from text style and colour, to control flow and
/// data lookups. Individual sections of an SeString are represented by
/// [`Payload`]s.
///
/// This implementation does not eagerly parse the inner structures of the
/// string, as such it may represent an invalid state until queried further.
#[repr(transparent)]
pub struct SeString(pub(super) Vec<u8>);

impl SeString {
	/// Constructs a new empty `SeString`.
	pub const fn new() -> Self {
		Self(Vec::new())
	}

	/// Converts to an [`SeStr`] slice.
	#[must_use]
	#[inline]
	pub fn as_se_str(&self) -> &SeStr {
		self
	}

	/// Creates a new `SeString` with at least the given capacity.
	pub fn with_capacity(capacity: usize) -> Self {
		Self(Vec::with_capacity(capacity))
	}

	/// Truncates the `SeString` to zero length.
	#[inline]
	pub fn clear(&mut self) {
		self.0.clear()
	}

	/// Returns the capacity this `SeString` can hold without reallocating.
	#[must_use]
	#[inline]
	pub fn capacity(&self) -> usize {
		self.0.capacity()
	}

	/// Reserves capacity for at least `additional` more capacity to be inserted
	/// in the given `SeString`. Does nothing if the capacity is
	/// already sufficient.
	///
	/// The collection may reserve more space to speculatively avoid frequent reallocations.
	#[inline]
	pub fn reserve(&mut self, additional: usize) {
		self.0.reserve(additional)
	}

	/// Tries to reserve capacity for at least `additional` more length units
	/// in the given `SeString`. The string may reserve more space to speculatively avoid
	/// frequent reallocations. After calling `try_reserve`, capacity will be
	/// greater than or equal to `self.len() + additional` if it returns `Ok(())`.
	/// Does nothing if capacity is already sufficient. This method preserves
	/// the contents even if an error occurs.
	#[inline]
	pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
		self.0.try_reserve(additional)
	}

	/// Reserves the minimum capacity for at least `additional` more capacity to
	/// be inserted in the given `SeString`. Does nothing if the capacity is
	/// already sufficient.
	///
	/// Note that the allocator may give the collection more space than it
	/// requests. Therefore, capacity can not be relied upon to be precisely
	/// minimal. Prefer [`reserve`] if future insertions are expected.
	///
	/// [`reserve`]: SeString::reserve
	#[inline]
	pub fn reserve_exact(&mut self, additional: usize) {
		self.0.reserve_exact(additional)
	}

	/// Tries to reserve the minimum capacity for at least `additional`
	/// more length units in the given `SeString`. After calling
	/// `try_reserve_exact`, capacity will be greater than or equal to
	/// `self.len() + additional` if it returns `Ok(())`.
	/// Does nothing if the capacity is already sufficient.
	///
	/// Note that the allocator may give the `SeString` more space than it
	/// requests. Therefore, capacity can not be relied upon to be precisely
	/// minimal. Prefer [`try_reserve`] if future insertions are expected.
	///
	/// [`try_reserve`]: SeString::try_reserve
	#[inline]
	pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
		self.0.try_reserve_exact(additional)
	}

	/// Shrinks the capacity of the `SeString` to match its length.
	#[inline]
	pub fn shrink_to_fit(&mut self) {
		self.0.shrink_to_fit()
	}

	/// Shrinks the capacity of the `SeString` with a lower bound.
	///
	/// The capacity will remain at least as large as both the length
	/// and the supplied value.
	///
	/// If the current capacity is less than the lower limit, this is a no-op.
	#[inline]
	pub fn shrink_to(&mut self, min_capacity: usize) {
		self.0.shrink_to(min_capacity)
	}

	/// Converts this `SeString` into a boxed [`SeStr`].
	#[must_use = "`self` will be dropped if the result is not used"]
	pub fn into_boxed_se_str(self) -> Box<SeStr> {
		let rw = Box::into_raw(self.0.into_boxed_slice()) as *mut SeStr;
		unsafe { Box::from_raw(rw) }
	}

	/// Consumes and leaks the `SeString`, returning a mutable reference to the contents,
	/// `&'a mut SeStr`.
	///
	/// The caller has free choice over the returned lifetime, including 'static.
	/// Indeed, this function is ideally used for data that lives for the remainder of
	/// the program’s life, as dropping the returned reference will cause a memory leak.
	///
	/// It does not reallocate or shrink the `SeString`, so the leaked allocation may include
	/// unused capacity that is not part of the returned slice. If you want to discard excess
	/// capacity, call [`into_boxed_os_str`], and then [`Box::leak`] instead.
	/// However, keep in mind that trimming the capacity may result in a reallocation and copy.
	///
	/// [`into_boxed_os_str`]: Self::into_boxed_os_str
	#[inline]
	pub fn leak<'a>(self) -> &'a mut SeStr {
		SeStr::from_inner_mut(self.0.leak())
	}

	/// Truncate the `SeString` to the specified length.
	///
	/// If `new_len` is greater than or equal to the string's current length, this has no
	/// effect.
	///
	/// Note that this method has no effect on the allocated capacity
	/// of the string. Unlike String and OsString, this method does not panic if `new_len` does not lie on a
	/// character boundary.
	#[inline]
	#[track_caller]
	pub fn truncate(&mut self, new_len: usize) {
		if new_len <= self.len() {
			self.0.truncate(new_len)
		}
	}

	/// Consumes the `SeString`, returning the underlying `Vec<u8>`.
	#[inline]
	#[must_use]
	pub fn into_inner(self) -> Vec<u8> {
		self.0
	}
}

impl From<Vec<u8>> for SeString {
	/// Converts a [`Vec<u8>`] into an [`SeString`].
	///
	/// This conversion does not allocate or copy memory.
	#[inline]
	fn from(s: Vec<u8>) -> SeString {
		Self(s)
	}
}

impl<T: ?Sized + AsRef<SeStr>> From<&T> for SeString {
	/// Copies any value implementing <code>[AsRef]&lt;[SeStr]&gt;</code>
	/// into a newly allocated [`SeString`].
	fn from(s: &T) -> SeString {
		Self(s.as_ref().as_bytes().to_owned())
	}
}

impl Index<std::ops::RangeFull> for SeString {
	type Output = SeStr;

	#[inline]
	fn index(&self, _index: RangeFull) -> &SeStr {
		SeStr::from_inner(self.0.as_slice())
	}
}

impl IndexMut<RangeFull> for SeString {
	#[inline]
	fn index_mut(&mut self, _index: RangeFull) -> &mut SeStr {
		SeStr::from_inner_mut(self.0.as_mut_slice())
	}
}

impl Deref for SeString {
	type Target = SeStr;

	#[inline]
	fn deref(&self) -> &SeStr {
		&self[..]
	}
}

impl DerefMut for SeString {
	#[inline]
	fn deref_mut(&mut self) -> &mut SeStr {
		&mut self[..]
	}
}

impl Borrow<SeStr> for SeString {
	#[inline]
	fn borrow(&self) -> &SeStr {
		&self[..]
	}
}

impl Default for SeString {
	/// Constructs an empty `SeString`.
	#[inline]
	fn default() -> SeString {
		SeString::new()
	}
}

impl Clone for SeString {
	#[inline]
	fn clone(&self) -> Self {
		SeString(self.0.clone())
	}

	/// Clones the contents of `source` into `self`.
	///
	/// This method is preferred over simply assigning `source.clone()` to `self`,
	/// as it avoids reallocation if possible.
	#[inline]
	fn clone_from(&mut self, source: &Self) {
		self.0.clone_from(&source.0)
	}
}

impl fmt::Debug for SeString {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(&**self, formatter)
	}
}

impl Hash for SeString {
	#[inline]
	fn hash<H: Hasher>(&self, state: &mut H) {
		(&**self).hash(state)
	}
}
