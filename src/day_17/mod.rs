// tag::setup[]
use crate::{error::AocError, grid::Grid, read_file, utils::Direction, Answer};
use priority_queue::PriorityQueue;
use std::{cmp, collections::HashSet};

fn ans_for_input(input: &str) -> Answer<u32, u32> {
	let map = read_input(input);
	println!("{map:?}");
	(17, (pt1(&map), pt2(&map))).into()
}

pub fn ans() -> Answer<u32, u32> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Map {
	Map::from_str_chars(input, |c| {
		c.to_digit(10)
			.ok_or(AocError::Other(format!("coult not convert {c:?} to digit")))
	})
	.unwrap()
}

type Map = Grid<u32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Route {
	pos: [usize; 2],
	heat_loss: u32,
	direction: Direction,
	prev_straight: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Badness {
	heat_loss: u32,
	distance: usize,
}

fn navigation_cost(
	map: &Map,
	must_turn_fn: impl Fn(usize) -> bool,
	can_turn_fn: impl Fn(usize) -> bool,
) -> u32 {
	use Direction::*;

	let [nr, nc] = map.dim();
	let mut visited = HashSet::new();

	let mut pq = PriorityQueue::new();

	for dir in [N, S, E, W] {
		pq.push(
			Route {
				pos: [0, 0],
				heat_loss: 0,
				direction: dir,
				prev_straight: 1,
			},
			cmp::Reverse(Badness {
				heat_loss: 0,
				distance: (nr - 1) + (nc - 1),
			}),
		);
	}

	while let Some((
		Route {
			pos: [ri, ci],
			heat_loss,
			direction,
			prev_straight,
		},
		_,
	)) = pq.pop()
	{
		if ri == nr - 1 && ci == nc - 1 {
			if can_turn_fn(prev_straight) {
				return heat_loss;
			}
			continue;
		}

		if !visited.insert(([ri, ci], prev_straight, direction)) {
			continue;
		}

		for dir in [N, S, E, W] {
			let moving_straight = dir == direction;
			if moving_straight && must_turn_fn(prev_straight + 1)
				|| !moving_straight && !can_turn_fn(prev_straight)
			{
				continue;
			}
			let mut ri = ri;
			let mut ci = ci;

			match dir {
				N if direction == S => continue,
				S if direction == N => continue,
				E if direction == W => continue,
				W if direction == E => continue,

				N if ri > 0 => ri -= 1,
				S if ri < nr - 1 => ri += 1,
				E if ci < nc - 1 => ci += 1,
				W if ci > 0 => ci -= 1,

				_ => continue,
			}

			let heat_loss = heat_loss + map.grid()[[ri, ci]];

			let route = Route {
				pos: [ri, ci],
				heat_loss,
				direction: dir,
				prev_straight: if moving_straight {
					prev_straight + 1
				} else {
					1
				},
			};
			let priority = cmp::Reverse(Badness {
				heat_loss,
				distance: (nr - ri - 1) + (nc - ci - 1),
			});

			pq.push(route, priority);
		}
	}

	panic!("couldn't find exit")
}
// end::setup[]

// tag::pt1[]
fn pt1(map: &Map) -> u32 {
	navigation_cost(map, |prev_straight| prev_straight > 3, |_| true)
}
// end::pt1[]

// tag::pt2[]
fn pt2(map: &Map) -> u32 {
	navigation_cost(
		map,
		|prev_straight| prev_straight > 10,
		|prev_straight| prev_straight >= 4,
	)
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
			&read_input(&read_file!("sample_input_1.txt")),
			(pt1, 102),
			(pt2, 94),
		);
		run_test(&read_input(&read_file!("sample_input_2.txt")), (pt2, 71));
	}

	#[test]
	fn test() {
		run_tests(
			&read_input(&read_file!("input.txt")),
			(pt1, 742),
			(pt2, 918),
		);
	}
}
