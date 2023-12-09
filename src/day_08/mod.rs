// tag::setup[]
use crate::{
	enum_map::EnumMap, error::AocResult, read_file, regex, Answer, AocError, ToResultDefaultErr,
};
use num::Integer;
use std::{array, collections::HashMap, str::FromStr};
use strum::EnumCount;
use strum_macros::EnumCount;

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let input = read_input(input);
	(8, (pt1(&input), pt2(&input))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Input {
	Input::from_str(input).unwrap()
}

#[derive(Debug, Clone, Copy, EnumCount)]
#[repr(u8)]
enum Direction {
	L,
	R,
}

impl From<Direction> for usize {
	fn from(value: Direction) -> Self {
		value as _
	}
}

impl TryFrom<char> for Direction {
	type Error = AocError;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		use Direction::*;
		Ok(match c {
			'L' => L,
			'R' => R,
			_ => return Err(AocError::Other(format!("invalid direction {c:?}"))),
		})
	}
}

#[derive(Debug, Clone)]
struct DirectionIter {
	directions: Vec<Direction>,
	index: usize,
}

impl FromStr for DirectionIter {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let directions = s
			.chars()
			.map(Direction::try_from)
			.collect::<AocResult<_>>()?;
		Ok(Self {
			directions,
			index: 0,
		})
	}
}

impl Iterator for DirectionIter {
	type Item = Direction;

	fn next(&mut self) -> Option<Self::Item> {
		let i = &mut self.index;
		let ans = self.directions[*i];
		*i = (*i + 1) % self.directions.len();
		Some(ans)
	}
}

type Pair = EnumMap<{ Direction::COUNT }, Direction, String>;

#[derive(Debug, Clone)]
struct Input {
	directions: Vec<Direction>,
	nodes: HashMap<String, Pair>,
}

impl FromStr for Input {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut lines = s.lines();
		let directions = lines
			.next()
			.to_result()?
			.chars()
			.map(Direction::try_from)
			.collect::<AocResult<_>>()?;

		lines.next().unwrap();

		let nodes = lines
			.map(|line| {
				let mut words = regex!(r"\w+").find_iter(line);
				let [src, left, right] = array::try_from_fn(|_| {
					AocResult::Ok(words.next().to_result()?.as_str().to_owned())
				})?;
				Ok((src, Pair::new([left, right])))
			})
			.collect::<AocResult<_>>()?;

		Ok(Self { directions, nodes })
	}
}
// end::setup[]

// tag::pt1[]
fn pt1(input: &Input) -> usize {
	let Input { directions, nodes } = input;

	let mut loc = "AAA";
	let mut n_steps = 0;
	while loc != "ZZZ" {
		let pair = &nodes[loc];
		loc = &pair[directions[n_steps % directions.len()]];
		n_steps += 1;
	}
	n_steps
}
// end::pt1[]

// tag::pt2[]
fn pt2(input: &Input) -> usize {
	let Input { directions, nodes } = input;

	let mut cycle_lens = Vec::new();
	for starting_point in nodes.keys() {
		if !starting_point.ends_with('A') {
			continue;
		}

		let mut i = 0;
		let mut loc = starting_point;
		while !loc.ends_with('Z') {
			let direction = directions[i % directions.len()];
			loc = &nodes[loc][direction];
			i += 1;
		}
		cycle_lens.push(i);
	}

	cycle_lens.into_iter().fold(1, |acc, n| acc.lcm(&n))
}
// end::pt2[]

#[cfg(test)]
mod test {
	#![allow(unused_imports)]

	use super::*;
	use crate::{run_test, run_tests};

	#[test]
	fn sample() {
		run_test(&read_input(&read_file!("sample_input_1.txt")), (pt1, 2));
		run_test(&read_input(&read_file!("sample_input_2.txt")), (pt1, 6));
		run_test(&read_input(&read_file!("sample_input_3.txt")), (pt2, 6));
	}

	#[test]
	fn test() {
		run_tests(
			&read_input(&read_file!("input.txt")),
			(pt1, 19099),
			(pt2, 17_099_847_107_071),
		);
	}
}
