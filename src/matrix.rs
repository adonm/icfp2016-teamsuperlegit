use std::ops::{Div,Index,Mul,MulAssign};
use std::fmt;

pub use core::*;

#[derive(Debug,PartialEq)]
pub struct Matrix33<N: Num> {
	points: [N; 9],
}

fn idx(index: (usize, usize)) -> usize {
	return index.0 * 3 + index.1
}

impl<N: Num> Matrix33<N> {
	pub fn scale(sx: N, sy: N) -> Matrix33<N> {
		println!("scale {} {}", sx, sy);
		Matrix33::new(
			(sx, N::zero(), N::zero()),
			(N::zero(), sy, N::zero()),
			(N::zero(), N::zero(), N::one())
		)
	}

	#[allow(dead_code)]
	pub fn shear(hx: N, hy: N) -> Matrix33<N> {
		Matrix33::new(
			(N::one(), hx, N::zero()),
			(hy, N::one(), N::zero()),
			(N::zero(), N::zero(), N::one()),
		)
	}

	pub fn rotate_angle(angle: f64 /* in radians */) -> Matrix33<N> {
		let (s, c) = (angle.sin(), angle.cos());
		Matrix33::rotate(N::from_f64(s), N::from_f64(c))
	}

	pub fn rotate(sine: N, cosine: N) -> Matrix33<N> {
		println!("rotate {} {}", sine, cosine);
		Matrix33::new(
			(cosine.clone(), sine.clone(), N::zero()),
			(-sine, cosine, N::zero()),
			(N::zero(), N::zero(), N::one()),
		)
	}

	pub fn translate(tx: N, ty: N) -> Matrix33<N> {
		println!("translate {} {}", tx, ty);
		Matrix33::new(
			(N::one(), N::zero(), N::zero()),
			(N::zero(), N::one(), N::zero()),
			(tx, ty, N::one()),
		)
	}

	pub fn identity() -> Matrix33<N> {
		Matrix33::new((N::one(), N::zero(), N::zero()), (N::zero(), N::one(), N::zero()), (N::zero(), N::zero(), N::one()))
	}

	pub fn new(row0: (N, N, N), row1: (N, N, N), row2: (N, N, N)) -> Matrix33<N> {
		Matrix33{points: [
			row0.0, row0.1, row0.2,
			row1.0, row1.1, row1.2,
			row2.0, row2.1, row2.2
		]}
	}

	pub fn then_scale(self, sx: N, sy: N) -> Matrix33<N> {
		self * Matrix33::scale(sx, sy)
	}

	pub fn then_rotate(self, sine: N, cosine: N) -> Matrix33<N> {
		self * Matrix33::rotate(sine, cosine)
	}

	#[allow(dead_code)]
	pub fn then_rotate_angle(self, angle: f64) -> Matrix33<N> {
		self * Matrix33::rotate_angle(angle)
	}

	pub fn then_translate(self, tx: N, ty: N) -> Matrix33<N> {
		self * Matrix33::translate(tx, ty)
	}

	pub fn transform(&self, p: Point<N>) -> Point<N> {
		let x = p.x.clone() * self[(0, 0)].clone() + p.y.clone() * self[(1, 0)].clone() + self[(2, 0)].clone();
		let y = p.x.clone() * self[(0, 1)].clone() + p.y.clone() * self[(1, 1)].clone() + self[(2, 1)].clone();
		Point{x: x, y: y}
	}

	fn clones(&self) -> (N, N, N, N, N, N, N, N, N) {
		(self.points[0].clone(), self.points[1].clone(), self.points[2].clone(), self.points[3].clone(), self.points[4].clone(), self.points[5].clone(), self.points[6].clone(), self.points[7].clone(), self.points[8].clone())
	}

	// https://en.wikipedia.org/wiki/Determinant
	pub fn det(&self) -> N {
		let (a, b, c, d, e, f, g, h, i) = self.clones();
		let (a2, b2, c2, d2, e2, f2, g2, h2, i2) = self.clones();
		a*e*i + b*f*g + c*d*h - c2*e2*g2 - b2*d2*i2 - a2*f2*h2
	}

	// https://en.wikipedia.org/wiki/Invertible_matrix#Methods_of_matrix_inversion
	#[allow(non_snake_case)]
	pub fn inverse(&self) -> Matrix33<N> {
		let (a, b, c, d, e, f, g, h, i) = self.clones();
		let (a2, b2, c2, d2, e2, f2, g2, h2, i2) = self.clones();
		let (a3, b3, c3, d3, e3, f3, g3, h3, i3) = self.clones();
		let (a4, b4, c4, d4, e4, f4, g4, h4, i4) = self.clones();
		let A = e*i - f*h;
		let B = -(d*i2 - f2*g);
		let C = d2*h2 - e2*g2;
		let D = -(b*i3 - c*h3);
		let E = a*i4 - c2*g3;
		let F = -(a2*h4 - b2*g4);
		let G = b3*f3 - c3*e3;
		let H = -(a3*f4 - c4*d3);
		let I = a4*e4 - b4*d4;
		Matrix33::new((A, D, G), (B, E, H), (C, F, I)) / self.det()
	}
}

