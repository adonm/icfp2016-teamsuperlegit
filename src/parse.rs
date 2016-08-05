use std::convert::From;
use std::fmt::Debug;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::vec::Vec;

pub use core::*;

#[derive(Debug)]
pub enum ParseError {
	BadPoint,
	BadLine,
	IOError,
	SubError,
}

impl ParseError {
	pub fn wrap<T, E>(res: Result<T, E>) -> Result<T,ParseError> {
		res.map_err(|_| {ParseError::SubError})
	}
}

impl From<::std::io::Error> for ParseError {
	fn from(_: ::std::io::Error) -> ParseError {
		ParseError::IOError
	}
}

impl<N: Num> FromStr for Point<N> {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Point<N>, Self::Err> {
		let fields: Vec<_> = s.split(",").collect();
		if fields.len() != 2 {
			return Err(ParseError::BadPoint);
		}
		return Ok(Point{x: try!(ParseError::wrap(fields[0].parse::<N>())), y: try!(ParseError::wrap(fields[1].parse::<N>()))});
	}
}

impl<N: Num> FromStr for Line<N> {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Line<N>, Self::Err> {
		let fields: Vec<_> = s.split(" ").collect();
		if fields.len() != 2 {
			return Err(ParseError::BadLine);
		}
		return Ok(Line{p1: try!(ParseError::wrap(fields[0].parse::<Point<N>>())), p2: try!(ParseError::wrap(fields[1].parse::<Point<N>>()))});
	}
}


/* pretty much the worst error reporting possible, sorry. no line numbers and
** the underlying error is masked by the ParseError type. */
pub fn parse<N: Num, R: Read>(stream: R) -> Result<(Shape<N>, Skeleton<N>), ParseError> {
	let mut reader = BufReader::new(stream);
	let num_polys: i64 = try!(parse_line(&mut reader));
	let mut shape = Vec::new();
	for _ in 0..num_polys {
		let num_points: i64 = try!(parse_line(&mut reader));
		let mut poly = Vec::new();
		for _ in 0..num_points {
			poly.push(try!(parse_line::<Point<N>,R>(&mut reader)));
		}
		shape.push(Polygon::new(poly));
	}
	let num_edges: i64 = try!(parse_line(&mut reader));
	let mut skel = Skeleton::new(Vec::new());
	for _ in 0..num_edges {
		skel = skel.push(try!(parse_line::<Line<N>,R>(&mut reader)));
	};
	Ok((Shape::new(shape), skel))
}

fn parse_line <T: FromStr+Debug, R: Read>(reader: &mut BufReader<R>) -> Result<T, ParseError> where <T as FromStr>::Err: Debug {
	let mut s = String::new();
	try!(reader.read_line(&mut s));
	//println!("{} => {:?}", s.trim(), s.trim().parse::<T>());
	ParseError::wrap(s.trim().parse::<T>())
}


#[cfg(test)]
mod tests {
	use super::*;
	extern crate num;
	use self::num::rational::BigRational;
	use self::num::bigint::BigInt;
	use std::fs::File;
	use super::super::BASEPATH;

	fn rati(n: i64, d: i64) -> BigRational {
		return BigRational::new(BigInt::from(n), BigInt::from(d));
	}

	#[test]
	fn test_point_parse() {
		assert_eq!(Point{x: 1, y: 0}, "1,0".parse::<Point<i32>>().unwrap());
		println!("{:?}", "4328029871649615121465353437184/8656059743299229793415925725865,-1792728671193156318471947026432/8656059743299229793415925725865".parse::<Point<BigRational>>().unwrap());
	}

	#[test]
	fn test_parse_problem1() {
		let f = File::open(format!("{}/001.problem.txt", BASEPATH)).unwrap();
		let (shape, skel) = parse::<i32, File>(f).unwrap();
		assert_eq!(1, shape.polys.len());
		assert_eq!(4, shape.polys[0].points.len());
		assert_eq!(4, skel.len());
	}

	#[test]
	fn test_parse_problem4() {
		let f = File::open(format!("{}/004.problem.txt", BASEPATH)).unwrap();
		let (shape, skel) = parse::<BigRational, File>(f).unwrap();
		assert_eq!(1, shape.polys.len());
		assert_eq!(4, shape.polys[0].points.len());
		assert_eq!(1.0, shape.polys[0].area());
		assert_eq!(4, skel.len());
		assert_eq!(Point{x: rati(1, 1), y: rati(0, 1)}, &shape.polys[0].points[1] - &shape.polys[0].points[0]);
		assert!(!shape.polys[0].is_hole());
	}

	#[test]
	fn test_parse_problem7() {
		let f = File::open(format!("{}/007.problem.txt", BASEPATH)).unwrap();
		let (shape, skel) = parse::<BigRational, File>(f).unwrap();
		assert_eq!(1, shape.polys.len());
		assert_eq!(4, shape.polys[0].points.len());
		assert_eq!(4, skel.len());
		println!("{}", shape.polys[0].points[0].x);
		//assert_eq!(0,1);
	}
}
