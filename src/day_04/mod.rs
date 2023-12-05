// tag::setup[]
use crate::{read_file, regex, Answer, AocResult, Cast, ToResultDefaultErr};
use std::{collections::HashSet, str::FromStr};

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let cards = read_input(input);
	(4, (pt1(&cards), pt2(&cards))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Vec<Card> {
	input
		.lines()
		.map(Card::from_str)
		.collect::<AocResult<_>>()
		.unwrap()
}

struct Card {
	#[allow(dead_code)]
	id: u32,
	winning_nums: HashSet<u32>,
	have_nums: Vec<u32>,
}

impl FromStr for Card {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let card_re =
			regex!(r"Card\s+(?P<id>\d+)\s*:\s*(?P<winning_nums>[^|]+)\|(?P<have_nums>.*)$");
		let m = card_re.captures(s).to_result()?;

		let id = m
			.name("id")
			.to_result()?
			.as_str()
			.parse::<u32>()
			.map_err(|e| e.to_string())?;

		let winning_nums = regex!(r"\d+")
			.find_iter(m.name("winning_nums").to_result()?.as_str())
			.map(|num_str| num_str.as_str().parse::<u32>().map_err(|e| e.to_string()))
			.collect::<AocResult<_>>()?;
		let have_nums = regex!(r"\d+")
			.find_iter(m.name("have_nums").to_result()?.as_str())
			.map(|num_str| num_str.as_str().parse::<u32>().map_err(|e| e.to_string()))
			.collect::<AocResult<_>>()?;

		Ok(Card {
			id,
			winning_nums,
			have_nums,
		})
	}
}

impl Card {
	fn n_winning(&self) -> usize {
		self.have_nums
			.iter()
			.filter(|n| self.winning_nums.contains(n))
			.count()
	}
}
// end::setup[]

// tag::pt1[]
fn pt1(cards: impl IntoIterator<Item = &Card>) -> usize {
	cards
		.into_iter()
		.map(|card| {
			let n_winning_nums = card.n_winning();
			if n_winning_nums == 0 {
				0
			} else {
				2_u32.pow((n_winning_nums - 1).cast()).cast()
			}
		})
		.sum()
}
// end::pt1[]

// tag::pt2[]
fn pt2(cards: &[Card]) -> usize {
	let mut card_counts = vec![1; cards.len()];

	for (i, card) in cards.iter().enumerate() {
		let count = card_counts[i];
		for c in card_counts.iter_mut().skip(i + 1).take(card.n_winning()) {
			*c += count;
		}
	}

	card_counts.iter().copied().sum()
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
			&*read_input(&read_file!("sample_input.txt")),
			(pt1, 13),
			(pt2, 30),
		);
		run_tests(
			&*read_input(&read_file!("input.txt")),
			(pt1, 26_443),
			(pt2, 6_284_877),
		);
	}
}
