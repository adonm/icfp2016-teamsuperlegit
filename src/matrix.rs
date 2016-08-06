use std::ops::{Index,Mul};

use core::*;

pub struct Matrix33<N: Num> {
	points: [N; 9],
}

fn idx(index: (usize, usize)) -> usize {
	return index.0 * 3 + index.1
}

impl<N: Num> Matrix33<N> {
	pub fn scale(sx: N, sy: N) -> Matrix33<N> {
		Matrix33::new(
			(sx, N::zero(), N::zero()),
			(N::zero(), sy, N::zero()),
			(N::zero(), N::zero(), N::one())
		)
	}

	pub fn shear(hx: N, hy: N) -> Matrix33<N> {
		Matrix33::new(
			(N::one(), hx, N::zero()),
			(hy, N::one(), N::zero()),
			(N::zero(), N::zero(), N::one()),
		)
	}

	pub fn rotate(angle: f64 /* in radians */) -> Matrix33<N> {
		let (s, c) = (angle.sin(), angle.cos());
		Matrix33::new(
			(N::from_f64(c), N::from_f64(s), N::zero()),
			(N::from_f64(-s), N::from_f64(c), N::zero()),
			(N::zero(), N::zero(), N::one()),
		)
	}

	pub fn translate(tx: N, ty: N) -> Matrix33<N> {
		Matrix33::new(
			(N::zero(), N::zero(), N::zero()),
			(N::zero(), N::zero(), N::zero()),
			(tx, ty, N::one()),
		)
	}

	pub fn new(row0: (N, N, N), row1: (N, N, N), row2: (N, N, N)) -> Matrix33<N> {
		Matrix33{points: [
			row0.0, row0.1, row0.2,
			row1.0, row1.1, row1.2,
			row2.0, row2.1, row2.2
		]}
	}

	fn refs(&self) -> (&N, &N, &N, &N, &N, &N, &N, &N, &N) {
		(&self.points[0], &self.points[1], &self.points[2], &self.points[3], &self.points[4], &self.points[5], &self.points[6], &self.points[7], &self.points[8])
	}

	pub fn det(&self) -> N {
		let (a, b, c, d, e, f, g, h, i) = self.refs();
		N::zero() // TODO
	}
}

// (row, col)
impl<N: Num> Index<(usize, usize)> for Matrix33<N> {
	type Output = N;
	fn index(&self, index: (usize, usize)) -> &N {
		assert!(index.0 >= 0 && index.0 < 3 && index.1 >= 0 && index.1 < 3);
		&self.points[idx(index)]
	}
}

impl<N: Num> Mul for Matrix33<N> {
	type Output = Matrix33<N>;
	fn mul(self, other: Matrix33<N>) -> Matrix33<N> {
		let mut p = [N::zero(), N::zero(), N::zero(), N::zero(), N::zero(), N::zero(), N::zero(), N::zero(), N::zero()];
		for i in 0..3 {
			for j in 0..3 {
				p[idx((i, j))] =
					self[(i, 0)].clone()*other[(0, j)].clone() +
					self[(i, 1)].clone()*other[(1, j)].clone() +
					self[(i, 2)].clone()*other[(2, j)].clone();
			}
		}
		Matrix33{points: p}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_mul() {
		let a = Matrix33::new( (1.0, 2.0, 3.0), (4.0, 5.0, 6.0), (7.0, 8.0, 9.0) );
		let b = Matrix33::new( (10.0, 11.0, 12.0), (13.0, 14.0, 15.0), (16.0, 17.0, 18.0) );
		let c = a * b;
		assert_eq!(84.0, c[(0,0)]);
		assert_eq!(90.0, c[(0,1)]);
		assert_eq!(96.0, c[(0,2)]);
		assert_eq!(201.0, c[(1,0)]);
		assert_eq!(216.0, c[(1,1)]);
		assert_eq!(231.0, c[(1,2)]);
		assert_eq!(318.0, c[(2,0)]);
		assert_eq!(342.0, c[(2,1)]);
		assert_eq!(366.0, c[(2,2)]);
	}
}
