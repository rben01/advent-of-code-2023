// tag::setup[]
use crate::{read_file, Answer, AocError};
use ndarray::prelude::*;
use std::{collections::HashSet, str::FromStr};

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let board = read_input(input);
	(16, (pt1(&board), pt2(&board))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Board {
	input.parse().unwrap()
}

#[derive(Debug, Clone, Copy)]
enum Tile {
	Empty,
	SplitterHorizontal,
	SplitterVertical,
	MirrorSlash,
	MirrorBackslash,
}

#[derive(Debug)]
struct Board(Array2<Tile>);

impl FromStr for Board {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut elems = Vec::new();
		let mut height = 0;

		for line in s.lines() {
			for c in line.chars() {
				let tile = match c {
					'.' => Tile::Empty,
					'-' => Tile::SplitterHorizontal,
					'|' => Tile::SplitterVertical,
					'/' => Tile::MirrorSlash,
					'\\' => Tile::MirrorBackslash,
					_ => return Err(AocError::Other(format!("invalid character {c:?}"))),
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
	N,
	S,
	E,
	W,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Photon {
	pos: [usize; 2],
	dir: Direction,
}
impl Board {
	fn n_energized(&self, start_pos: [usize; 2], start_dir: Direction) -> usize {
		use Direction::*;
		use Tile::*;

		let Self(board) = self;
		let (nr, nc) = board.dim();

		let mut photons = vec![Photon {
			pos: start_pos,
			dir: start_dir,
		}];

		let mut lit_tiles = Array2::from_shape_simple_fn(board.dim(), || false);

		let mut seen_photons = HashSet::new();

		// handle one photon in its entirety before moving onto the next
		while let Some(Photon { mut pos, mut dir }) = photons.pop() {
			loop {
				if !seen_photons.insert(Photon { pos, dir }) {
					break;
				}
				lit_tiles[pos] = true;
				let [ri, ci] = pos;
				match board[pos] {
					SplitterHorizontal if matches!(dir, N | S) => {
						// if we end up with two photons (we aren't against the edge of the
						// board), "this" one goes east; we save the westward bound for later.
						// otherwise "this" one goes in the open direction
						if ci == 0 {
							dir = E;
						} else if ci == nc - 1 {
							dir = W;
						} else {
							photons.push(Photon {
								pos: [ri, ci - 1],
								dir: W,
							});

							dir = E;
						}
					}
					SplitterVertical if matches!(dir, E | W) => {
						// same logic as above, except we keep the southbound photon and save
						// the northbound for later
						if ri == 0 {
							dir = S;
						} else if ri == nr - 1 {
							dir = N;
						} else {
							photons.push(Photon {
								pos: [ri - 1, ci],
								dir: N,
							});

							dir = S;
						}
					}
					MirrorSlash => {
						dir = match dir {
							N => E,
							E => N,
							S => W,
							W => S,
						}
					}
					MirrorBackslash => {
						dir = match dir {
							N => W,
							W => N,
							S => E,
							E => S,
						}
					}
					_ => {}
				}

				pos = match dir {
					N if ri > 0 => [ri - 1, ci],
					S if ri < nr - 1 => [ri + 1, ci],
					E if ci < nc - 1 => [ri, ci + 1],
					W if ci > 0 => [ri, ci - 1],
					_ => break,
				};
			}
		}

		lit_tiles.into_iter().filter(|&lit| lit).count()
	}
}
// end::setup[]

// tag::pt1[]
fn pt1(board: &Board) -> usize {
	board.n_energized([0, 0], Direction::E)
}
// end::pt1[]

// tag::pt2[]
fn pt2(board: &Board) -> usize {
	use Direction::*;
	let (nr, nc) = board.0.dim();

	(0..nr)
		.flat_map(|ri| [([ri, 0], E), ([ri, nc - 1], W)])
		.chain((0..nc).flat_map(|ci| [([0, ci], S), ([nr - 1, ci], N)]))
		.map(|(start_pos, start_dir)| board.n_energized(start_pos, start_dir))
		.max()
		.unwrap()
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
			&read_input(&read_file!("sample_input.txt")),
			(pt1, 46),
			(pt2, 51),
		);
	}

	#[test]
	fn test() {
		run_tests(
			&read_input(&read_file!("input.txt")),
			(pt1, 8125),
			(pt2, 8489),
		);
	}
}
