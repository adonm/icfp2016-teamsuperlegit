/* vim: set noexpandtab : */

mod generic;
mod geom;
mod solve;

pub use self::generic::*;
pub use self::geom::*;
pub use self::solve::*;

/* Test helper functions go here. */
#[cfg(test)]
mod tests {
	use super::*;
	extern crate num;
	pub use self::num::rational::BigRational;
	pub use num::Float;

	pub fn p(x: i32, y: i32) -> Point<i32> {
		Point{x: x, y: y}
	}

	pub fn p64(x: f64, y: f64) -> Point<f64> {
		Point{x: x, y: y}
	}
    
    pub fn pNum<N:Num>(x: N, y:N) -> Point<N> {
        Point{x: x, y: y}
    }
}
