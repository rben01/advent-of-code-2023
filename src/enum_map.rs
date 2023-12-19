use std::{
	array,
	fmt::{self, Write},
	marker::PhantomData,
	ops::{Index, IndexMut},
};
use strum::IntoEnumIterator;

// We want to add `E: EnumCount` so we can replace `N` with `E::COUNT`, but this
// requires a highly unstable compiler feature, `generic_const_exprs`. So we're stuck
// with the somewhat redundant `N`
#[derive(Debug)]
pub(crate) struct EnumMap<const N: usize, E, T>([T; N], PhantomData<E>);

impl<const N: usize, E, T: Copy> Copy for EnumMap<N, E, T> {}
impl<const N: usize, E, T: Clone> Clone for EnumMap<N, E, T> {
	fn clone(&self) -> Self {
		Self(self.0.clone(), PhantomData)
	}
}

impl<const N: usize, E, T> EnumMap<N, E, T> {
	#[allow(dead_code)]
	pub(crate) fn new(arr: [T; N]) -> Self {
		Self(arr, PhantomData)
	}

	pub(crate) fn into_array(self) -> [T; N] {
		self.0
	}
}

impl<const N: usize, E, T: Default> Default for EnumMap<N, E, T> {
	fn default() -> Self {
		Self(array::from_fn(|_| T::default()), PhantomData)
	}
}

impl<const N: usize, E: Into<usize>, T> Index<E> for EnumMap<N, E, T> {
	type Output = T;

	fn index(&self, index: E) -> &Self::Output {
		&self.0[index.into()]
	}
}

impl<const N: usize, E: Into<usize>, T> IndexMut<E> for EnumMap<N, E, T> {
	fn index_mut(&mut self, index: E) -> &mut Self::Output {
		&mut self.0[index.into()]
	}
}

impl<const N: usize, E: IntoEnumIterator, T> IntoIterator for EnumMap<N, E, T> {
	type Item = (E, T);
	type IntoIter = std::iter::Zip<E::Iterator, <[T; N] as IntoIterator>::IntoIter>;

	fn into_iter(self) -> Self::IntoIter {
		E::iter().zip(self.0)
	}
}

impl<const N: usize, E: Into<usize>, T: Default> FromIterator<(E, T)> for EnumMap<N, E, T> {
	fn from_iter<I: IntoIterator<Item = (E, T)>>(iter: I) -> Self {
		let mut this = Self::default();
		for (idx, x) in iter {
			this[idx] = x;
		}
		this
	}
}

impl<const N: usize, E, T> fmt::Display for EnumMap<N, E, T>
where
	T: Copy + fmt::Display,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_char('[')?;
		let mut iter = (*self).into_array().into_iter();
		if let Some(x) = iter.next() {
			write!(f, "{x}")?;
		}
		for x in iter {
			write!(f, ", {x}")?;
		}
		f.write_char(']')?;
		Ok(())
	}
}
