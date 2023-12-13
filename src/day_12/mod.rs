// tag::setup[]
use crate::{
	error::{AocResult, ToResultDefaultErr},
	read_file, Answer, AocError,
};
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

#[derive(Debug, Clone, Copy)]
struct RowRef<'a> {
	tiles: &'a [Tile],
	lengths: &'a [usize],
}

// tag::debugging[]
impl fmt::Display for RowRef<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for t in self.tiles {
			f.write_char(match t {
				Tile::Known(Spring::Operational) => '.',
				Tile::Known(Spring::Damaged) => '#',
				Tile::Unknown => '?',
			})?;
		}

		f.write_char(',')?;
		for n in self.lengths {
			write!(f, "{n},")?;
		}

		Ok(())
	}
}
// end::debugging[]

impl<'a> RowRef<'a> {
	fn new(Row { tiles, lengths }: &'a Row) -> Self {
		Self { tiles, lengths }
	}
}

impl<'a> RowRef<'a> {
	/// Strategy: divide and conquer. Take a length of `middle_length` damaged springs
	/// and run it over the list of tiles; where it "fits" (there is a contiguous section
	/// of either unknown or known-damaged tiles, and before and after it are either the
	/// ends of the list or  unknown or known-operational tiles), take the front lengths
	/// and count how many ways they work for the part before where the `middle_length`
	/// fit, and multiply that by the number of ways the back lengths work for the part
	/// after where the middle length fit. Sum over all places the middle length fit;
	/// tada.
	fn count_solns(self, cache: &mut HashMap<(&'a [Tile], &'a [usize]), usize>) -> usize {
		let Self { tiles, lengths } = self;
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
			if tiles[i..i + len]
				.iter()
				.all(|&t| t != Tile::Known(Spring::Operational))
			{
				let front_count = if i == 0 {
					usize::from(front.is_empty())
				} else if tiles[i - 1] == Tile::Known(Spring::Damaged) {
					continue;
				} else {
					RowRef {
						tiles: &tiles[..i - 1],
						lengths: front,
					}
					.count_solns(cache)
				};

				let back_count = if i == tiles.len() - len {
					usize::from(back.is_empty())
				} else if tiles[i + len] == Tile::Known(Spring::Damaged) {
					continue;
				} else {
					RowRef {
						tiles: &tiles[i + len + 1..],
						lengths: back,
					}
					.count_solns(cache)
				};

				count += front_count * back_count;
			}
		}

		cache.insert((tiles, lengths), count);

		count
	}
}

// end::setup[]

// tag::pt1[]
fn pt1(rows: &[Row]) -> usize {
	let mut cache = HashMap::new();
	rows.iter()
		.map(|row| RowRef::new(row).count_solns(&mut cache))
		.sum()
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