impl<N: Num> Div<N> for Matrix33<N> {
	type Output = Self;
	fn div(self, d: N) -> Matrix33<N> {
		Matrix33{ points: [
			self.points[0].clone() / d.clone(),
			self.points[1].clone() / d.clone(),
			self.points[2].clone() / d.clone(),
			self.points[3].clone() / d.clone(),
			self.points[4].clone() / d.clone(),
			self.points[5].clone() / d.clone(),
			self.points[6].clone() / d.clone(),
			self.points[7].clone() / d.clone(),
			self.points[8].clone() / d
		]}
	}
}

// (row, col)
impl<N: Num> Index<(usize, usize)> for Matrix33<N> {
	type Output = N;
	fn index(&self, index: (usize, usize)) -> &N {
		assert!(index.0 < 3 && index.1 < 3);
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

impl<N: Num> MulAssign for Matrix33<N> {
	fn mul_assign(&mut self, other: Matrix33<N>) {
		let m = (*self).clone() * other;
		self.points = m.points;
	}
}

impl<N: Num> Clone for Matrix33<N> {
	fn clone(&self) -> Matrix33<N> {
		Matrix33{points: [
			self.points[0].clone(), self.points[1].clone(), self.points[2].clone(),
			self.points[3].clone(), self.points[4].clone(), self.points[5].clone(),
			self.points[6].clone(), self.points[7].clone(), self.points[8].clone()
		]}
	}
}

impl<N: Num> fmt::Display for Matrix33<N> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[ {} {} {} ]\n[ {} {} {} ]\n[ {} {} {} ]\n", &self.points[0], &self.points[1], &self.points[2], &self.points[3], &self.points[4], &self.points[5], &self.points[6], &self.points[7], &self.points[8])
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use num::Float;

	pub fn p<N:Num>(x: N, y: N) -> Point<N> {
		Point{x: x, y: y}
	}

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

	#[test]
	fn test_mulassign() {
		let mut a = Matrix33::translate(1.0, 1.0);
		a *= Matrix33::scale(2.0, 1.0);
		a *= Matrix33::translate(-1.0, -1.0);

		assert_eq!(p(5.0, 4.0), a.transform(p(2.0, 4.0)));
	}

	#[test]
	fn test_scale() {
		assert_eq!(p(-2.5, 28.0), Matrix33::scale(-1.0, 4.0).transform(p(2.5, 7.0)));
	}

	#[test]
	fn test_rotate() {
		// close enough :S
		assert_eq!(p(1.0, -0.00000000000000006123233995736766), Matrix33::rotate_angle(90.0.to_radians()).transform(p(0.0, -1.0)));
	}

	#[test]
	fn test_translate() {
		assert_eq!(p(6.0, -0.5), Matrix33::translate(4.0, -2.5).transform(p(2.0, 2.0)));
	}

	#[test]
	fn test_combined() {
		let m = Matrix33::scale(2.5, 1.5) * Matrix33::translate(-4.0, -4.0);
		assert_eq!(p(-1.5, -2.5), m.transform(p(1.0, 1.0)));
		assert_eq!(p(1.0, -7.0), m.transform(p(2.0, -2.0)));
	}

	#[test]
	fn test_flip_about_y3() {
		let m = Matrix33::translate(0.0, -3.0) * Matrix33::scale(1.0, -1.0) * Matrix33::translate(0.0, 3.0);
		assert_eq!(p(4.0, 2.0), m.transform(p(4.0, 4.0)));
		assert_eq!(p(2.5, 5.0), m.transform(p(2.5, 1.0)));

		let m2 = Matrix33::translate(0.0, -3.0).then_scale(1.0, -1.0).then_translate(0.0, 3.0);
		assert_eq!(p(4.0, 2.0), m2.transform(p(4.0, 4.0)));
		assert_eq!(p(2.5, 5.0), m2.transform(p(2.5, 1.0)));

		let m2i = m2.clone().inverse();
		assert_eq!(p(4.0, 4.0), m2i.transform(p(4.0, 2.0)));
		assert_eq!(p(2.5, 1.0), m2i.transform(p(2.5, 5.0)));
	}

	#[test]
	fn test_determinant() {
		assert_eq!(18.0, Matrix33::new((-2.0, 2.0, -3.0), (-1.0, 1.0, 3.0), (2.0, 0.0, -1.0)).det());
		assert_eq!(-18.0, Matrix33::new((-2.0, 2.0, -3.0), (0.0, 2.0, -4.0), (0.0, 0.0, 4.5)).det());
	}

	fn eq(a: f64, b: f64) -> bool {
		(a - b).abs() < 1e-9
	}

	#[test]
	fn test_inverse() {
		let inv = Matrix33::new((1.0, 0.0, 2.0), (1.0, 2.0, 5.0), (1.0, 5.0, -1.0)).inverse() / (1.0 / -21.0);
		assert!(eq(-27.0, inv[(0, 0)]));
		assert!(eq(10.0, inv[(0, 1)]));
		assert!(eq(-4.0, inv[(0, 2)]));
		assert!(eq(6.0, inv[(1, 0)]));
		assert!(eq(-3.0, inv[(1, 1)]));
		assert!(eq(-3.0, inv[(1, 2)]));
		assert!(eq(3.0, inv[(2, 0)]));
		assert!(eq(-5.0, inv[(2, 1)]));
		assert!(eq(2.0, inv[(2, 2)]));
	}
}
