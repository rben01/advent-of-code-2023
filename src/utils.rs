#![allow(dead_code)]

use num::{CheckedAdd, CheckedSub, Num, One, Zero};
use std::ops::RangeBounds;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Direction {
	N,
	S,
	E,
	W,
}

fn get_adjacent<
	'a,
	X: 'a + Copy + Num + Zero + One + CheckedAdd + CheckedSub + Ord,
	Y: 'a + Copy + Num + Zero + One + CheckedAdd + CheckedSub + Ord,
	XRangeT: 'a + RangeBounds<X>,
	YRangeT: 'a + RangeBounds<Y>,
>(
	pos_xy: (X, Y),
	x_bounds: XRangeT,
	y_bounds: YRangeT,
	relative_dirs: impl 'a + IntoIterator<Item = (i8, i8)>,
) -> impl 'a + Iterator<Item = (X, Y)> {
	let (x, y) = pos_xy;
	relative_dirs.into_iter().filter_map(move |(dx, dy)| {
		let new_x = match dx.signum() {
			..=-1 => x.checked_sub(&X::one())?,
			0 => x,
			1.. => x.checked_add(&X::one())?,
		};
		let new_y = match dy.signum() {
			..=-1 => y.checked_sub(&Y::one())?,
			0 => y,
			1.. => y.checked_add(&Y::one())?,
		};

		(x_bounds.contains(&new_x) && y_bounds.contains(&new_y)).then_some((new_x, new_y))
	})
}

pub(crate) fn get_nsew_adjacent<
	'a,
	X: 'a + Copy + Num + Zero + One + CheckedAdd + CheckedSub + Ord,
	Y: 'a + Copy + Num + Zero + One + CheckedAdd + CheckedSub + Ord,
	XRangeT: 'a + RangeBounds<X>,
	YRangeT: 'a + RangeBounds<Y>,
>(
	pos_xy: (X, Y),
	x_bounds: XRangeT,
	y_bounds: YRangeT,
) -> impl 'a + Iterator<Item = (X, Y)> {
	get_adjacent(
		pos_xy,
		x_bounds,
		y_bounds,
		[(-1, 0), (1, 0), (0, -1), (0, 1)],
	)
}

pub(crate) fn get_nsew_diag_adjacent<
	'a,
	X: 'a + Copy + Num + Zero + One + CheckedAdd + CheckedSub + Ord,
	Y: 'a + Copy + Num + Zero + One + CheckedAdd + CheckedSub + Ord,
	XRangeT: 'a + RangeBounds<X>,
	YRangeT: 'a + RangeBounds<Y>,
>(
	pos_xy: (X, Y),
	x_bounds: XRangeT,
	y_bounds: YRangeT,
) -> impl 'a + Iterator<Item = (X, Y)> {
	get_adjacent(
		pos_xy,
		x_bounds,
		y_bounds,
		[
			(1, 0),   // →
			(1, 1),   // ↗︎
			(0, 1),   // ↑
			(-1, 1),  // ↖︎
			(-1, 0),  // ←
			(-1, -1), // ↙︎
			(0, -1),  // ↓
			(1, -1),  // ↘︎
		],
	)
}
