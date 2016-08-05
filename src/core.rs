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

pub trait Num: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div + Sized + FromStr + Debug + Ord + ToF64 + Clone {}
impl<N> Num for N where N: Add<Output=N> + Sub<Output=N> + Mul<Output=N> + Div + Sized + FromStr + Debug + Ord + ToF64 + Clone {}

#[derive(Debug,PartialEq)]
pub struct Point<N: Num> {
	pub x: N,
	pub y: N,
}

#[derive(Debug)]
pub struct Line<N: Num> {
    pub p1: Point<N>, 
    pub p2: Point<N>
}

#[derive(Debug)]
pub struct Polygon<N: Num> {
	is_hole: bool,
	area: f64,
	pub points: Vec<Point<N>>,
}

pub type Shape<N> = Vec<Polygon<N>>;

pub type Skeleton<N> = Vec<Line<N>>;

impl<N: Num> Add for Point<N> {
	type Output=Self;
	fn add(self, other: Point<N>) -> Self {
		Point{x: self.x + other.x, y: self.y + other.y}
	}
}

impl<'a, N: Num> Add for &'a Point<N> {
	type Output=Point<N>;
	fn add(self, other: &Point<N>) -> Point<N> {
		Point{x: self.x.clone() + other.x.clone(), y: self.y.clone() + other.y.clone()}
	}
}

impl<N: Num> Sub for Point<N> {
	type Output=Self;
	fn sub(self, other: Point<N>) -> Self {
		Point{x: self.x - other.x, y: self.y - other.y}
	}
}

impl<'a, N: Num> Sub for &'a Point<N> {
	type Output=Point<N>;
	fn sub(self, other: &Point<N>) -> Point<N> {
		Point{x: self.x.clone() - other.x.clone(), y: self.y.clone() - other.y.clone()}
	}
}

impl<N: Num> Polygon<N> {
	pub fn new(points: Vec<Point<N>>) -> Polygon<N> {
		let (clockwise, area) = orient_area(&points);
		Polygon{points: points, area: area, is_hole: clockwise}
	}

	pub fn is_hole(self) -> bool {
		self.is_hole
	}

	pub fn area(self) -> f64 {
		self.area
	}
}

pub fn angle<'a, N: Num>(p0: &'a Point<N>, p1: &'a Point<N>) -> f64 {
	let d = p1 - p0;
	return d.x.to_f64().atan2(d.y.to_f64());
}

fn half_tri_area<'a, N: Num>(p0: &'a Point<N>, p1: &'a Point<N>) -> N {
	(p1.x.clone() - p0.x.clone()) * (p1.y.clone() + p0.y.clone())
}

/* returns a tuple where the first element is true if the poly points are in clockwise order,
** and the second element is the area contained within. thx to:
** http://stackoverflow.com/questions/1165647/how-to-determine-if-a-list-of-polygon-points-are-in-clockwise-order */
fn orient_area<N: Num>(points: &Vec<Point<N>>) -> (bool, f64) {
	let n = points.len();
	let mut sum = half_tri_area(&points[n-1], &points[0]);
	for edge in points.windows(2) {
		sum = sum + half_tri_area(&edge[0], &edge[1]);
	}
	let f = sum.to_f64();
	return (f >= 0.0, f.abs() / 2.0)
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
	fn test_area() {
		assert_eq!(1.0, Polygon::new(vec!(p(0, 0), p(1, 0), p(1, 1), p(0, 1))).area());
		assert_eq!(1.0, Polygon::new(vec!(p(0, 0), p(0, 1), p(1, 1), p(1, 0))).area());
		assert_eq!(4.0, Polygon::new(vec!(p(0, 0), p(2, 0), p(2, 2), p(0, 2))).area());
	}

	#[test]
	fn test_float() {
		assert_eq!(0.5f64, "4328029871649615121465353437184/8656059743299229793415925725865".parse::<BigRational>().unwrap().to_f64());
		assert_eq!(0.25f64, "1/4".parse::<BigRational>().unwrap().to_f64());
		assert_eq!(1.1f64, "11/10".parse::<BigRational>().unwrap().to_f64());
	}
}
