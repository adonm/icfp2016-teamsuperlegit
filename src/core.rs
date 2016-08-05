use std::f64::INFINITY;
use std::ops::{Add,Sub,Mul,Div};
use std::clone::Clone;
use std::cmp::Ord;
use std::fmt::Debug;
use std::str::FromStr;

extern crate num;
use num::rational::BigRational;
use num::ToPrimitive;

pub trait ToF64 {
	fn to_f64(&self) -> f64;
}

impl ToF64 for i32 {
	fn to_f64(&self) -> f64 { *self as f64 }
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
pub struct Point<N: Num> {
	pub x: N,
	pub y: N,
}

#[derive(Debug)]
pub struct Line<N: Num>(pub Point<N>, pub Point<N>);

#[derive(Debug)]
pub struct Polygon<N: Num> {
	is_hole: bool,
	pub points: Vec<Point<N>>,
}

pub type Shape<N> = Vec<Polygon<N>>;

pub type Skeleton<N> = Vec<Line<N>>;

impl<N: Num> Add for Point<N> where N: Add<Output=N> {
	type Output=Self;
	fn add(self, other: Point<N>) -> Self {
		Point{x: self.x + other.x, y: self.y + other.y}
	}
}

impl<N: Num> Sub for Point<N> where N: Sub<Output=N> {
	type Output=Self;
	fn sub(self, other: Point<N>) -> Self {
		Point::<N>{x: self.x - other.x, y: self.y - other.y}
	}
}

impl<N: Num> Polygon<N> where N: Sub<Output=N>+Add<Output=N> {
	pub fn new(points: Vec<Point<N>>) -> Polygon<N> {
		let clockwise = is_clockwise(&points);
		Polygon{points: points, is_hole: clockwise}
	}

	pub fn is_hole(self) -> bool {
		self.is_hole
	}
}

pub fn angle<N: Num>(p0: &Point<N>, p1: &Point<N>) -> f64 where N: Sub<Output=N> {
	let dx = p1.x.clone() - p0.x.clone();
	let dy = p1.y.clone() - p0.y.clone();
	return dx.to_f64().atan2(dy.to_f64());
}

fn is_clockwise<N: Num>(points: &Vec<Point<N>>) -> bool where N: Sub<Output=N>+Add<Output=N> {
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
