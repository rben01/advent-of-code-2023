// tag::setup[]
use crate::{read_file, regex, Answer, AocResult};
use std::collections::HashMap;
use strum_macros::EnumString;

fn ans_for_input(input: &str) -> Answer<u32, u32> {
	let games = read_input(input).unwrap();

	(1, (pt1(&games), pt2(&games))).into()
}

pub fn ans() -> Answer<u32, u32> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> AocResult<Vec<Game>> {
	input
		.lines()
		.map(|line| {
			let game = regex!(r"Game (?P<id>\d+):\s*");
			let game_id_match = game
				.captures(line)
				.and_then(|cap| cap.name("id"))
				.ok_or_else(|| "could not get game id".to_owned())?;

			let game_id_end = game_id_match.end();
			let game_id = game_id_match
				.as_str()
				.parse::<u32>()
				.map_err(|e| e.to_string())?;

			let rounds = regex!(r"[^:;]+")
				.find_iter(&line[game_id_end..])
				.map(|m| {
					let round = m.as_str().trim();

					regex!(r"(?P<count>\d+)\s+(?P<color>\w+)")
						.captures_iter(round)
						.map(|cube_counts| {
							let count = cube_counts
								.name("count")
								.ok_or_else(|| format!("could not get count from round {round:?}"))?
								.as_str()
								.parse::<u32>()
								.map_err(|e| e.to_string())?;
							let color = cube_counts
								.name("color")
								.ok_or_else(|| format!("could not get color from round {round:?}"))?
								.as_str()
								.parse::<Color>()
								.map_err(|e| e.to_string())?;

							Ok(CubeCount { color, count })
						})
						.collect::<AocResult<Vec<_>>>()
						.map(|counts| Round { counts })
				})
				.collect::<AocResult<Vec<_>>>()?;

			Ok(Game {
				id: game_id,
				rounds,
			})
		})
		.collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString)]
#[strum(ascii_case_insensitive)]
enum Color {
	Red,
	Green,
	Blue,
}

#[derive(Debug, Clone, Copy)]
struct CubeCount {
	color: Color,
	count: u32,
}

#[derive(Debug)]
struct Round {
	counts: Vec<CubeCount>,
}

#[derive(Debug)]
struct Game {
	id: u32,
	rounds: Vec<Round>,
}

impl Round {
	fn is_possible(&self, upper_limits: &HashMap<Color, u32>) -> bool {
		self.counts.iter().all(|CubeCount { color, count }| {
			count
				<= upper_limits
					.get(color)
					.unwrap_or_else(|| panic!("`upper_limits` did not contain {color:?}"))
		})
	}
}

impl Game {
	fn is_possible(&self, upper_limits: &HashMap<Color, u32>) -> bool {
		self.rounds
			.iter()
			.all(|round| round.is_possible(upper_limits))
	}
}
// end::setup[]

// tag::pt1[]
fn pt1(games: impl IntoIterator<Item = &Game>) -> u32 {
	let upper_limits = [
		CubeCount {
			color: Color::Red,
			count: 12,
		},
		CubeCount {
			color: Color::Green,
			count: 13,
		},
		CubeCount {
			color: Color::Blue,
			count: 14,
		},
	]
	.into_iter()
	.map(|CubeCount { color, count }| (color, count))
	.collect::<HashMap<_, _>>();
	games
		.into_iter()
		.filter_map(|game| game.is_possible(&upper_limits).then_some(game.id))
		.sum()
}
// end::pt1[]

// tag::pt2[]
fn pt2(games: impl IntoIterator<Item = &Game>) -> u32 {
	games
		.into_iter()
		.map(|game| {
			let mut curr_counts = HashMap::new();
			for Round { counts } in &game.rounds {
				for &CubeCount { color, count } in counts {
					curr_counts
						.entry(color)
						.and_modify(|value| *value = count.max(*value))
						.or_insert(count);
				}
			}
			curr_counts.values().product::<u32>()
		})
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
		run_test(
			&*read_input(&read_file!("sample_input.txt")).unwrap(),
			(pt1, 8),
		);
		run_test(
			&*read_input(&read_file!("sample_input.txt")).unwrap(),
			(pt2, 2286),
		);
		run_tests(
			&*read_input(&read_file!("input.txt")).unwrap(),
			(pt1, 2683),
			(pt2, 49710),
		);
	}
}
