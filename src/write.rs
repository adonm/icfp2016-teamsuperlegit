use std::io::{Error,Write};
use std::convert::From;

use core::*;

pub trait Folds<N: Num> {
	// Given a source point, returns its final destination after applying all the folds
	fn transform(&self, src: &Point<N>) -> Point<N>;
}

// XXX unfinished
fn facets<N: Num>(skel: Skeleton<N>) -> (Vec<Point<N>>, Vec<Vec<usize>>) {
	let mut points = Vec::new();
	let mut facets = Vec::new();
	/* 1. find edges which share a vertex
	** 2. sort edges according to angle
	** 3. construct poly using shortest line segments along adjacent angles */
	for i in 0..skel.lines.len() {
		for j in i+1..skel.lines.len() {
			if let Some(p) = intersect_lines(&skel.lines[i], &skel.lines[j]) {
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

pub fn solution<N: Num, F: Folds<N>, W: Write>(writer: W, skel: Skeleton<N>, folds: F) -> Result<(), Error> {
	let (points, facets) = facets(skel);
	write(writer, points, facets, folds)
}

// currently private but may be a better entry point in the future?
// `points` is the list of vertices that make up the facet corners
// `facets` is a list of integer sequences, where each integer is an index into `points`
fn write<N: Num, F: Folds<N>, W: Write>(mut writer: W, points: Vec<Point<N>>, facets: Vec<Vec<usize>>, folds: F) -> Result<(), Error> {
	try!(write!(writer, "{}\n", points.len()));
	for p in points.iter() {
		try!(write!(writer, "{}\n", p));
	}
	try!(write!(writer, "{}\n", facets.len()));
	for facet in facets {
		for index in facet {
			try!(write!(writer, "{} ", index));
		}
	}
	try!(write!(writer, "\n"));
	for p in points {
		try!(write!(writer, "{}\n", folds.transform(&p)));
	}
	Ok(())
}
