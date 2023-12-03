// tag::setup[]
use crate::{read_file, Answer};

fn ans_for_input(input: &str) -> Answer<u32, u32> {
	let text = read_input(input);
	(1, (pt1(text.lines()), pt2(text.lines()))).into()
}

pub fn ans() -> Answer<u32, u32> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> &str {
	input
}
// end::setup[]

// tag::pt1[]
fn lines_to_nums(lines: impl IntoIterator<Item = &str>, words_as_digits: bool) -> u32 {
	// digit_strs[i] => stringified(i)
	let digit_strs = [
		"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
	];
	lines
		.into_iter()
		.map(|line| {
			let mut first_digit = None;
			let mut last_digit = None;

			for (i, c) in line.char_indices() {
				let digit = c.to_digit(10).or_else(|| {
					if words_as_digits {
						(0..).zip(&digit_strs).find_map(|(digit_value, digit_str)| {
							line[i..].starts_with(digit_str).then_some(digit_value)
						})
					} else {
						None
					}
				});

				if digit.is_some() {
					if first_digit.is_none() {
						first_digit = digit;
					}
					last_digit = digit;
				}
			}

			first_digit.unwrap() * 10 + last_digit.unwrap()
		})
		.sum()
}

fn pt1(lines: impl IntoIterator<Item = &str>) -> u32 {
	lines_to_nums(lines, false)
}
// end::pt1[]

// tag::pt2[]
fn pt2(lines: impl IntoIterator<Item = &str>) -> u32 {
	lines_to_nums(lines, true)
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
			read_input(&read_file!("sample_input_1.txt")).lines(),
			(pt1, 142),
		);
		run_test(
			read_input(&read_file!("sample_input_2.txt")).lines(),
			(pt2, 281),
		);
		run_tests(
			read_input(&read_file!("input.txt")).lines(),
			(pt1, 55816),
			(pt2, 54980),
		);
	}
}
