// Num and other such generic nonsense
use super::*;

use std::clone::Clone;
use std::cmp::{Ord,Ordering,PartialOrd};
use std::fmt::{Debug,Display};
use std::fmt;
use std::f64::INFINITY;
use std::ops::{Add,Sub,Mul,Div,Neg};
use std::str::FromStr;
use std::panic;

extern crate num;
use num::rational::BigRational;
use num::ToPrimitive;

pub trait SuperLegit {
	fn to_f64(&self) -> f64;
	fn from_f64(f64) -> Self;
	fn zero() -> Self;
	fn one() -> Self;
}

impl SuperLegit for i32 {
	fn to_f64(&self) -> f64 { *self as f64 }
	fn from_f64(f: f64) -> Self { f as i32 }
	fn zero() -> Self { 0 }
	fn one() -> Self { 1 }
}

impl SuperLegit for f64 {
	fn to_f64(&self) -> f64 { *self }
	fn from_f64(f: f64) -> Self { f }
	fn zero() -> Self { 0.0 }
	fn one() -> Self { 1.0 }
}

pub fn divide<N:Num>( a: N, b: N ) -> Option<N> {
    if b == N::from_f64(0.0){
        None
    } else {
        Some( a / b )
    }
    
}

impl SuperLegit for BigRational {
	fn to_f64(&self) -> f64 {
		// BUG converts very large negatives to positive infinity
		self.numer().to_f64().unwrap_or(INFINITY) / self.denom().to_f64().unwrap_or(1.0)
	}

	fn from_f64(f: f64) -> Self { 
		// handle impossible floats
		BigRational::from_float(f).unwrap_or(num::one::<BigRational>())
	}

	fn zero() -> Self { num::zero::<BigRational>() }
	fn one() -> Self { num::one::<BigRational>() }
}

pub trait Num: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Neg<Output=Self> + Sized + FromStr + Debug + Display + PartialOrd + PartialEq + Clone + panic::RefUnwindSafe + SuperLegit {}
impl<N> Num for N where N: Add<Output=N> + Sub<Output=N> + Mul<Output=N> + Div<Output=N> + Neg<Output=N> + Sized + FromStr + Debug + Display + PartialOrd + PartialEq + Clone + panic::RefUnwindSafe + SuperLegit {}

impl<N: Num> Add for Point<N> {
	type Output=Self;
	fn add(self, other: Point<N>) -> Self {
		Point{x: self.x + other.x, y: self.y + other.y}
	}
}

impl<'a, 'b, N: Num> Add<&'b Point<N>> for &'a Point<N> {
	type Output=Point<N>;
	fn add(self, other: &'b Point<N>) -> Point<N> {
		Point{x: self.x.clone() + other.x.clone(), y: self.y.clone() + other.y.clone()}
	}
}

impl<'a, N: Num> Add<&'a Point<N>> for Point<N> {
	type Output=Point<N>;
	fn add(self, other: &'a Point<N>) -> Point<N> {
		Point{x: self.x + other.x.clone(), y: self.y + other.y.clone()}
	}
}

impl<'a, N: Num> Add<Point<N>> for &'a Point<N> {
	type Output=Point<N>;
	fn add(self, other: Point<N>) -> Point<N> {
		Point{x: other.x + self.x.clone(), y: other.y + self.y.clone()}
	}
}

impl<N: Num> Sub for Point<N> {
	type Output=Self;
	fn sub(self, other: Point<N>) -> Self {
		Point{x: self.x - other.x, y: self.y - other.y}
	}
}

impl<'a, 'b, N: Num> Sub<&'b Point<N>> for &'a Point<N> {
	type Output=Point<N>;
	fn sub(self, other: &'b Point<N>) -> Point<N> {
		Point{x: self.x.clone() - other.x.clone(), y: self.y.clone() - other.y.clone()}
	}
}

impl<'a, N: Num> Sub<&'a Point<N>> for Point<N> {
	type Output=Point<N>;
	fn sub(self, other: &'a Point<N>) -> Point<N> {
		Point{x: self.x - other.x.clone(), y: self.y - other.y.clone()}
	}
}

impl<'a, N: Num> Sub<Point<N>> for &'a Point<N> {
	type Output=Point<N>;
	fn sub(self, other: Point<N>) -> Point<N> {
		Point{x: self.x.clone() - other.x, y: self.y.clone() - other.y }
	}
}

impl<N: Num> Display for Point<N> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match f.precision() {
			Some(p) => write!(f, "{:.prec$},{:.prec$}", self.x.to_f64(), self.y.to_f64(), prec=p),
			_ => write!(f, "{},{}", self.x, self.y),
		}
	}
}

pub fn eq_eps<N: Num>(a: &N, b: &N) -> bool {
  return (b.clone() - a.clone()).to_f64().abs() < 0.000001;
}

/* can't derive(Eq) because we support Point<f64> and f64 doesn't provide a total ordering
 * (╯°□°)╯ sᴎɐᴎ */
impl<N: Num> Eq for Point<N> {
}

impl<N: Num> PartialEq for Point<N> {
  fn eq(&self, other: &Point<N>) -> bool {
    eq_eps(&self.x, &other.x) && eq_eps(&self.y, &other.y)
  }
}

impl<N: Num> Ord for Point<N> {
	fn cmp(&self, other: &Point<N>) -> Ordering {
		if self < other { Ordering::Less }
		else if self > other { Ordering::Greater }
		else { Ordering::Equal }
	}
}

impl<N: Num> Display for Line<N> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} -> {}", self.p1, self.p2)
	}
}


#[cfg(test)]
mod tests {
	use super::super::tests::*;

	#[test]
	fn test_ops() {
		assert_eq!(p(5, 7), p(2, 4) + p(3, 3));
		assert_eq!(p(-1, 1), p(2, 4) - p(3, 3));
	}

	#[test]
	fn test_commutivity() {
		let (p1, p2) = (p(1.0, 1.5), p(1.25, 2.5));
		assert_eq!(p(2.25, 4.0), &p1 + &p2);
		assert_eq!(p(2.25, 4.0), &p2 + &p1);
		assert_eq!(p(3.5, 6.5), &p1 + &(p2.scale(2.0)));
		assert_eq!(p(3.25, 5.5), &(p1.scale(2.0)) + &p2);

		assert_eq!(p(0.25, 1.0), p2 - &p1);
	}

	#[test]
	fn test_format() {
		assert_eq!("1.1234,2.2345", format!("{}", p(1.1234, 2.2345)));
		assert_eq!("1.1,2.2", format!("{:.1}", p(1.1234, 2.2345)));
	}
}
