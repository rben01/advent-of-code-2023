// tag::setup[]
use crate::{read_file, regex, Answer, AocError, ToResultDefaultErr};
use std::{collections::HashMap, str::FromStr};

fn ans_for_input(input: &str) -> Answer<i64, i64> {
	let input = read_input(input);
	(5, (pt1(&input), pt2(&input))).into()
}

pub fn ans() -> Answer<i64, i64> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Input {
	input.parse().unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Span {
	lo: i64,
	hi: i64,
}

#[derive(Debug, Clone, Copy)]
struct RangeMap {
	src: i64,
	dst: i64,
	len: i64,
}

#[derive(Debug)]
struct Mapping {
	to: String,
	ranges: Vec<RangeMap>,
}

impl Mapping {
	fn map_seed_spans(&self, spans: Vec<Span>) -> Vec<Span> {
		let mut shifted_spans = Vec::new();
		let mut unshifted_spans = spans;
		let mut new_unshifted_spans = Vec::new();

		for &RangeMap { src, dst, len } in &self.ranges {
			new_unshifted_spans.clear();

			for &span in &unshifted_spans {
				// if no overlap, push span as is
				if src >= span.hi || src + len < span.lo {
					new_unshifted_spans.push(span);
					continue;
				}

				// every range that intersects a span cuts it into three (not necessarily
				// nonempty) sections: left, middle, right
				let left_lo = span.lo;
				let left_hi = left_lo.max(src);

				let right_hi = span.hi;
				let right_lo = right_hi.min(src + len);

				for (lo, hi) in [(left_lo, left_hi), (right_lo, right_hi)] {
					if lo < hi {
						new_unshifted_spans.push(Span { lo, hi });
					}
				}

				// middle of span (overlaps with range)
				if left_hi < right_lo {
					let shift = dst - src;

					shifted_spans.push(Span {
						lo: left_hi + shift,
						hi: right_lo + shift,
					});
				}
			}

			// save some allocations by reusing Vecs
			std::mem::swap(&mut unshifted_spans, &mut new_unshifted_spans);
		}

		let mut split_spans = [unshifted_spans, shifted_spans].concat();

		// merge adjacent/overlapping spans
		split_spans.sort_by_key(|span| span.lo);

		let mut spans = Vec::<Span>::new();
		for next_span in split_spans {
			if let Some(prev_span) = spans.pop() {
				// if the spans overlap, merge them
				if prev_span.hi >= next_span.lo {
					spans.push(Span {
						lo: prev_span.lo,
						hi: next_span.hi,
					});
				} else {
					spans.push(prev_span);
					spans.push(next_span);
				}
			} else {
				spans.push(next_span);
			}
		}

		spans
	}
}

#[derive(Debug)]
struct Input {
	seeds: Vec<i64>,
	mappings: HashMap<String, Mapping>,
}

impl FromStr for Input {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// exactly one empty line at end
		let mut lines = s.trim_end().lines().chain(std::iter::once(""));

		let first = lines.next().to_result()?;
		let seeds = regex!(r"\d+")
			.find_iter(first)
			.map(|m| m.as_str().parse::<i64>())
			.collect::<Result<_, _>>()?;

		let mut mappings = HashMap::new();

		let mut from = None;
		let mut to = None;
		let mut ranges = Vec::new();

		lines.next().to_result()?;
		for line in lines {
			if line.trim().is_empty() {
				mappings.insert(
					from.take().to_result()?,
					Mapping {
						to: to.take().to_result()?,
						ranges,
					},
				);

				ranges = Vec::new();
			} else if let Some(caps) = regex!(r"(?P<from>\w+)-to-(?P<to>\w+) map").captures(line) {
				[from, to] =
					["from", "to"].map(|name| Some(caps.name(name).unwrap().as_str().to_owned()));
			} else {
				let caps = regex!(r"(?P<dst>\d+)\s+(?P<src>\d+)\s+(?P<len>\d+)")
					.captures(line)
					.unwrap();
				let [src, dst, len] = ["src", "dst", "len"]
					.map(|name| caps.name(name).unwrap().as_str().parse::<i64>());
				let [src, dst, len] = [src?, dst?, len?];
				ranges.push(RangeMap { src, dst, len });
			}
		}

		Ok(Input { seeds, mappings })
	}
}

impl Input {
	fn follow_seed_spans(&self, mut spans: Vec<Span>) -> Vec<Span> {
		let mut src = "seed";

		while src != "location" {
			let mapping = &self.mappings[src];
			spans = mapping.map_seed_spans(spans);
			src = &mapping.to;
		}

		spans
	}
}
// end::setup[]

// tag::pt1[]
fn pt1(input: &Input) -> i64 {
	input
		.seeds
		.iter()
		.flat_map(|&seed| {
			input.follow_seed_spans(vec![Span {
				lo: seed,
				hi: seed + 1,
			}])
		})
		.map(|span| span.lo)
		.min()
		.unwrap()
}
// end::pt1[]

// tag::pt2[]
fn pt2(input: &Input) -> i64 {
	let seed_spans = input
		.seeds
		.chunks_exact(2)
		.map(|seed_data| {
			let start = seed_data[0];
			let len = seed_data[1];
			Span {
				lo: start,
				hi: start + len,
			}
		})
		.collect();

	input
		.follow_seed_spans(seed_spans)
		.iter()
		.map(|span| span.lo)
		.min()
		.unwrap()
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
			&read_input(&read_file!("sample_input.txt")),
			(pt1, 35),
			(pt2, 46),
		);
		run_tests(
			&read_input(&read_file!("input.txt")),
			(pt1, 278_755_257),
			(pt2, 26_829_166),
		);
	}
}
