use std::{
	collections::{HashMap, HashSet},
	fmt::Write,
};

// tag::setup[]
use crate::{read_file, utils::get_nsew_diag_adjacent, Answer};
use ndarray::prelude::*;

fn ans_for_input(input: &str) -> Answer<u32, u32> {
	let grid = read_input(input);
	(3, (pt1(grid.view()), pt2(grid.view()))).into()
}

pub fn ans() -> Answer<u32, u32> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Array2<Entry> {
	let mut height = 0;

	let mut items = Vec::new();
	for line in input.lines() {
		height += 1;
		for c in line.chars() {
			items.push(Entry::from(c));
		}
	}

	let width = items.len() / height;
	Array2::from_shape_vec((height, width), items).expect("could not make array")
}

#[derive(Debug, Clone, Copy)]
enum Entry {
	Blank,
	Symbol(char),
	Digit(u32),
}

impl From<char> for Entry {
	fn from(value: char) -> Self {
		match value {
			'.' => Self::Blank,
			'0'..='9' => Self::Digit(value.to_digit(10).unwrap()),
			_ => Self::Symbol(value),
		}
	}
}
// end::setup[]

// tag::pt1[]
fn pt1(games: ArrayView2<Entry>) -> u32 {
	let (height, width) = games.raw_dim().into_pattern();
	let mut sum = 0;

	let mut curr_num = 0;
	let mut should_include = false;

	for (ri, row) in games.rows().into_iter().enumerate() {
		for (ci, &entry) in row.into_iter().enumerate() {
			if ci == 0 || matches!(entry, Entry::Blank | Entry::Symbol(_)) {
				if should_include {
					sum += curr_num;
				}
				curr_num = 0;
				should_include = false;
			}

			if let Entry::Digit(d) = entry {
				curr_num = 10 * curr_num + d;

				if !should_include {
					should_include = get_nsew_diag_adjacent((ci, ri), 0..width, 0..height)
						.any(|(x, y)| matches!(games[[y, x]], Entry::Symbol(_)));
				}
			}
		}
	}
	sum
}
// end::pt1[]

// tag::pt2[]
fn pt2(games: ArrayView2<Entry>) -> u32 {
	let (height, width) = games.raw_dim().into_pattern();

	let mut gears = HashMap::<[usize; 2], Vec<u32>>::new();
	let mut curr_num = 0;
	let mut curr_gear_locs = HashSet::new();

	for (ri, row) in games.rows().into_iter().enumerate() {
		for (ci, &entry) in row.into_iter().enumerate() {
			if ci == 0 || matches!(entry, Entry::Blank | Entry::Symbol(_)) {
				for &gear_loc in &curr_gear_locs {
					gears
						.entry(gear_loc)
						.and_modify(|nums| nums.push(curr_num))
						.or_insert_with(|| vec![curr_num]);
				}
				curr_num = 0;
				curr_gear_locs.clear();
			}

			if let Entry::Digit(d) = entry {
				curr_num = 10 * curr_num + d;
				for (x, y) in get_nsew_diag_adjacent((ci, ri), 0..width, 0..height) {
					if matches!(games[[y, x]], Entry::Symbol('*')) {
						curr_gear_locs.insert([x, y]);
					}
				}
			}
		}
	}

	gears
		.into_values()
		.filter_map(|nums| (nums.len() == 2).then(|| nums.into_iter().product::<u32>()))
		.sum()
}
// end::pt2[]

#[cfg(test)]
mod test {
	#![allow(unused_imports)]

	use super::*;
	use crate::{run_test, run_tests};

	#[test]
	fn test() {
		run_tests(
			read_input(&read_file!("sample_input.txt")).view(),
			(pt1, 4361),
			(pt2, 467_835),
		);
		run_tests(
			read_input(&read_file!("input.txt")).view(),
			(pt1, 531_561),
			(pt2, 83_279_367),
		);
	}
}
