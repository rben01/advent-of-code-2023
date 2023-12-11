// tag::setup[]
use crate::{read_file, Answer, AocError};
use ndarray::prelude::*;
use std::{collections::HashMap, str::FromStr};

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let input = read_input(input);
	println!("{input:?}");
	(10, (pt1(&input), pt2(&input))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Input {
	input.parse().unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
	N,
	S,
	E,
	W,
}

impl Direction {
	fn invert(self) -> Self {
		use Direction::*;
		match self {
			N => S,
			S => N,
			E => W,
			W => E,
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Pipe {
	/// Vertical
	Ns,
	/// Horizontal
	Ew,
	/// North-East
	Ne,
	/// North-West
	Nw,
	/// South-East
	Se,
	/// South-West
	Sw,
}

impl Pipe {
	fn directions(self) -> (Direction, Direction) {
		use Direction::*;
		use Pipe::*;
		match self {
			Ns => (N, S),
			Ew => (E, W),
			Ne => (N, E),
			Nw => (N, W),
			Se => (S, E),
			Sw => (S, W),
		}
	}

	fn next_dir(self, from_dir: Direction) -> Direction {
		let (d1, d2) = self.directions();
		if d1 == from_dir {
			d2
		} else if d2 == from_dir {
			d1
		} else {
			panic!("invalid pipe and direction: {self:?}, {from_dir:?}")
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Tile {
	Pipe(Pipe),
	Ground,
	Start,
}

impl TryFrom<char> for Tile {
	type Error = AocError;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		use Pipe::*;
		Ok(match c {
			'|' => Tile::Pipe(Ns),
			'-' => Tile::Pipe(Ew),
			'L' => Tile::Pipe(Ne),
			'J' => Tile::Pipe(Nw),
			'F' => Tile::Pipe(Se),
			'7' => Tile::Pipe(Sw),
			'.' => Tile::Ground,
			'S' => Tile::Start,
			_ => return Err(AocError::Other(format!("invalid char {c:?}"))),
		})
	}
}

#[derive(Debug)]
struct Input {
	map: Array2<Tile>,
	start: [usize; 2],
}

impl FromStr for Input {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use Direction::*;

		let mut height = 0;
		let mut start = None;

		let mut elems = Vec::new();

		for (ri, line) in s.lines().enumerate() {
			for (ci, c) in line.char_indices() {
				let tile = c.try_into()?;
				elems.push(tile);
				if matches!(tile, Tile::Start) {
					start = Some([ri, ci]);
				}
			}
			height += 1;
		}

		let width = elems.len() / height;
		let mut map = Array2::from_shape_vec((height, width), elems)?;
		let start = start.unwrap();

		let [ri, ci] = start;

		let mut incoming_directions = [N; 2];
		let mut inc_dir_idx = 0;

		let idx_err = || AocError::Other(format!("too many incoming direcions at {start:?}"));

		if ri > 0 {
			if let Tile::Pipe(pipe) = map[[ri - 1, ci]] {
				let (d1, d2) = pipe.directions();
				if d1 == S || d2 == S {
					*incoming_directions
						.get_mut(inc_dir_idx)
						.ok_or_else(idx_err)? = N;
					inc_dir_idx += 1;
				}
			}
		};

		if ri < height {
			if let Tile::Pipe(pipe) = map[[ri + 1, ci]] {
				let (d1, d2) = pipe.directions();
				if d1 == N || d2 == N {
					*incoming_directions
						.get_mut(inc_dir_idx)
						.ok_or_else(idx_err)? = S;
					inc_dir_idx += 1;
				}
			}
		};

		if ci > 0 {
			if let Tile::Pipe(pipe) = map[[ri, ci - 1]] {
				let (d1, d2) = pipe.directions();
				if d1 == E || d2 == E {
					*incoming_directions
						.get_mut(inc_dir_idx)
						.ok_or_else(idx_err)? = W;
					inc_dir_idx += 1;
				}
			}
		}

		if ci < width {
			if let Tile::Pipe(pipe) = map[[ri, ci + 1]] {
				let (d1, d2) = pipe.directions();
				if d1 == W || d2 == W {
					*incoming_directions
						.get_mut(inc_dir_idx)
						.ok_or_else(idx_err)? = E;
					inc_dir_idx += 1;
				}
			}
		};

		if inc_dir_idx < 2 {
			return Err(AocError::Other(format!(
				"only {inc_dir_idx} incoming pipes at {start:?}"
			)));
		}

		let starting_pipe = match incoming_directions {
			[N, S] | [S, N] => Pipe::Ns,
			[E, W] | [W, E] => Pipe::Ew,
			[N, E] | [E, N] => Pipe::Ne,
			[N, W] | [W, N] => Pipe::Nw,
			[S, E] | [E, S] => Pipe::Se,
			[S, W] | [W, S] => Pipe::Sw,
			_ => {
				return Err(AocError::Other(format!(
					"invalid incoming_directions {incoming_directions:?}"
				)))
			}
		};

		map[start] = Tile::Pipe(starting_pipe);

		Ok(Input { map, start })
	}
}

impl Input {
	// Todo: make this an iterator instead of a Vec
	fn traverse(&self) -> Vec<([usize; 2], Pipe)> {
		use Direction::*;

		let Input { map, start } = self;
		let start = *start;
		let [mut ri, mut ci] = start;

		let (starting_pipe, mut move_dir) = match map[start] {
			Tile::Pipe(pipe) => (pipe, pipe.directions().0),
			Tile::Ground => unreachable!("attempting to start at 'ground' tile"),
			Tile::Start => unreachable!("did not succesffully remove 'start' tile"),
		};

		let mut points = vec![(start, starting_pipe)];

		loop {
			(ri, ci) = match move_dir {
				N => (ri - 1, ci),
				S => (ri + 1, ci),
				E => (ri, ci + 1),
				W => (ri, ci - 1),
			};

			let prev_dir = move_dir.invert();

			let pipe;
			(pipe, move_dir) = match map[[ri, ci]] {
				Tile::Pipe(pipe) => {
					let (d1, d2) = pipe.directions();
					let dir = if prev_dir == d1 {
						d2
					} else if prev_dir == d2 {
						d1
					} else {
						panic!("invalid move_dir, prev_dir, directions: {move_dir:?}, {prev_dir:?}, {:?}", (d1, d2));
					};
					(pipe, dir)
				}
				Tile::Ground => unreachable!("pipes led to a ground tile"),
				Tile::Start => {
					unreachable!("did not succesffully remove 'start' tile, and pipes led there")
				}
			};

			if [ri, ci] == start {
				break;
			}

			points.push(([ri, ci], pipe));
		}

		points
	}
}
// end::setup[]

// tag::pt1[]
fn pt1(input: &Input) -> usize {
	input.traverse().len() / 2
}
// end::pt1[]

// tag::pt2[]
fn pt2(input: &Input) -> usize {
	let map = &input.map;
	let mut n_interior_points = 0;

	let path_points = input.traverse().into_iter().collect::<HashMap<_, _>>();

	let mut bend_direction = Direction::E;

	for ((ri, ci), _) in map.indexed_iter() {
		if path_points.contains_key(&[ri, ci]) {
			continue;
		}

		// check number of crossings between point and (perpendicular) pipes from point to
		// exterior of map. if parity is odd, point is inside (enclosed by pipes) \
		// note that `x ^= true` is equivalent to `x = !x`, ie `toggle` (which doesn't
		// exist)
		let mut odd_parity = false;

		for i in ri..map.nrows() {
			if let Some(pipe) = path_points.get(&[i, ci]) {
				match pipe {
					Pipe::Ns => {}
					Pipe::Ew => odd_parity ^= true,
					// The N* pipes must be preceded at some point in time by a S* pipe
					Pipe::Ne => {
						if bend_direction == Direction::W {
							odd_parity ^= true;
						}
					}
					Pipe::Nw => {
						if bend_direction == Direction::E {
							odd_parity ^= true;
						}
					}
					// Set the direction of the preceding bend
					Pipe::Se => bend_direction = Direction::E,
					Pipe::Sw => bend_direction = Direction::W,
				}
			}
		}

		if odd_parity {
			n_interior_points += 1;
		}
	}

	n_interior_points
}
// end::pt2[]

#[cfg(test)]
mod test {
	#![allow(unused_imports)]

	use super::*;
	use crate::{run_test, run_tests};

	#[test]
	fn sample() {
		run_test(&read_input(&read_file!("sample_input_1.txt")), (pt1, 4));
		run_test(&read_input(&read_file!("sample_input_2.txt")), (pt1, 8));
		run_test(&read_input(&read_file!("sample_input_3.txt")), (pt2, 8));
		run_test(&read_input(&read_file!("sample_input_4.txt")), (pt2, 10));
	}

	#[test]
	fn test() {
		run_tests(
			&read_input(&read_file!("input.txt")),
			(pt1, 7145),
			(pt2, 445),
		);
	}
}
