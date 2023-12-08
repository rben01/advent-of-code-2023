// tag::setup[]
use crate::{enum_map::EnumMap, error::AocResult, read_file, Answer, AocError, ToResultDefaultErr};
use std::{
	cmp,
	fmt::{self, Write},
	str::FromStr,
};
use strum::EnumCount;
use strum_macros::EnumCount;

fn ans_for_input(input: &str) -> Answer<u32, u32> {
	let input = read_input(input);
	(7, (pt1(input.clone()), pt2(input))).into()
}

pub fn ans() -> Answer<u32, u32> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> Vec<Wager> {
	input
		.lines()
		.map(Wager::from_str)
		.collect::<AocResult<_>>()
		.unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumCount)]
#[repr(u8)]
enum Card {
	Wild,
	C2,
	C3,
	C4,
	C5,
	C6,
	C7,
	C8,
	C9,
	T,
	J,
	Q,
	K,
	A,
}

// tag::debugging[]
impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use Card::*;
		f.write_char(match self {
			Wild => 'W',
			C2 => '2',
			C3 => '3',
			C4 => '4',
			C5 => '5',
			C6 => '6',
			C7 => '7',
			C8 => '8',
			C9 => '9',
			T => 'T',
			J => 'J',
			Q => 'Q',
			K => 'K',
			A => 'A',
		})
	}
}
// end::debugging[]

impl From<Card> for usize {
	fn from(card: Card) -> Self {
		(card as u8).into()
	}
}

impl TryFrom<char> for Card {
	type Error = AocError;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		use Card::*;
		Ok(match c {
			'2' => C2,
			'3' => C3,
			'4' => C4,
			'5' => C5,
			'6' => C6,
			'7' => C7,
			'8' => C8,
			'9' => C9,
			'T' => T,
			'J' => J,
			'Q' => Q,
			'K' => K,
			'A' => A,
			_ => return Err(AocError::Other(format!("invalid char {c:?}"))),
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

impl HandType {
	/// `card_counts` must be sorted descending
	fn new(card_counts: &[usize]) -> Self {
		use HandType::*;
		match card_counts {
			[5, ..] => FiveOfAKind,
			[4, ..] => FourOfAKind,
			[3, 2, ..] => FullHouse,
			[3, ..] => ThreeOfAKind,
			[2, 2, ..] => TwoPair,
			[2, ..] => OnePair,
			[1, ..] => HighCard,
			_ => panic!("invalid counts {card_counts:?}"),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand([Card; 5]);

// tag::debugging[]
impl fmt::Display for Hand {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for card in self.0 {
			write!(f, "{card}")?;
		}
		Ok(())
	}
}
// end::debugging[]

impl FromStr for Hand {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut hand = [Card::A; 5];
		let mut chars = s.chars();

		let mut i = 0;
		for c in chars.by_ref() {
			hand[i] = c.try_into()?;
			i += 1;
		}

		if i == 5 {
			Ok(Hand(hand))
		} else {
			Err(AocError::Other(format!("invalid hand {s:?}")))
		}
	}
}

type CardCounts = EnumMap<{ Card::COUNT }, Card, usize>;

impl Hand {
	fn get_card_counts(self) -> CardCounts {
		let mut counts = CardCounts::default();
		for card in self.0 {
			counts[card] += 1;
		}
		counts
	}

	fn hand_type(self) -> HandType {
		let mut counts = self.get_card_counts().into_array();
		counts.sort_by_key(|&c| cmp::Reverse(c));
		HandType::new(&counts)
	}
}

impl cmp::Ord for Hand {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		match self.hand_type().cmp(&other.hand_type()) {
			cmp::Ordering::Equal => self.0.cmp(&other.0),
			ord => ord,
		}
	}
}

impl cmp::PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WildHand(Hand);

impl WildHand {
	fn hand_type(self) -> HandType {
		let mut counts = self.0.get_card_counts();
		let n_jokers = counts[Card::J];
		counts[Card::J] = 0;

		let mut counts = counts.into_array();
		counts.sort_by_key(|&c| cmp::Reverse(c));
		// the best hand with jokers is always found by just treating them as the most
		// populous card
		counts[0] += n_jokers;

		HandType::new(&counts)
	}
}

// tag::debugging[]
impl fmt::Display for WildHand {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
// end::debugging[]

impl cmp::Ord for WildHand {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		match self.hand_type().cmp(&other.hand_type()) {
			cmp::Ordering::Equal => {
				// replace Jack with Wild
				let mut this_hand = self.0;
				let mut other_hand = other.0;

				for arr in [&mut this_hand.0, &mut other_hand.0] {
					for c in arr {
						if *c == Card::J {
							*c = Card::Wild;
						}
					}
				}

				this_hand.0.cmp(&other_hand.0)
			}
			ord => ord,
		}
	}
}

impl cmp::PartialOrd for WildHand {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Debug, Clone, Copy)]
struct Wager {
	hand: Hand,
	bid: u32,
}

impl FromStr for Wager {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split_whitespace();
		let hand = parts.next().to_result()?.parse()?;
		let bid = parts.next().to_result()?.parse()?;

		Ok(Wager { hand, bid })
	}
}

#[derive(Debug, Clone, Copy)]
struct WildWager {
	hand: WildHand,
	bid: u32,
}

impl From<Wager> for WildWager {
	fn from(Wager { hand, bid }: Wager) -> Self {
		Self {
			hand: WildHand(hand),
			bid,
		}
	}
}
// end::setup[]

// tag::pt1[]
fn pt1(mut wagers: Vec<Wager>) -> u32 {
	wagers.sort_by_key(|w| w.hand);
	wagers.iter().zip(1..).map(|(w, i)| w.bid * i).sum()
}
// end::pt1[]

// tag::pt2[]
fn pt2(wagers: Vec<Wager>) -> u32 {
	let mut wagers = wagers.into_iter().map(WildWager::from).collect::<Vec<_>>();
	wagers.sort_by_key(|w| w.hand);
	wagers.iter().zip(1..).map(|(w, i)| w.bid * i).sum()
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
			read_input(&read_file!("sample_input.txt")),
			(pt1, 6440),
			(pt2, 5905),
		);
	}

	#[test]
	fn test() {
		run_tests(
			read_input(&read_file!("input.txt")),
			(pt1, 248_113_761),
			(pt2, 246_285_222),
		);
	}
}
