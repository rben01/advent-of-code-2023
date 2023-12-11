// tag::setup[]
use crate::{error::AocResult, read_file, Answer, AocError};
use std::str::FromStr;

fn ans_for_input(input: &str) -> Answer<i64, i64> {
	let seqs = read_input(input);
	(9, (pt1(&seqs), pt2(&seqs))).into()
}

pub fn ans() -> Answer<i64, i64> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Vec<Sequence> {
	input
		.lines()
		.map(Sequence::from_str)
		.collect::<AocResult<_>>()
		.unwrap()
}

#[derive(Debug, Clone)]
struct Sequence(Vec<i64>);

impl FromStr for Sequence {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.split_whitespace()
			.map(|w| Ok(w.parse()?))
			.collect::<AocResult<_>>()
			.map(Self)
	}
}

impl Sequence {
	fn diff(&self) -> Self {
		Self(self.0.iter().map_windows(|&[a, b]| b - a).collect())
	}

	fn is_zeroed(&self) -> bool {
		self.0.iter().all(|&n| n == 0)
	}

	fn first(&self) -> i64 {
		self.0[0]
	}

	fn last(&self) -> i64 {
		self.0[self.0.len() - 1]
	}

	fn next_num(&self) -> i64 {
		if self.is_zeroed() {
			return 0;
		}

		let mut last_nums = vec![self.last()];
		let mut diffs = self.diff();
		while !diffs.is_zeroed() {
			last_nums.push(diffs.last());
			diffs = diffs.diff();
		}

		last_nums.into_iter().sum()
	}

	fn prev_num(&self) -> i64 {
		if self.is_zeroed() {
			return 0;
		}

		let mut first_nums = vec![self.first()];
		let mut diffs = self.diff();
		while !diffs.is_zeroed() {
			first_nums.push(diffs.first());
			diffs = diffs.diff();
		}

		first_nums
			.into_iter()
			.zip(0..)
			.map(|(n, i)| (-1_i64).pow(i) * n)
			.sum()
	}
}
// end::setup[]

// tag::pt1[]
fn pt1(seqs: &[Sequence]) -> i64 {
	seqs.iter().map(Sequence::next_num).sum()
}
// end::pt1[]

// tag::pt2[]
fn pt2(seqs: &[Sequence]) -> i64 {
	seqs.iter().map(Sequence::prev_num).sum()
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
			(pt1, 114),
			(pt2, 2),
		);
	}

	#[test]
	fn test() {
		run_tests(
			&*read_input(&read_file!("input.txt")),
			(pt1, 1_953_784_198),
			(pt2, 957),
		);
	}
}
