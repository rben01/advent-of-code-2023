// tag::setup[]
use crate::{
	error::{AocResult, ToResultDefaultErr},
	read_file, Answer, AocError,
};
use std::{
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

// tag::debugging[]
impl fmt::Display for Row {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for t in &self.tiles {
			f.write_char(match t {
				Tile::Known(Spring::Operational) => '.',
				Tile::Known(Spring::Damaged) => '#',
				Tile::Unknown => '?',
			})?;
		}

		for n in &self.lengths {
			write!(f, ",{n}")?;
		}

		Ok(())
	}
}
// end::debugging[]

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

#[allow(clippy::too_many_lines)]
impl Row {
	/// precondition: every row ends with a known operational tile (which is used to
	/// "cleave off" the last damaged run in case it goes all the way to the end)
	fn count_solns(&self) -> usize {
		#[derive(Debug, Clone, Copy)]
		struct Candidate {
			up_to_idx: usize,
			complete_runs_seen: usize,
			curr_run_len: usize,
		}

		println!("{self}");

		let Self { tiles, lengths } = self;
		let tiles = {
			let mut v = tiles.clone();
			// a dummy operation tile at the ends to "cleave off" runs
			v.push(Tile::Known(Spring::Operational));
			v
		};

		let min_runs_by_idx =
			{
				let mut v = vec![0; tiles.len()];
				let mut in_run = false;
				let mut n_runs = 0;

				for (i, tile) in tiles.iter().enumerate().rev() {
					match tile {
						Tile::Known(Spring::Damaged) => in_run = true,
						Tile::Known(Spring::Operational) => {
							if in_run {
								n_runs += 1;
								in_run = false;
							}
						}
						Tile::Unknown => {
							if in_run {
								n_runs += 1;
								in_run = false;
							} else {
								in_run = true;
							}
						}
					}
					v[i] = n_runs;
				}

				v
			};

		println!("{min_runs_by_idx:?}");

		let max_runs_by_idx =
			{
				let mut v = vec![0; tiles.len()];
				let mut in_run = false;
				let mut n_runs = 0;

				for (i, tile) in tiles.iter().enumerate() {
					match tile {
						Tile::Known(Spring::Damaged) => in_run = true,
						Tile::Known(Spring::Operational) => {
							if in_run {
								n_runs += 1;
								in_run = false;
							}
						}
						Tile::Unknown => {
							if in_run {
								n_runs += 1;
								in_run = false;
							} else {
								in_run = true;
							}
						}
					}
					v[i] = n_runs;
				}

				v
			};

		// println!("{:?}", (&min_runs_by_idx, &max_runs_by_idx));

		let n_lengths = lengths.len();

		let mut count = 0;

		let mut candidates = vec![Candidate {
			up_to_idx: 0,
			complete_runs_seen: 0,
			curr_run_len: 0,
		}];

		'candidates: while let Some(Candidate {
			up_to_idx,
			mut complete_runs_seen,
			mut curr_run_len,
		}) = candidates.pop()
		{
			// println!(
			// 	"{:?}",
			// 	(
			// 		candidate,
			// 		max_runs_by_idx[up_to_idx],
			// 		min_runs_by_idx[up_to_idx]
			// 	)
			// );
			if complete_runs_seen > max_runs_by_idx[up_to_idx]
				|| complete_runs_seen + min_runs_by_idx[up_to_idx] + 1 < n_lengths
			{
				// println!("{self}, continuing");
				continue;
			}

			for (i, &tile) in tiles.iter().enumerate().skip(up_to_idx) {
				let Tile::Known(state) = tile else {
					// this is the first unknown tile yet to be (tentatively) replaced

					// check if Operational is a valid candidate
					if curr_run_len == 0 || curr_run_len == lengths[complete_runs_seen] {
						candidates.push(Candidate {
							up_to_idx: i + 1,
							complete_runs_seen: usize::from(curr_run_len > 0) + complete_runs_seen,
							curr_run_len: 0,
						});
					}

					// check if Damaged is a valid candidate
					if complete_runs_seen < n_lengths && curr_run_len < lengths[complete_runs_seen]
					{
						candidates.push(Candidate {
							up_to_idx: i + 1,
							complete_runs_seen,
							curr_run_len: curr_run_len + 1,
						});
					}

					continue 'candidates;
				};

				match state {
					Spring::Operational => {
						if curr_run_len > 0 {
							if curr_run_len != lengths[complete_runs_seen] {
								continue 'candidates;
							}

							complete_runs_seen += 1;

							if complete_runs_seen > n_lengths {
								continue 'candidates;
							}

							curr_run_len = 0;
						}
					}
					Spring::Damaged => {
						curr_run_len += 1;

						if complete_runs_seen >= n_lengths
							|| curr_run_len > lengths[complete_runs_seen]
						{
							continue 'candidates;
						}
					}
				}
			}

			if complete_runs_seen >= n_lengths {
				count += 1;
			}
		}

		dbg!(count)
		// count
	}
}

// end::setup[]

// tag::pt1[]
fn pt1(rows: &[Row]) -> usize {
	rows.iter().map(|row| row.count_solns()).sum()
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
