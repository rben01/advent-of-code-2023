// tag::setup[]
use crate::{
	error::{AocResult, ToResultDefaultErr},
	read_file, Answer, AocError, Cast,
};
use std::str::FromStr;

fn ans_for_input(input: &str) -> Answer<u32, u32> {
	let instrs = read_input(input);
	(15, (pt1(&instrs), pt2(&instrs))).into()
}

pub fn ans() -> Answer<u32, u32> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Vec<Instr> {
	input
		.trim()
		.split(',')
		.map(Instr::from_str)
		.collect::<AocResult<_>>()
		.unwrap()
}

fn hash_into(h: &mut u32, b: u8) {
	*h += u32::from(b);
	*h *= 17;
	*h %= 256;
}

#[derive(Debug, Clone, Copy)]
enum Operation {
	Remove,
	Add(u8),
}

#[derive(Debug, Clone)]
struct Instr {
	label: Vec<u8>,
	op: Operation,
}

impl FromStr for Instr {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut bytes = s.bytes();

		let mut label = Vec::new();
		let mut op_byte = 0;

		for b in bytes.by_ref() {
			match b {
				b'a'..=b'z' | b'A'..=b'Z' => {
					label.push(b);
				}
				_ => {
					op_byte = b;
					break;
				}
			}
		}

		let op = match op_byte {
			b'-' => Operation::Remove,
			b'=' => Operation::Add(bytes.next().to_result()? - b'0'),
			b => return Err(AocError::Other(format!("invalid byte {b:?}"))),
		};
		if bytes.next().is_some() {
			return Err(AocError::Other(format!("did not consume all of {s:?}")));
		}

		Ok(Self { label, op })
	}
}

impl Instr {
	fn hash(&self) -> u32 {
		let mut h = 0;
		for &b in &self.label {
			hash_into(&mut h, b);
		}
		match self.op {
			Operation::Remove => hash_into(&mut h, b'-'),
			Operation::Add(n) => {
				hash_into(&mut h, b'=');
				hash_into(&mut h, n + b'0');
			}
		}
		h
	}
}
// end::setup[]

// tag::pt1[]
fn pt1(instrs: &[Instr]) -> u32 {
	instrs.iter().map(|instr| instr.hash()).sum()
}
// end::pt1[]

// tag::pt2[]
fn pt2(instrs: &[Instr]) -> u32 {
	#[derive(Debug, Clone, PartialEq, Eq)]
	struct Lens<'a> {
		label: &'a [u8],
		focal_len: u8,
	}

	let mut boxes = std::array::from_fn::<_, 256, _>(|_| Vec::<Lens>::new());

	for Instr { label, op } in instrs {
		let box_idx = {
			let mut h = 0;
			for &b in label {
				hash_into(&mut h, b);
			}
			h.cast::<usize>()
		};
		let box_ = &mut boxes[box_idx];
		match op {
			Operation::Remove => {
				if let Some(i) = box_.iter().position(|lens| lens.label == label) {
					box_.remove(i);
				}
			}
			&Operation::Add(focal_len) => match box_.iter_mut().find(|lens| lens.label == label) {
				Some(lens) => lens.focal_len = focal_len,
				None => box_.push(Lens { label, focal_len }),
			},
		}
	}

	(1..)
		.zip(boxes)
		.map(|(box_idx, box_)| {
			box_idx
				* ((1..)
					.zip(box_)
					.map(|(slot_idx, lens)| slot_idx * u32::from(lens.focal_len)))
				.sum::<u32>()
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
	fn sample() {
		run_tests(
			&*read_input(&read_file!("sample_input.txt")),
			(pt1, 1320),
			(pt2, 145),
		);
	}

	#[test]
	fn test() {
		run_tests(
			&*read_input(&read_file!("input.txt")),
			(pt1, 505_427),
			(pt2, 243_747),
		);
	}
}
