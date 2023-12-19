// tag::setup[]
use crate::{
	enum_map::EnumMap,
	error::{AocResult, ToResultDefaultErr},
	read_file, regex, Answer, AocError,
};
use indexmap::IndexMap;
use std::{ops::ControlFlow, str::FromStr};
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, EnumString};

fn ans_for_input(input: &str) -> Answer<i64, i64> {
	let (rules, inputs) = read_input(input).unwrap();
	(19, (pt1((&*inputs, &rules)), pt2((&*inputs, &rules)))).into()
}

pub fn ans() -> Answer<i64, i64> {
	ans_for_input(&read_file!("input.txt"))
}

fn read_input(input: &str) -> AocResult<(IndexMap<String, RuleSet>, Vec<AttrMap>)> {
	let mut rules = IndexMap::new();
	let mut inputs = Vec::new();

	let mut lines = input.lines();
	for line in lines.by_ref() {
		if line.trim().is_empty() {
			break;
		}

		let caps = regex!(r"(?<name>\w+)\{(?<rules_str>.*)\}")
			.captures(line)
			.to_result()?;

		let name = caps.name("name").to_result()?.as_str().to_owned();
		let rules_str = caps.name("rules_str").to_result()?.as_str();

		let mut curr_rules = Vec::new();
		let mut final_rule = None;

		for rule_m in regex!("[^,]+").find_iter(rules_str) {
			match rule_m.as_str().parse() {
				Ok(rule) => curr_rules.push(rule),
				Err(_) => final_rule = Some(rule_m.as_str().parse()?),
			}
		}

		assert!(rules
			.insert(
				name,
				RuleSet {
					rules: curr_rules,
					otherwise: final_rule.unwrap(),
				},
			)
			.is_none());
	}

	for line in lines {
		let mut attrs = AttrMap::default();
		for caps in regex!(r"(?<attr>\w+)=(?<value>\d+)").captures_iter(line) {
			let [attr, value] = ["attr", "value"]
				.try_map(|name| AocResult::Ok(caps.name(name).to_result()?.as_str()))?;
			let attr = attr.parse::<Attr>()?;
			let value = value.parse()?;
			attrs[attr] = value;
		}

		inputs.push(attrs);
	}

	Ok((rules, inputs))
}

type AttrMap = EnumMap<{ Attr::COUNT }, Attr, i64>;

#[derive(Debug, Clone, Copy, EnumCount, EnumIter, EnumString)]
#[strum(ascii_case_insensitive)]
#[repr(u8)]
enum Attr {
	X,
	M,
	A,
	S,
}

impl From<Attr> for usize {
	fn from(attr: Attr) -> Self {
		attr as _
	}
}

#[derive(Debug, Clone, Copy, EnumString)]
enum Comparison {
	#[strum(serialize = "<")]
	Lt,
	#[strum(serialize = ">")]
	Gt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
	Accept,
	Reject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Action(ControlFlow<Outcome, String>);

impl FromStr for Action {
	type Err = !;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use Outcome::*;

		Ok(Self(match s {
			"A" => ControlFlow::Break(Accept),
			"R" => ControlFlow::Break(Reject),
			_ => ControlFlow::Continue(s.to_owned()),
		}))
	}
}

#[derive(Debug, Clone, Copy)]
struct Condition {
	attr: Attr,
	cmp: Comparison,
	value: i64,
}

#[derive(Debug)]
struct Rule {
	condition: Condition,
	action: Action,
}

impl FromStr for Rule {
	type Err = AocError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let caps = regex!(r"(?<attr>\w+)(?<cmp>>|<)(?<value>\d+):(?<action>\w+)")
			.captures(s)
			.to_result()?;
		let [attr, cmp, value, action] = ["attr", "cmp", "value", "action"]
			.try_map(|name| AocResult::Ok(caps.name(name).to_result()?.as_str()))?;

		let attr = attr.parse()?;
		let cmp = cmp.parse()?;
		let value = value.parse()?;
		let action = action.parse()?;

		Ok(Self {
			condition: Condition { attr, cmp, value },
			action,
		})
	}
}

#[derive(Debug)]
struct RuleSet {
	rules: Vec<Rule>,
	otherwise: Action,
}

// end::setup[]

// tag::pt1[]
impl Rule {
	fn applied_to(&self, attrs: AttrMap) -> Option<&Action> {
		let Self { condition, action } = self;
		let &Condition { attr, cmp, value } = condition;

		let x = attrs[attr];
		let applies = match cmp {
			Comparison::Lt => x < value,
			Comparison::Gt => x > value,
		};
		applies.then_some(action)
	}
}

