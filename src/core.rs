/* vim: set noexpandtab : */

use std::f64::INFINITY;
use std::ops::{Add,Sub,Mul,Div};
use std::clone::Clone;
use std::cmp::Ord;
use std::fmt::Debug;
use std::str::FromStr;

extern crate num;
use num::rational::BigRational;
use num::ToPrimitive;

#[derive(Debug,Clone,PartialEq)]

pub struct Point<N: Num> {
	pub x: N,
	pub y: N,
}

#[derive(Debug,Clone)]
pub struct Line<N: Num> {
	pub p1: Point<N>, 
	pub p2: Point<N>
}

#[derive(Debug,Clone)]
pub struct Polygon<N: Num> {
	is_hole: bool,
	square: bool,
	area: f64,
	pub points: Vec<Point<N>>,
}

#[derive(Debug,Clone)]
pub struct Shape<N: Num> {
	pub polys: Vec<Polygon<N>>,
}

#[derive(Debug,Clone)]
pub struct Skeleton<N: Num> {
  pub lines: Vec<Line<N>>,
}

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

impl<N: Num> Add for Point<N> {
	type Output=Self;
	fn add(self, other: Point<N>) -> Self {
		Point{x: self.x + other.x, y: self.y + other.y}
	}
}

//This assumes l0 -> l1 is clockwise, and l0.p2==l1.p1
pub fn dot<N: Num>(l0: &Line<N>, l1: &Line<N>) -> f64 {
	let p0 = &l0.p1;
	let p1 = &l0.p2;
	let p2 = &l1.p2;
	
	let dx1 = p1.x.to_f64() - p0.x.to_f64();
	let dx2 = p2.x.to_f64() - p1.x.to_f64();
	let dy1 = p1.y.to_f64() - p0.y.to_f64();
	let dy2 = p2.y.to_f64() - p1.y.to_f64();
	
	dx1*dy2 - dy1*dx2
	
}

pub fn is_convex<N: Num>(l0: &Line<N>, l1: &Line<N>) -> bool {
	dot(l0,l1) > 0.0
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

pub fn p_distance<N: Num>(p1: Point<N>, p2: Point<N>) -> f64 {
  let d = p1 - p2;
	return (d.x.to_f64().powi(2) + d.y.to_f64().powi(2)).sqrt();
}

impl<N: Num> Polygon<N> {
	pub fn new(points: Vec<Point<N>>) -> Polygon<N> {
		let (clockwise, area, square) = orient_area(&points);
		Polygon{points: points, area: area, square: square, is_hole: clockwise}
	}

	pub fn is_hole(&self) -> bool {
		self.is_hole
	}

	pub fn square(&self) -> bool {
		self.square
	}

	pub fn area(&self) -> f64 {
		self.area
	}

  pub fn longest_edge(self) -> (Point<N>, Point<N>) {
		let mut max: f64 = p_distance(self.points.last().unwrap().clone(), self.points[0].clone());
		let mut longest: (Point<N>, Point<N>) = (self.points.last().unwrap().clone(), self.points[0].clone());
		for edge in self.points.windows(2) {
		  let distance = p_distance(edge[0].clone(), edge[1].clone());
			if distance > max {
				max = distance;
				longest = (Point{x: edge[0].x.clone(), y: edge[0].y.clone()}, Point{x: edge[1].x.clone(), y: edge[1].y.clone()});
			}
		}

		return longest;
  }
}

impl<N: Num> Shape<N> {
	pub fn new(polys: Vec<Polygon<N>>) -> Shape<N> {
		Shape{polys: polys}
	}

	pub fn area(self) -> f64 {
		let mut a = 0.0;
		for p in self.polys {
			let sgn = if p.is_hole() { -1.0 } else { 1.0 };
			a += sgn * p.area();
		}
		a
	}
}

impl<N: Num> Skeleton<N> {
	pub fn new(lines: Vec<Line<N>>) -> Skeleton<N> {
    return Skeleton{lines: lines};
	}

  pub fn clone(self) -> Skeleton<N> {
    return Skeleton{lines: self.lines.clone()};
  }

  pub fn push(self, line: Line<N>) -> Skeleton<N> {
		let mut lines: Vec<Line<N>> = self.lines.clone();
    lines.push(line);
		return Skeleton{lines: lines};
  }

  pub fn lines(self) -> Vec<Line<N>> {
    return self.lines.clone();
  }

	pub fn len(self) -> usize {
	  return self.lines.len();
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
fn orient_area<N: Num>(points: &Vec<Point<N>>) -> (bool, f64, bool) {
	let n = points.len();
	let mut square = n == 4;
	let mut sum = half_tri_area(&points[n-1], &points[0]);
	let edge1angle = angle(&points[n-1], &points[0]);
	for edge in points.windows(2) {
		if square {
			let edgeangle = angle(&edge[0], &edge[1]);
			let cornerangle = (edge1angle - edgeangle).abs();
			if cornerangle % 90.0_f64.to_radians() > 0.0 {
				square = false;
			}
		}
		sum = sum + half_tri_area(&edge[0], &edge[1]);
	}
	let f = sum.to_f64();
	return (f >= 0.0, f.abs() / 2.0, square)
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
	fn test_is_convex_1(){
		let l1 = Line{ p1: p(1,1), p2: p(2,2) };
		let l2 = Line{ p1: p(2,2), p2: p(3,1) };
		
		assert!(is_convex(&l1,&l2)==false);
	}
	#[test]
	fn test_is_convex_2(){
		let l1 = Line{ p1: p(1,1), p2: p(2,2) };
		let l2 = Line{ p1: p(2,2), p2: p(3,4) };
		
		assert!(is_convex(&l1,&l2)==true);
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
		assert!(Polygon::new(vec!(p(1, 1), p(1, 2), p(2, 2), p(2, 1))).is_hole());
	}

	#[test]
	fn test_area() {
		assert_eq!(1.0, Polygon::new(vec!(p(0, 0), p(1, 0), p(1, 1), p(0, 1))).area());
		assert_eq!(1.0, Polygon::new(vec!(p(0, 0), p(0, 1), p(1, 1), p(1, 0))).area());
		let p22 = Polygon::new(vec!(p(0, 0), p(2, 0), p(2, 2), p(0, 2)));
		assert_eq!(4.0, p22.area());
		let p44 = Polygon::new(vec!(p(0, 0), p(4, 0), p(4, 4), p(0, 4)));
		let hole12 = Polygon::new(vec!(p(1, 1), p(1, 2), p(2, 2), p(2, 1)));
		assert!(hole12.is_hole());
		assert_eq!(15.0, Shape::new(vec!(p44, hole12)).area());
	}

	#[test]
	fn test_longest() {
		assert_eq!((p(1,0), p(2,2)), Polygon::new(vec!(p(0, 0), p(1, 0), p(2, 2), p(0, 1))).longest_edge());
	}

	#[test]
	fn test_float() {
		assert_eq!(0.5f64, "4328029871649615121465353437184/8656059743299229793415925725865".parse::<BigRational>().unwrap().to_f64());
		assert_eq!(0.25f64, "1/4".parse::<BigRational>().unwrap().to_f64());
		assert_eq!(1.1f64, "11/10".parse::<BigRational>().unwrap().to_f64());
	}

}
