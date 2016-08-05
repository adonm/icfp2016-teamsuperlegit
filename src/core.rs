use std::f64::INFINITY;
use std::ops::{Add,Sub,Mul,Div};
use std::clone::Clone;
use std::cmp::Ord;
use std::fmt::Debug;
use std::str::FromStr;

extern crate num;
use num::bigint::{BigInt,Sign};
use num::rational::BigRational;
use num::pow::pow;
use num::{ToPrimitive,Zero};
use num::Float;

pub trait ToF64 {
	fn to_f64(&self) -> f64;
}

impl ToF64 for i32 {
	fn to_f64(&self) -> f64 { *self as f64 }
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
	fn to_f64(&self) -> f64 {
		// BUG converts very large negatives to positive infinity
		self.numer().to_f64().unwrap_or(INFINITY) / self.denom().to_f64().unwrap_or(1.0)
	}
}

pub trait Num: Add<Output=Self> + Sub<Output=Self> + Mul + Div + Sized + FromStr + Debug + Ord + ToF64 + Clone {}
impl<N> Num for N where N: Add<Output=N> + Sub<Output=N> + Mul + Div + Sized + FromStr + Debug + Ord + ToF64 + Clone {}

#[derive(Debug,PartialEq)]
pub struct Point<T: Num> {
	pub x: T,
	pub y: T,
}

#[derive(Debug)]
pub struct Line<T: Num>(pub Point<T>, pub Point<T>);

#[derive(Debug)]
pub struct Polygon<T: Num> {
	is_hole: bool,
	pub points: Vec<Point<T>>,
}

pub type Shape<T> = Vec<Polygon<T>>;

pub type Skeleton<T> = Vec<Line<T>>;

impl<T: Num> Add for Point<T> where T: Add<Output=T> {
	type Output=Self;
	fn add(self, other: Point<T>) -> Self {
		Point{x: self.x + other.x, y: self.y + other.y}
	}
}

impl<T: Num> Sub for Point<T> where T: Sub<Output=T> {
	type Output=Self;
	fn sub(self, other: Point<T>) -> Self {
		Point::<T>{x: self.x - other.x, y: self.y - other.y}
	}
}

impl<T: Num> Polygon<T> where T: Sub<Output=T>+Add<Output=T> {
	pub fn new(points: Vec<Point<T>>) -> Polygon<T> {
		let clockwise = is_clockwise(&points);
		Polygon{points: points, is_hole: clockwise}
	}

	pub fn is_hole(self) -> bool {
		self.is_hole
	}
}

pub fn angle<T: Num>(p0: &Point<T>, p1: &Point<T>) -> f64 where T: Sub<Output=T> {
	let dx = p1.x.clone() - p0.x.clone();
	let dy = p1.y.clone() - p0.y.clone();
	return dx.to_f64().atan2(dy.to_f64());
}

fn is_clockwise<T: Num>(points: &Vec<Point<T>>) -> bool where T: Sub<Output=T>+Add<Output=T> {
	let n = points.len();
	let mut sum = (points[0].x.clone() - points[n-1].x.clone()).to_f64() * (points[0].y.clone() + points[n-1].y.clone()).to_f64();
	for edge in points.windows(2) {
		sum += (edge[1].x.clone() - edge[0].x.clone()).to_f64() * (edge[1].y.clone() + edge[0].y.clone()).to_f64();
	}
	sum >= 0.0
}

#[cfg(test)]
mod tests {
	use super::*;
	extern crate num;
	use self::num::rational::BigRational;
	use num::Float;

	fn p(x: i32, y: i32) -> Point<i32> {
		Point{x: x, y: y}
	}

	#[test]
	fn test_ops() {
		assert_eq!(p(5, 7), p(2, 4) + p(3, 3));
		assert_eq!(p(-1, 1), p(2, 4) - p(3, 3));
	}

	#[test]
	fn test_angle() {
		assert_eq!(45.0.to_radians(), angle(&p(0, 0), &p(1, 1)));
		assert_eq!(-135.0.to_radians(), angle(&p(1, 1), &p(0, 0)));
	}

	#[test]
	fn test_clockwise() {
		assert!(!Polygon::new(vec!(p(0, 0), p(1, 0), p(1, 1), p(0, 1))).is_hole());
		assert!(Polygon::new(vec!(p(0, 0), p(0, 1), p(1, 1), p(1, 0))).is_hole());
	}

	#[test]
	fn test_float() {
		assert_eq!(0.5f64, "4328029871649615121465353437184/8656059743299229793415925725865".parse::<BigRational>().unwrap().to_f64());
		assert_eq!(0.25f64, "1/4".parse::<BigRational>().unwrap().to_f64());
		assert_eq!(1.1f64, "11/10".parse::<BigRational>().unwrap().to_f64());
	}
}
