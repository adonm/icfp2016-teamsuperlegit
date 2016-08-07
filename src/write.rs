use std::io::{Error,Write};

use core::*;

pub trait Folds<N: Num> {
	// Given a source point, returns its final destination after applying all the folds
	fn transform(&self, src: &Point<N>) -> Point<N>;
}

// XXX unfinished
fn facets<N: Num>(skel: Skeleton<N>) -> (Vec<Point<N>>, Vec<Vec<usize>>) {
	let mut points = Vec::new();
	let facets = Vec::new();
	/* 1. find edges which share a vertex
	** 2. sort edges according to angle
	** 3. construct poly using shortest line segments along adjacent angles */
	for i in 0..skel.lines.len() {
		for j in i+1..skel.lines.len() {
			if let Some(p) = intersect_discrete(&skel.lines[i], &skel.lines[j]) {
				points.push(p);
			}
		}
	}
	for line in skel.lines {
		points.push(line.p1.clone());
		points.push(line.p2.clone());
	}
	//points.sort_by(|a, b| if (a < b) { Ordering::Less } else if (a > b) { Ordering::Greater } else { Ordering::Equal }});
	points.sort();
	points.dedup();

	(points, facets)
}

pub fn from_skeleton<N: Num, F: Folds<N>, W: Write>(writer: W, skel: Skeleton<N>, folds: F) -> Result<(), Error> {
	let (points, facets) = facets(skel);
	let mut dst = Vec::new();
	for p in points.iter() {
		dst.push(folds.transform(p));
	}
	write(writer, points, facets, dst)
}

pub fn from_polys<N: Num, W: Write>(writer: W, polys: Vec<Polygon<N>>) -> Result<(), Error> {
	let mut src = Vec::new();
	let mut dst: Vec<Point<N>> = Vec::new();
	let mut facets = Vec::new();
	for poly in polys {
		let mut facet = Vec::new();
		for point in poly.points {
			let i = {
				if let Some(i) = dst.iter().position(|p| &point == p) {
					i
				} else {
					dst.push(point.clone());
					src.push(poly.transform.inverse().transform(point));
					dst.len() - 1
				}
			};
			facet.push(i);
		}
		facets.push(facet);
	}
	write(writer, src, facets, dst)
}

// currently private but may be a better entry point in the future?
// `points` is the list of vertices that make up the facet corners
// `facets` is a list of integer sequences, where each integer is an index into `points`
fn write<N: Num, W: Write>(mut writer: W, src: Vec<Point<N>>, facets: Vec<Vec<usize>>, dst: Vec<Point<N>>) -> Result<(), Error> {
	assert_eq!(src.len(), dst.len());
	try!(write!(writer, "{}\n", src.len()));
	for p in src {
		try!(write!(writer, "{}\n", p));
	}
	try!(write!(writer, "{}\n", facets.len()));
	for (i, facet) in facets.iter().enumerate() {
		try!(write!(writer, "{} ", facet.len()));
		for index in facet {
			try!(write!(writer, "{} ", index));
		}
		try!(write!(writer, "\n"));
	}
	for p in dst {
		try!(write!(writer, "{}\n", p));
	}
	Ok(())
}
