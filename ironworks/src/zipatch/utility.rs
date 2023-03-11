use std::{
	collections::HashMap,
	hash::Hash,
	ops::{Deref, DerefMut},
};

use binrw::{binrw, BinRead, BinWrite};

// Traits literally just to clean up the code a bit.
pub trait BinReadWrite: BinRead<Args = ()> + BinWrite<Args = ()> {}
impl<T> BinReadWrite for T where T: BinRead<Args = ()> + BinWrite<Args = ()> {}

pub trait BrwMapKey: Clone + Eq + Hash + BinReadWrite {}
impl<T> BrwMapKey for T where T: Clone + Eq + Hash + BinReadWrite {}

pub trait BrwMapValue: Clone + BinReadWrite {}
impl<T> BrwMapValue for T where T: Clone + BinReadWrite {}

/// HashMap wrapper that can be serialized to a binary format represented as a count and array of items.
#[binrw]
#[derive(Debug)]
pub struct BrwMap<K: BrwMapKey, V: BrwMapValue> {
	#[br(temp)]
	#[bw(calc = map.len().try_into().unwrap())]
	count: u32,

	#[br(
		count = count,
		map = |value: Vec<(K, V)>| value.into_iter().collect()
	)]
	#[bw(
		map = |value| value.clone().into_iter().collect::<Vec<(K, V)>>()
	)]
	map: HashMap<K, V>,
}

impl<K: BrwMapKey, V: BrwMapValue> Default for BrwMap<K, V> {
	fn default() -> Self {
		Self {
			map: Default::default(),
		}
	}
}

impl<K: BrwMapKey, V: BrwMapValue> Deref for BrwMap<K, V> {
	type Target = HashMap<K, V>;

	fn deref(&self) -> &Self::Target {
		&self.map
	}
}

impl<K: BrwMapKey, V: BrwMapValue> DerefMut for BrwMap<K, V> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.map
	}
}

/// Vec wrapper that can be serialized to a binary format represented as a count and array of items.
#[binrw]
#[derive(Debug, Clone)]
pub struct BrwVec<T: BinReadWrite> {
	#[br(temp)]
	#[bw(calc = vec.len().try_into().unwrap())]
	count: u32,

	#[br(count = count)]
	vec: Vec<T>,
}

impl<T: BinReadWrite> Default for BrwVec<T> {
	fn default() -> Self {
		Self {
			vec: Default::default(),
		}
	}
}

impl<T: BinReadWrite> Deref for BrwVec<T> {
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		&self.vec
	}
}

impl<T: BinReadWrite> DerefMut for BrwVec<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.vec
	}
}

impl<T: BinReadWrite> FromIterator<T> for BrwVec<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		Self {
			vec: Vec::from_iter(iter),
		}
	}
}
