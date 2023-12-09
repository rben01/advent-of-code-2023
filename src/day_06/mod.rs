// tag::setup[]
use crate::{error::AocResult, read_file, regex, Answer, AocError, ToResultDefaultErr};
use std::str::FromStr;

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let input = read_input(input);
	(6, (pt1(&input), pt2(&input))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Input {
	input.parse().unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Race {
	time: u64,
	distance: u64,
}

impl Race {
	/// Let `T` be total race time \
	/// Let `t` be charge time (and hence speed) \
	/// Let `D` be distance record \
	/// Then the distance we go is `(T-t)*t`. We set a record when this exceeds `D`, ie.
	/// `t^2-T*t+D < 0`. Solving the quadratic inequality in `t`, we get: \
	/// `(T-sqrt(T^2-4D))/2 < t < (T+sqrt(T^2-4D))/2`
	/// The number of ways to break the record is the number of integer solutions for t
	fn n_records(self) -> usize {
		#![allow(
			non_snake_case,
			clippy::cast_possible_truncation,
			clippy::cast_sign_loss,
			clippy::cast_precision_loss
		)]

		fn is_approx_integer(f: f64) -> bool {
			(f - f.round()).abs() < 1e-6
		}

		let T = self.time as f64;
		let D = self.distance as f64;

		let discr = T.powi(2) - 4.0 * D;
		if discr < 0.0 {
			return 0;
		}

		let lower = (T - discr.sqrt()) / 2.0;
		let upper = (T + discr.sqrt()) / 2.0;

		// and now we just need to count the number of integers (strictly) between lower
		// and upper
		let lower_int = if is_approx_integer(lower) {
			lower.round() + 1.0
		} else {
			lower.ceil()
		};
		let upper_int = if is_approx_integer(upper) {
			upper.round() - 1.0
		} else {
			upper.floor()
		};

		(upper_int - lower_int + 1.0).max(0.0) as _
	}
}

#[derive(Debug)]
struct Input(Vec<Race>);

impl FromStr for Input {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let digits_re = regex!(r"\d+");
		let mut lines = s.lines();

		let times = digits_re.find_iter(lines.next().to_result()?);
		let distances = digits_re.find_iter(lines.next().to_result()?);

		let races = times
			.zip(distances)
			.map(|(time, distance)| {
				let [time, distance] = [time, distance].try_map(|m| m.as_str().parse())?;
				Ok(Race { time, distance })
			})
			.collect::<AocResult<_>>()?;

		Ok(Input(races))
	}
}

// end::setup[]

// tag::pt1[]
fn pt1(input: &Input) -> usize {
	input.0.iter().map(|race| race.n_records()).product()
}
// end::pt1[]

// tag::pt2[]
fn pt2(input: &Input) -> usize {
	fn n_digits_b10(mut n: u64) -> u32 {
		let mut i = 0;
		while n > 0 {
			n /= 10;
			i += 1;
		}
		i
	}
	let (time, distance) =
		input
			.0
			.iter()
			.fold((0, 0), |(t_acc, d_acc), &Race { time, distance }| {
				let [n_digits_t, n_digits_d] = [time, distance].map(n_digits_b10);
				let t_acc = t_acc * 10_u64.pow(n_digits_t) + time;
				let d_acc = d_acc * 10_u64.pow(n_digits_d) + distance;
				(t_acc, d_acc)
			});

	pt1(&Input(vec![Race { time, distance }]))
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
			&read_input(&read_file!("sample_input.txt")),
			(pt1, 288),
			(pt2, 71503),
		);
	}

	#[test]
	fn test() {
		run_tests(
			&read_input(&read_file!("input.txt")),
			(pt1, 1_084_752),
			(pt2, 28_228_952),
		);
	}
}
