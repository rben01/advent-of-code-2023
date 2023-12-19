use crate::error::{AocError, AocResult};
use ndarray::prelude::*;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Grid<T>(Array2<T>);

impl<T> FromStr for Grid<T>
where
	T: TryFrom<char>,
	AocError: From<<T as TryFrom<char>>::Error>,
{
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut elems = Vec::with_capacity(s.len());
		let mut height = 0;

		for line in s.lines() {
			for c in line.chars() {
				elems.push(c.try_into()?);
			}
			height += 1;
		}

		let width = elems.len() / height;

		let arr = Array2::from_shape_vec((height, width), elems)?;

		Ok(Self(arr))
	}
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.grid())
	}
}

impl<T> Grid<T> {
	pub(crate) fn from_str_chars(
		s: &str,
		char_to_t: impl Fn(char) -> AocResult<T>,
	) -> AocResult<Self> {
		let mut elems = Vec::with_capacity(s.len());
		let mut height = 0;

		for line in s.lines() {
			for c in line.chars() {
				elems.push(char_to_t(c)?);
			}
			height += 1;
		}

		let width = elems.len() / height;

		let arr = Array2::from_shape_vec((height, width), elems)?;

		Ok(Self(arr))
	}

	pub(crate) fn grid(&self) -> ArrayView2<T> {
		self.0.view()
	}

	pub(crate) fn dim(&self) -> [usize; 2] {
		self.grid().dim().into()
	}
}
