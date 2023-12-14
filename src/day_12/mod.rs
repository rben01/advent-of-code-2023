// tag::setup[]
use crate::{
	error::{AocResult, ToResultDefaultErr},
	read_file, Answer, AocError,
};
use ndarray::prelude::*;
use std::{
	collections::HashMap,
	fmt::{self, Write},
	str::FromStr,
};

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let springs = read_input(input);
	(12, (pt1(&springs), pt2(&springs))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Vec<Row> {
	input
		.lines()
		.map(|s| s.parse())
		.collect::<AocResult<_>>()
		.unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Spring {
	Operational,
	Damaged,
}

impl From<Spring> for usize {
	fn from(value: Spring) -> Self {
		value as _
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
	Known(Spring),
	Unknown,
}

impl TryFrom<char> for Tile {
	type Error = AocError;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		Ok(match c {
			'#' => Tile::Known(Spring::Damaged),
			'.' => Tile::Known(Spring::Operational),
			'?' => Tile::Unknown,
			_ => return Err(AocError::Other(format!("invalid char {c:?}"))),
		})
	}
}

#[derive(Debug)]
struct Row {
	tiles: Vec<Tile>,
	lengths: Vec<usize>,
}

impl FromStr for Row {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut words = s.split_whitespace();
		let tiles = words
			.next()
			.to_result()?
			.chars()
			.map(|c| c.try_into())
			.collect::<AocResult<_>>()?;
		let lengths = words
			.next()
			.to_result()?
			.split(',')
			.map(|s| Ok(s.parse()?))
			.collect::<AocResult<_>>()?;

		Ok(Self { tiles, lengths })
	}
}

fn count_solns<'a>(
	tiles: &'a [Tile],
	lengths: &'a [usize],
	valid_run_lens: ArrayView2<bool>,
	cache: &mut HashMap<(&'a [Tile], &'a [usize]), usize>,
) -> usize {
	if let Some(&count) = cache.get(&(tiles, lengths)) {
		return count;
	}

	if lengths.is_empty() {
		return usize::from(tiles.iter().all(|&t| t != Tile::Known(Spring::Damaged)));
	}

	// since `lengths` is not empty, `back` (on the line that assigns `front`, not on
	// the line that assigns `len`) has at least one elem
	let (front, back) = lengths.split_at(lengths.len() / 2);
	let (&len, back) = back.split_first().unwrap();

	if len > tiles.len() {
		return 0;
	}

	let mut count = 0;

	for i in 0..=(tiles.len() - len) {
		if valid_run_lens[[i, len]] {
			let front_count =
				if i == 0 {
					usize::from(front.is_empty())
				} else if tiles[i - 1] == Tile::Known(Spring::Damaged) {
					continue;
				} else {
					count_solns(
						&tiles[..i - 1],
						front,
						valid_run_lens.slice(s![..i - 1, ..]),
						cache,
					)
				};

			let back_count = if i == tiles.len() - len {
				usize::from(back.is_empty())
			} else if tiles[i + len] == Tile::Known(Spring::Damaged) {
				continue;
			} else {
				count_solns(
					&tiles[i + len + 1..],
					back,
					valid_run_lens.slice(s![i + len + 1.., ..]),
					cache,
				)
			};

			count += front_count * back_count;
		}
	}

	cache.insert((tiles, lengths), count);

	count
}

impl Row {
	fn count_solns<'a>(&'a self, cache: &mut HashMap<(&'a [Tile], &'a [usize]), usize>) -> usize {
		let Self { tiles, .. } = self;
		let mut tiles = tiles.clone();
		// Add a known operational tile to the end of the row. This won't affect the
		// answers, but does make our lives a lot easier because we know each run of
		// damaged springs will end before the last tile, which simplifies the logic quite
		// a bit.
		tiles.push(Tile::Known(Spring::Operational));

		// Matrix such that `mat[[r, c]]` means a run of damaged tiles of length c can
		// start at the r^th tile. (The first column, c == 0, is entirely false.)
		let mut valid_run_lens =
			Array2::from_shape_simple_fn((tiles.len(), tiles.len() + 1), || false);

		for i in 0..tiles.len() - 1 {
			// if the tile before is damaged, then this tile definitely isn't the start of
			// a run of damaged tiles; also not the start of a run if it's operational
			if i > 0 && tiles[i - 1] == Tile::Known(Spring::Damaged)
				|| tiles[i] == Tile::Known(Spring::Operational)
			{
				continue;
			}
			for j in i + 1..=tiles.len() {
				if tiles[j - 1] == Tile::Known(Spring::Operational) {
					break;
				}
				if tiles[j] != Tile::Known(Spring::Damaged) {
					valid_run_lens[[i, j - i]] = true;
				}
			}
		}

		count_solns(&self.tiles, &self.lengths, valid_run_lens.view(), cache)
	}
}

// end::setup[]

// tag::pt1[]
fn pt1(rows: &[Row]) -> usize {
	let mut cache = HashMap::new();
	rows.iter().map(|row| row.count_solns(&mut cache)).sum()
}
// end::pt1[]

// tag::pt2[]
fn pt2(rows: &[Row]) -> usize {
	let new_rows = rows
		.iter()
		.map(|Row { tiles, lengths }| {
			let mut new_row = Row {
				tiles: Vec::new(),
				lengths: Vec::new(),
			};
			for i in 0..5 {
				if i > 0 {
					new_row.tiles.push(Tile::Unknown);
				}
				new_row.tiles.extend(tiles);
				new_row.lengths.extend(lengths);
			}
			new_row
		})
		.collect::<Vec<_>>();

	pt1(&new_rows)
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
			(pt1, 21),
			(pt2, 525_152),
		);
	}

	#[test]
	fn test() {
		run_tests(
			&*read_input(&read_file!("input.txt")),
			(pt1, 7716),
			(pt2, 779_032_247_216),
		);
	}
}
