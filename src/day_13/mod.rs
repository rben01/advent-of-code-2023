// tag::setup[]
use crate::{error::AocResult, read_file, Answer, AocError};
use ndarray::{prelude::*, Zip};
use std::str::FromStr;

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let grids = read_input(input);
	(13, (pt1(&grids), pt2(&grids))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Vec<Grid> {
	input
		.split("\n\n")
		.map(Grid::from_str)
		.collect::<AocResult<_>>()
		.unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
	Ash,
	Rock,
}

struct Grid(Array2<Tile>);

impl FromStr for Grid {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut elems = Vec::new();
		let mut height = 0;
		for line in s.lines() {
			for c in line.chars() {
				let tile = match c {
					'.' => Tile::Ash,
					'#' => Tile::Rock,
					_ => return Err(AocError::Other(format!("invalid char {c:?}"))),
				};
				elems.push(tile);
			}
			height += 1;
		}

		let width = elems.len() / height;
		Ok(Self(Array2::from_shape_vec((height, width), elems)?))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
	Horizontal,
	Vertical,
}

impl Grid {
	fn grid(&self) -> ArrayView2<'_, Tile> {
		self.0.view()
	}

	fn check_symmetry(&self, axis: Axis, index: usize, smudged: bool) -> Result<(), ()> {
		let grid = self.grid();

		// we will right this wrong immediately, but it makes for cleaner code
		let mut lower = index + 1;
		let mut upper = index;

		if lower >= grid.len_of(axis) {
			return Err(());
		}

		let mut did_change_one = false;

		while lower > 0 && upper < grid.len_of(axis) - 1 {
			lower -= 1;
			upper += 1;

			let n_diff = Zip::from(grid.index_axis(axis, lower))
				.and(grid.index_axis(axis, upper))
				.fold(0, |acc, x, y| acc + usize::from(x != y));

			match n_diff {
				0 => {}
				1 if smudged => {
					if did_change_one {
						return Err(());
					}
					did_change_one = true;
				}
				1.. => return Err(()),
			}
		}

		if smudged && !did_change_one {
			Err(())
		} else {
			Ok(())
		}
	}

	fn find_symmetry(&self, dir: Direction, smudged: bool) -> Option<(Direction, usize)> {
		let axis = match dir {
			Direction::Horizontal => Axis(0),
			Direction::Vertical => Axis(1),
		};
		(0..self.grid().len_of(axis)).find_map(|i| {
			self.check_symmetry(axis, i, smudged)
				.map(|()| (dir, i + 1))
				.ok()
		})
	}

	fn axis_of_symmetry(&self, smudged: bool) -> (Direction, usize) {
		self.find_symmetry(Direction::Horizontal, smudged)
			.or_else(|| self.find_symmetry(Direction::Vertical, smudged))
			.unwrap()
	}
}

fn symmetry_num(grids: &[Grid], smudged: bool) -> usize {
	grids
		.iter()
		.map(|g| {
			let (axis, i) = g.axis_of_symmetry(smudged);
			match axis {
				Direction::Horizontal => i * 100,
				Direction::Vertical => i,
			}
		})
		.sum()
}
// end::setup[]

// tag::pt1[]
fn pt1(grids: &[Grid]) -> usize {
	symmetry_num(grids, false)
}
// end::pt1[]

// tag::pt2[]
fn pt2(grids: &[Grid]) -> usize {
	symmetry_num(grids, true)
}
// end::pt2[]

#[cfg(test)]
mod test {
	#![allow(unused_imports)]

	use super::*;
	use crate::{run_test, run_tests};

	#[test]
	fn sample() {
		run_tests(
			&*read_input(&read_file!("sample_input.txt")),
			(pt1, 405),
			(pt2, 400),
		);
	}

	#[test]
	fn test() {
		run_tests(
			&*read_input(&read_file!("input.txt")),
			(pt1, 31877),
			(pt2, 42996),
		);
	}
}
