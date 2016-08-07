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

	pub fn p<N: Num>(x: N, y: N) -> Point<N> {
		Point{x: x, y: y}
	}
}
