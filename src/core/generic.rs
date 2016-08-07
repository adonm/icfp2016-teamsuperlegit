// Num and other such generic nonsense
use super::*;

use std::clone::Clone;
use std::cmp::{Ord,Ordering,PartialOrd};
use std::fmt::{Debug,Display};
use std::fmt;
use std::f64::INFINITY;
use std::ops::{Add,Sub,Mul,Div,Neg};
use std::str::FromStr;

extern crate num;
use num::rational::BigRational;
use num::ToPrimitive;
use self::num::bigint::BigInt;

pub trait SuperLegit {
	fn to_f64(&self) -> f64;
    fn to_rat(&self) -> BigRational;
	fn from_f64(f64) -> Self;
	fn zero() -> Self;
	fn one() -> Self;
	fn abs(&self) -> Self;
}

impl SuperLegit for i32 {
	fn to_f64(&self) -> f64 { *self as f64 }
    fn to_rat(&self) -> BigRational { BigRational::from_f64(self.clone() as f64) }
	fn from_f64(f: f64) -> Self { f as i32 }
	fn zero() -> Self { 0 }
	fn one() -> Self { 1 }
	fn abs(&self) -> Self { if self < &0 { -self } else { *self }}
}

impl SuperLegit for f64 {
	fn to_f64(&self) -> f64 { *self }
    fn to_rat(&self) -> BigRational { BigRational::from_f64(self.clone()) }
	fn from_f64(f: f64) -> Self { f }
	fn zero() -> Self { 0.0 }
	fn one() -> Self { 1.0 }
	fn abs(&self) -> Self { if self < &0.0 { -self } else { *self } }
}

pub fn divide<N:Num>( a: N, b: N ) -> Option<N> {
    if b == N::from_f64(0.0){
        None
    } else {
        Some( a / b )
    }
    
}


pub fn eq_eps_custom<N: Num, M: Num>(a: &N, b: &M, eps: f64) -> bool {
  return (b.clone().to_f64() - a.clone().to_f64()).abs() < eps;
}



pub fn find_close_rational_point<N: Num>(p: Point<N>) -> Point<BigRational> {
    
    let x = match find_close_rational(p.clone().x) {
        Some(z) => z,
        None => p.clone().x.to_rat()
    };
    let y = match find_close_rational(p.clone().y) {
        Some(z) => z,
        None => p.clone().y.to_rat()
    };
    
	let p = Point{
		x: x,
		y: y
	};
    
	p
}


pub fn find_close_rational<N:Num>( x: N ) -> Option<BigRational> {
    
    let do_i_do = false;
    
    if do_i_do ==false {
        return None;
    }
    
    for i in 0..100 {
        
        for j in 1..100 {
            
            let y = BigRational::new(BigInt::from(i), BigInt::from(j));
            if eq_eps_custom(&x,&y, 0.0000000000001) {
                return Some(y);
            }
            
            
            let z = BigRational::new(BigInt::from(-i), BigInt::from(j));
            if eq_eps_custom(&x,&z, 0.0000000000001) {
                return Some(z);
            }
            
        }
        
    }
    
    None
    
}


impl SuperLegit for BigRational {
	fn to_f64(&self) -> f64 {
		// BUG converts very large negatives to positive infinity
		self.numer().to_f64().unwrap_or(INFINITY) / self.denom().to_f64().unwrap_or(1.0)
	}
    
    fn to_rat(&self) -> BigRational { self.clone() }

	fn from_f64(f: f64) -> Self { 
		// handle impossible floats
		BigRational::from_float(f).unwrap_or(num::one::<BigRational>())
	}

	fn zero() -> Self { num::zero::<BigRational>() }
	fn one() -> Self { num::one::<BigRational>() }
	fn abs(&self) -> Self { if self < &Self::zero() { -self } else { self.clone() }}
}

pub trait Num: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Neg<Output=Self> + Sized + FromStr + Debug + Display + PartialOrd + PartialEq + Clone + SuperLegit {}
impl<N> Num for N where N: Add<Output=N> + Sub<Output=N> + Mul<Output=N> + Div<Output=N> + Neg<Output=N> + Sized + FromStr + Debug + Display + PartialOrd + PartialEq + Clone + SuperLegit {}

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
    use super::*;

    extern crate num;
    use num::ToPrimitive;
    use self::num::bigint::BigInt;
	#[test]
	fn test_ops() {
		assert_eq!(p(5, 7), p(2, 4) + p(3, 3));
		assert_eq!(p(-1, 1), p(2, 4) - p(3, 3));
	}

    
	#[test]
    fn find_close_rat_test(){
        
        let n = find_close_rational( 0.3333333333333333333333 ).unwrap();
        assert_eq!(n,BigRational::new(BigInt::from(1), BigInt::from(3)));
        
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
