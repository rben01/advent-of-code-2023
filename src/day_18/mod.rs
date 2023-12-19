// tag::setup[]
use crate::{
	error::{AocResult, ToResultDefaultErr},
	read_file, regex, Answer, AocError,
};
use std::str::FromStr;
use strum_macros::EnumString;

fn ans_for_input(input: &str) -> Answer<i64, i64> {
	let instrs = read_input(input);
	(18, (pt1(&instrs), pt2(&instrs))).into()
}

pub fn ans() -> Answer<i64, i64> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Vec<CombinedInstr> {
	input
		.lines()
		.map(|line| line.parse())
		.collect::<AocResult<_>>()
		.unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
enum Direction {
	U,
	D,
	L,
	R,
}

#[derive(Debug, Clone, Copy)]
struct CombinedInstr {
	pt1_direction: Direction,
	pt1_dist: i64,
	pt2_direction: Direction,
	pt2_dist: i64,
}

impl FromStr for CombinedInstr {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let m = regex!(r"(?<dir>\w+)\s+(?<dist>\d+)\s+\(#(?<color_dist>\w{5})(?<color_dir>\w)\)")
			.captures(s)
			.to_result()?;
		let pt1_direction = m.name("dir").unwrap().as_str().parse()?;
		let pt1_dist = m.name("dist").unwrap().as_str().parse()?;

		let pt2_dist = i64::from_str_radix(m.name("color_dist").unwrap().as_str(), 16)?;
		let pt2_direction = match m.name("color_dir").unwrap().as_str() {
			"0" => Direction::R,
			"1" => Direction::D,
			"2" => Direction::L,
			"3" => Direction::U,
			c => return Err(AocError::Other(format!("invalid char {c:?}"))),
		};

		Ok(Self {
			pt1_direction,
			pt1_dist,
			pt2_direction,
			pt2_dist,
		})
	}
}

#[derive(Debug, Clone, Copy)]
struct Instr {
	direction: Direction,
	dist: i64,
}

impl Instr {
	fn from_pt1(
		CombinedInstr {
			pt1_direction: direction,
			pt1_dist: dist,
			..
		}: CombinedInstr,
	) -> Self {
		Self { direction, dist }
	}
	fn from_pt2(
		CombinedInstr {
			pt2_direction: direction,
			pt2_dist: dist,
			..
		}: CombinedInstr,
	) -> Self {
		Self { direction, dist }
	}
}

fn get_n_interior_points(instrs: &[Instr]) -> i64 {
	use Direction::*;

	// Strategy: use the <https://en.wikipedia.org/wiki/Shoelace_formula> \
	// Minor issue: we don't have the vertices defining the edges that surround a
	// polygon; rather, we have the centers of tiles; the outsides of those tiles are
	// what surround the polygon. Since the tiles' centers lie in the center of the
	// squares, we undercount by 0.5 per edge tile. We also have the corners to consider;
	// since we go around one full time, that's another full tile of area that the
	// shoelace theorem will miss. So at the end, we add `edge_len / 2 + 1` to the
	// naive shoelace answer of `twice_area / 2`

	let p0 = [0, 0]; // arbitrary point
	let mut pos = p0;
	let mut twice_area = 0;
	let mut edge_len = 0;

	for &Instr {
		direction, dist, ..
	} in instrs
	{
		let [x1, y1] = pos;

		let [x2, y2] = match direction {
			U => [x1, y1 - dist],
			D => [x1, y1 + dist],
			L => [x1 - dist, y1],
			R => [x1 + dist, y1],
		};

		// shoelace theorem: we add det([[x1, x2], [y1, y2]])
		twice_area += x1 * y2 - x2 * y1;
		edge_len += dist;

		pos = [x2, y2];
	}

	twice_area.abs() / 2 + edge_len / 2 + 1
}
// end::setup[]

// tag::pt1[]
fn pt1(instrs: &[CombinedInstr]) -> i64 {
	let instrs = instrs
		.iter()
		.copied()
		.map(Instr::from_pt1)
		.collect::<Vec<_>>();
	get_n_interior_points(&instrs)
}
// end::pt1[]

// tag::pt2[]
fn pt2(instrs: &[CombinedInstr]) -> i64 {
	let instrs = instrs
		.iter()
		.copied()
		.map(Instr::from_pt2)
		.collect::<Vec<_>>();
	get_n_interior_points(&instrs)
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
			(pt1, 62),
			(pt2, 952_408_144_115),
		);
	}

	#[test]
	fn test() {
		run_tests(
			&*read_input(&read_file!("input.txt")),
			(pt1, 95356),
			(pt2, 92_291_468_914_147),
		);
	}
}
