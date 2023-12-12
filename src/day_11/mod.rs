// tag::setup[]
use crate::{read_file, Answer, AocError};
use std::{collections::HashSet, str::FromStr};

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let img = read_input(input);
	(11, (pt1(&img), pt2(&img))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Image {
	input.parse().unwrap()
}

#[derive(Debug)]
struct Image {
	galaxy_locs: Vec<[usize; 2]>,
	empty_rows: Vec<bool>,
	empty_cols: Vec<bool>,
}

impl FromStr for Image {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut height = 0;
		let mut n_tiles = 0;

		let mut galaxy_locs = Vec::new();
		let mut occupied_rows = HashSet::new();
		let mut occupied_cols = HashSet::new();

		for (ri, line) in s.lines().enumerate() {
			for (ci, c) in line.char_indices() {
				n_tiles += 1;
				match c {
					'.' => {}
					'#' => {
						galaxy_locs.push([ri, ci]);
						occupied_rows.insert(ri);
						occupied_cols.insert(ci);
					}
					_ => return Err(AocError::Other(format!("invalid char {c:?}"))),
				};
			}
			height += 1;
		}
		let width = n_tiles / height;

		let empty_rows = (0..height).map(|ri| !occupied_rows.contains(&ri)).collect();
		let empty_cols = (0..width).map(|ci| !occupied_cols.contains(&ci)).collect();

		Ok(Self {
			galaxy_locs,
			empty_rows,
			empty_cols,
		})
	}
}

fn get_distances(img: &Image, expansion_factor: usize) -> usize {
	let Image {
		galaxy_locs,
		empty_rows,
		empty_cols,
	} = img;

	let mut net_dist = 0;
	for (i, &p1) in galaxy_locs.iter().enumerate().skip(1) {
		for &p2 in galaxy_locs.iter().take(i) {
			// one shortest path is just to go straight horizontal and then straight
			// vertical, which obviously has the length computed as `dist` below

			let [r1, c1] = p1;
			let [r2, c2] = p2;

			let r_min = r1.min(r2);
			let r_max = r1.max(r2);
			let c_min = c1.min(c2);
			let c_max = c1.max(c2);

			let dist = (r_max - r_min)
				+ (r_min..r_max).filter(|&ri| empty_rows[ri]).count() * (expansion_factor - 1)
				+ (c_max - c_min)
				+ (c_min..c_max).filter(|&ci| empty_cols[ci]).count() * (expansion_factor - 1);

			net_dist += dist;
		}
	}

	net_dist
}
// end::setup[]

// tag::pt1[]
fn pt1(img: &Image) -> usize {
	get_distances(img, 2)
}
// end::pt1[]

// tag::pt2[]
fn pt2(img: &Image) -> usize {
	get_distances(img, 1_000_000)
}
// end::pt2[]

#[cfg(test)]
mod test {
	#![allow(unused_imports)]

	use super::*;
	use crate::{run_test, run_tests};

	#[test]
	fn sample() {
		let input = read_input(&read_file!("sample_input.txt"));
		run_test(&input, (pt1, 374));
		run_test(&input, (|img| get_distances(img, 10), 1030));
		run_test(&input, (|img| get_distances(img, 100), 8410));
	}

	#[test]
	fn test() {
		run_tests(
			&read_input(&read_file!("input.txt")),
			(pt1, 9_805_264),
			(pt2, 779_032_247_216),
		);
	}
}
