// tag::setup[]
use crate::{read_file, Answer, AocError, Cast};
use ndarray::prelude::*;
use num::range_step_inclusive;
use std::{collections::HashMap, str::FromStr};

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let mut grid = read_input(input);
	(14, (pt1(&mut grid.clone()), pt2(&mut grid))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Grid {
	input.parse().unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
	Empty,
	Round,
	Square,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
	N,
	S,
	E,
	W,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid(Array2<Tile>);

impl FromStr for Grid {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut elems = Vec::new();
		let mut height = 0;
		for line in s.lines() {
			for c in line.chars() {
				let tile = match c {
					'.' => Tile::Empty,
					'O' => Tile::Round,
					'#' => Tile::Square,
					_ => return Err(AocError::Other(format!("invalid char {c:?}"))),
				};
				elems.push(tile);
			}
			height += 1;
		}
		let width = elems.len() / height;
		Ok(Grid(Array2::from_shape_vec((height, width), elems)?))
	}
}

impl Grid {
	fn tilt(&mut self, dir: Direction) {
		#![allow(clippy::similar_names)]

		use Direction::*;

		let Self(grid) = self;
		let (nr, nc) = grid.dim();

		if nr < 2 || nc < 2 {
			return;
		}

		// inclusive
		let (outer_iter_start, outer_iter_stop) = match dir {
			N => (1, nr.cast::<i32>() - 1),
			S => (nr.cast::<i32>() - 2, 0),
			E => (nc.cast::<i32>() - 2, 0),
			W => (0, nc.cast::<i32>() - 1),
		};
		let outer_iter_step = if outer_iter_start < outer_iter_stop {
			1
		} else {
			-1
		};

		let (inner_iter_lo, inner_iter_hi) = match dir {
			N | S => (0, nc),
			E | W => (0, nr),
		};

		for outer_idx in range_step_inclusive(outer_iter_start, outer_iter_stop, outer_iter_step) {
			let outer_idx = outer_idx.cast();
			for inner_idx in inner_iter_lo..inner_iter_hi {
				let orig_idx = match dir {
					N | S => [outer_idx, inner_idx],
					E | W => [inner_idx, outer_idx],
				};

				if grid[orig_idx] != Tile::Round {
					continue;
				}

				let [mut ri, mut ci] = orig_idx;

				while match dir {
					N => ri > 0,
					S => ri < nr - 1,
					E => ci < nc - 1,
					W => ci > 0,
				} && grid[[ri, ci]] == Tile::Empty
				{
					match dir {
						N => ri -= 1,
						S => ri += 1,
						E => ci += 1,
						W => ci -= 1,
					}
				}

				grid.swap(orig_idx, [ri, ci]);
			}
		}
	}

	fn load(&self, dir: Direction) -> usize {
		use Direction::*;

		let Self(grid) = self;
		let (nr, nc) = grid.dim();

		let get_weight = |(ri, ci): (usize, usize)| match dir {
			N => nr - ri,
			S => ri + 1,
			W => nc - ci,
			E => ci + 1,
		};

		grid.indexed_iter()
			.filter(|&(_, &tile)| (tile == Tile::Round))
			.map(|(idx, _)| get_weight(idx))
			.sum::<usize>()
	}
}
// end::setup[]

// tag::pt1[]
fn pt1(grid: &mut Grid) -> usize {
	grid.tilt(Direction::N);
	grid.load(Direction::N)
}
// end::pt1[]

// tag::pt2[]
fn pt2(grid: &mut Grid) -> usize {
	use Direction::*;

	let mut seen_grid_idxs = HashMap::new();

	let mut i = 0_usize;
	let n_cycles = 1_000_000_000;
	let mut found_loop = false;

	while i < n_cycles {
		match seen_grid_idxs.get(&grid.0) {
			Some(&loop_start_idx) => {
				if !found_loop {
					found_loop = true;
					let cycle_len = i - loop_start_idx;
					let remaining_rounds = n_cycles - i;
					let remainder = remaining_rounds % cycle_len;
					// fast forward to end
					i = n_cycles - remainder;
				}
			}
			None => {
				seen_grid_idxs.insert(grid.0.clone(), i);
			}
		}

		for dir in [N, W, S, E] {
			grid.tilt(dir);
		}

		i += 1;
	}
	grid.load(N)
}
// end::pt2[]

#[cfg(test)]
mod test {
	#![allow(unused_imports)]

	use super::*;
	use crate::{run_test, run_tests};

	#[test]
	fn sample() {
		let mut grid = read_input(&read_file!("sample_input.txt"));
		run_test(&mut grid.clone(), (pt1, 136));
		run_test(&mut grid, (pt2, 64));
	}

	#[test]
	fn test() {
		let mut input = read_input(&read_file!("input.txt"));
		run_test(&mut input.clone(), (pt1, 110_779));
		run_test(&mut input, (pt2, 86069));
	}
}
