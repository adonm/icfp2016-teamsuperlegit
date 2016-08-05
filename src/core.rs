use std::f64::INFINITY;
use std::ops::{Add,Sub,Mul,Div};
use std::cmp::Ord;
use std::fmt::Debug;
use std::str::FromStr;

extern crate num;
use num::bigint::{BigInt,Sign};
use num::rational::BigRational;
use num::pow::pow;
use num::{ToPrimitive,Zero};

pub trait ToF64 {
	fn to_f64(self) -> f64;
}

impl ToF64 for i32 {
	fn to_f64(self) -> f64 { self as f64 }
}

fn abs<'a>(r: &'a BigInt) -> BigInt {
	if r.sign() == Sign::Minus { -r } else { r.clone() }
}

fn signum(r: &BigRational) -> f64 {
	match (r.numer().sign(), r.denom().sign()) {
	(n, _) if n == Sign::NoSign => 0.0,
	(n, d) if (n == Sign::Minus) ^ (d == Sign::Minus) => -1.0,
	_ => 1.0,
	}
}

impl ToF64 for BigRational {
	fn to_f64(self) -> f64 {
		// BUG converts very large negatives to positive infinity
		self.numer().to_f64().unwrap_or(INFINITY) / self.denom().to_f64().unwrap_or(1.0)
	}
}

pub trait Num: Add + Sub + Mul + Div + Sized + FromStr + Debug + Ord + ToF64 {}
impl<N> Num for N where N: Add + Sub + Mul + Div + Sized + FromStr + Debug + Ord + ToF64 {}

#[derive(Debug,PartialEq)]
pub struct Point<T: Num> {
	pub x: T,
	pub y: T,
}

#[derive(Debug)]
pub struct Line<T: Num>(pub Point<T>, pub Point<T>);

pub struct Polygon<T: Num> {
	pub points: Vec<Point<T>>,
}

pub type Shape<T> = Vec<Polygon<T>>;

pub type Skeleton<T> = Vec<Line<T>>;

impl<T: Num> Add for Point<T> where T: Add<Output=T> {
	type Output=Point<T>;
	fn add(self, other: Point<T>) -> Point<T> {
		Point{x: self.x + other.x, y: self.y + other.y}
	}
}

impl<T: Num> Sub for Point<T> where T: Sub<Output=T> {
	type Output=Point<T>;
	fn sub(self, other: Point<T>) -> Point<T> {
		Point::<T>{x: self.x - other.x, y: self.y - other.y}
	}
}

impl<T: Num> Polygon<T> {
	pub fn new(points: Vec<Point<T>>) -> Polygon<T> {
		Polygon{points: points}
	}

	pub fn is_hole(self) -> bool {
		is_clockwise(self)
	}
}

/*fn angle<T: Num>(p0: &Point<T>, p1: &Point<T>) -> f64 where T: Sub<Output=T> {
	// `p1 - p0` doesn't work for some reason???
	let dx = p1.x - p0.x;
	let dy = p1.y - p0.y;
	return dx.to_f64() / dy.to_f64();
}*/

fn is_clockwise<T: Num>(poly: Polygon<T>) -> bool {
	for pair in poly.points.windows(2) {
		//angle(&pair[0], &pair[1]);
	}
	false
}

#[cfg(test)]
mod tests {
	use super::*;
	extern crate num;
	use self::num::rational::BigRational;

	#[test]
	fn test_add() {
		assert_eq!(Point::<i32>{x: 5, y: 7}, Point{x: 2, y: 4} + Point{x: 3, y: 3});
	}

	#[test]
	fn test_float() {
		assert_eq!(0.5f64, "4328029871649615121465353437184/8656059743299229793415925725865".parse::<BigRational>().unwrap().to_f64());
		assert_eq!(0.25f64, "1/4".parse::<BigRational>().unwrap().to_f64());
		assert_eq!(1.1f64, "11/10".parse::<BigRational>().unwrap().to_f64());
	}
}