fn pt1((inputs, rules): (&[AttrMap], &IndexMap<String, RuleSet>)) -> i64 {
	let init_rule_name = "in";

	inputs
		.iter()
		.map(|&inp| {
			let mut rule_name = init_rule_name;

			loop {
				let rule_set = &rules[rule_name];

				let Action(action) = rule_set
					.rules
					.iter()
					.find_map(|rule| rule.applied_to(inp))
					.unwrap_or(&rule_set.otherwise);

				match action {
					ControlFlow::Continue(name) => rule_name = &name,
					ControlFlow::Break(outcome) => {
						return match outcome {
							Outcome::Accept => inp.into_array().into_iter().sum(),
							Outcome::Reject => 0,
						};
					}
				}
			}
		})
		.sum()
}
// end::pt1[]

// tag::pt2[]
#[derive(Debug, Clone, Copy)]
struct ConditionEq {
	condition: Condition,
	eq_allowed: bool,
}

impl std::ops::Not for ConditionEq {
	type Output = Self;

	fn not(self) -> Self::Output {
		use Comparison::*;

		let Self {
			condition: Condition { attr, cmp, value },
			eq_allowed,
		} = self;
		Self {
			condition: Condition {
				attr,
				cmp: match cmp {
					Lt => Gt,
					Gt => Lt,
				},
				value,
			},
			eq_allowed: !eq_allowed,
		}
	}
}

#[derive(Debug, Clone, Copy)]
struct Criteria(EnumMap<{ Attr::COUNT }, Attr, (i64, i64)>);

#[derive(Debug, Clone)]
struct Traversal<'a> {
	arrive_at: &'a str,
	with_criteria: Criteria,
}

impl Criteria {
	fn add_condition(&mut self, condition: ConditionEq) {
		let ConditionEq {
			condition: Condition { attr, cmp, value },
			eq_allowed,
		} = condition;
		let (low, high) = &mut self.0[attr];
		match (cmp, eq_allowed) {
			(Comparison::Lt, true) => *high = value.min(*high),
			(Comparison::Lt, false) => *high = (value - 1).min(*high),
			(Comparison::Gt, true) => *low = value.max(*low),
			(Comparison::Gt, false) => *low = (value + 1).max(*low),
		}
	}

	fn validity(self) -> Result<(), ()> {
		if self
			.0
			.into_array()
			.into_iter()
			.all(|(low, high)| low <= high)
		{
			Ok(())
		} else {
			Err(())
		}
	}
}

fn handle_action<'a>(
	paths: &mut Vec<Traversal<'a>>,
	initial_conditions: &mut Vec<Criteria>,
	Action(action): &'a Action,
	criteria: Criteria,
) {
	match action {
		ControlFlow::Continue(name) => paths.push(Traversal {
			arrive_at: name,
			with_criteria: criteria,
		}),
		ControlFlow::Break(Outcome::Accept) => initial_conditions.push(criteria),
		ControlFlow::Break(Outcome::Reject) => {}
	}
}

fn pt2((_, rules): (&[AttrMap], &IndexMap<String, RuleSet>)) -> i64 {
	// Strategy: start at "in" and just traverse, taking every bifurcation, looking for
	// the Accept states. The nice thing about this, as opposed to working backwards from
	// the Accept states to find our way to "in", is that the ranges are automatically
	// disjoint. (Going backwards probably has a shorter runtime but requires
	// disentagling any overlaps.)

	let mut paths = vec![Traversal {
		arrive_at: "in",
		with_criteria: Criteria(EnumMap::new([(1, 4000); 4])),
	}];
	let mut initial_conditions = Vec::new();

	while let Some(path) = paths.pop() {
		let Traversal {
			arrive_at,
			with_criteria: mut criteria,
		} = path;

		if criteria.validity().is_err() {
			continue;
		}

		let rule_set = &rules[arrive_at];
		for Rule { condition, action } in &rule_set.rules {
			let trigger_condition = ConditionEq {
				condition: *condition,
				eq_allowed: false,
			};

			let mut yes_criteria = criteria;
			yes_criteria.add_condition(trigger_condition);

			handle_action(&mut paths, &mut initial_conditions, action, yes_criteria);

			criteria.add_condition(!trigger_condition);
		}

		handle_action(
			&mut paths,
			&mut initial_conditions,
			&rule_set.otherwise,
			criteria,
		);
	}

	initial_conditions
		.into_iter()
		.map(|ranges| {
			ranges
				.0
				.into_array()
				.into_iter()
				.map(|(low, high)| (high - low + 1).max(0))
				.product::<i64>()
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
		let (rules, inputs) = read_input(&read_file!("sample_input.txt")).unwrap();
		run_tests((&*inputs, &rules), (pt1, 19114), (pt2, 167_409_079_868_000));
	}

	#[test]
	fn test() {
		let (rules, inputs) = read_input(&read_file!("input.txt")).unwrap();

		run_tests(
			(&*inputs, &rules),
			(pt1, 492_702),
			(pt2, 138_616_621_185_978),
		);
	}
}
