use super::*;

use ndarray::rcarr2;
use ndarray::RcArray;
use ndarray::Ix;

#[derive(Debug,Clone,PartialEq,PartialOrd)]
pub struct Point<N: Num> {
	pub x: N,
	pub y: N,
}

#[derive(Debug,Clone,PartialEq)]
pub struct Line<N: Num> {
	pub p1: Point<N>,
	pub p2: Point<N>
}

#[derive(Debug,Clone,PartialEq)]
pub struct Polygon<N: Num> {
	is_hole: bool,
	square: bool,
	area: f64,
//    tranform: // 3x3 matrix
	corners: Vec<(Line<N>, Line<N>)>,
	pub points: Vec<Point<N>>,
	pub transform: RcArray<N, (Ix, Ix)>
}

#[derive(Debug,Clone)]
pub struct Shape<N: Num> {
	pub polys: Vec<Polygon<N>>,
}

#[derive(Debug,Clone)]
pub struct Skeleton<N: Num> {
	pub lines: Vec<Line<N>>,
}

//This assumes l0 -> l1 is clockwise, and l0.p2==l1.p1
fn dot<N: Num>(l0: &Line<N>, l1: &Line<N>) -> N {
	let d1 = &l0.p2 - &l0.p1;
	let d2 = &l1.p2 - &l0.p2;

	d1.x*d2.y - d1.y*d2.x
}


//This assumes l0 -> l1 is clockwise, and l0.p2==l1.p1
fn dot_points<N: Num>(a: &Point<N>, b: &Point<N>) -> N {
	N::from_f64(a.x.to_f64() * b.x.to_f64() + a.y.to_f64() * b.y.to_f64())
}

// infinite line intersection
pub fn intersect<N:Num>(a: &Line<N>, b: &Line<N>) -> Option<Point<N>> {
	let e = &a.p1 - &a.p2;
	let f = &b.p1 - &b.p2;

	let p = Point{ x: -e.x, y: e.y };

	// if dot_points is 0 bignum dies
	if dot_points(&f,&p).to_f64() != 0.0 {
		let h = ( dot_points(&(&a.p2-&b.p2),&p) ) / ( dot_points(&f,&p) );
		if h >= N::from_f64(0.0) && h<=N::from_f64(1.0) {
			return Some( &b.p2 + &Point{x:f.x*h.clone(), y:f.y*h.clone()} )
		}
	}
	return None
}

fn cross_scalar<N: Num>(a: &Point<N>, b: &Point<N>) -> N {
	a.x.clone() * b.y.clone() - a.y.clone() * b.x.clone()
}

// http://stackoverflow.com/a/1968345
// discrete line intersection
pub fn intersect_lines<N: Num>(a: &Line<N>, b: &Line<N>) -> Option<Point<N>> {
	let s1 = &a.p2 - &a.p1;
	let s2 = &b.p2 - &b.p1;
	let c1 = &a.p1 - &b.p1;

	let s = cross_scalar(&s1, &c1) / cross_scalar(&s1, &s2);
	let t = cross_scalar(&s2, &c1) / cross_scalar(&s1, &s2);

	if (s >= N::zero()) && (s < N::one()) && (t >= N::zero()) && (t <= N::one()) {
		return Some(&a.p1 + s1.scale(t));
	}

	None
}

// Use intersect_poly_inf or _discrete below instead of this function
pub fn intersect_poly<N: Num>(line: Line<N>, other: Polygon<N>, discrete: bool) -> Option<(Point<N>, Point<N>)> {
	let mut candidates = Vec::new();
	for boundary in other.to_lines().iter() {
		// If the beginning or end of the line are coincident to the boundary, they need to be added
		if boundary.coincident(&line.p1) {
			candidates.push(line.p1.clone());
		}

		if boundary.coincident(&line.p2) {
			candidates.push(line.p2.clone());
		}

		// Check normal intersections
		let point: Option<Point<N>>;
		if discrete {
			point = intersect_lines(&line, &boundary);
		} else {
			point = intersect(&line, &boundary);
		}

		if point != None {
			let point_c = point.unwrap().clone();
			candidates.push(point_c);
		}
	}

	candidates.sort();
	candidates.dedup();

	println!("intersect_poly (discrete={}) for {}, {} candidates - ", discrete, line, candidates.len());
	for p in candidates.clone() {
		println!("{}", p);
	}

	if candidates.len() == 2 {
		return Some((candidates[0].clone(), candidates[1].clone()));
	} else {
		assert!(candidates.len() == 0 || candidates.len() == 1);
		return None
	}
}

// Return the pair of points where a line intersects the given poly (discrete lines). 
//
// If the line starts in the square and finishes outside, return None.
// If the line does not intersect return None
pub fn intersect_poly_discrete<N:Num>(line: Line<N>, other: Polygon<N>) -> Option<(Point<N>, Point<N>)> {
	intersect_poly(line, other, true)
}

// Return the pair of points where a line intersects the given poly, if it is
// extended to infinity in both directions
//
// If the line starts in the square and finishes outside, return None.
// If the line does not intersect return None
pub fn intersect_poly_inf<N:Num>(line: Line<N>, other: Polygon<N>) -> Option<(Point<N>, Point<N>)> {
	intersect_poly(line, other, false)
}

pub fn gradient<N:Num>(l: &Line<N>) -> N {
	( l.p2.y.clone() - l.p1.y.clone() ) / ( l.p2.x.clone() - l.p1.x.clone() )
}

//reflect point p on axis l
pub fn flip_point<N:Num>(p: &Point<N>, vertex1: &Point<N>, vertex2: &Point<N>) -> Point<N> {
	let two = N::from_f64(2.0);
    let a = gradient(&Line{p1:vertex1.clone(),p2:vertex2.clone()});
	let c = vertex1.y.clone() - vertex1.x.clone() * a.clone();
	let x = p.x.clone();
	let y = p.y.clone();
    let d = (x.clone() + (y.clone() - c.clone())*a.clone())/(N::from_f64(1.0) + a.clone()*a.clone());

	Point{ x: two.clone()*d.clone() - x.clone(), y: two.clone()*d.clone()*a.clone() - y.clone() + two.clone()*c.clone() }
}

//flips both points of a line on an axis
pub fn flip_line<N:Num>(line: &Line<N>, vertex1: &Point<N>, vertex2: &Point<N>) -> Line<N> {
	Line{ p1: flip_point(&line.p1,&vertex1,&vertex2), p2: flip_point(&line.p2,&vertex1,&vertex2) }
}

// If there is an intersection, assume line.p1 is the point that does not get flipped
pub fn fold_line<N:Num>(line: &Line<N>, vertex1: &Point<N>, vertex2: &Point<N>) -> Vec<Line<N>> {
	let intersect = intersect_lines(&line,&Line{p1:vertex1.clone(),p2:vertex2.clone()});

	match intersect {
		Some(p) => {
			let l1 = Line{p1: p.clone(), p2: line.p1.clone() };
			let l2 = Line{p1: p.clone(), p2: flip_point(&line.p2,&vertex1,&vertex2) };
			vec!(l1,l2)
		}
        None => vec!(flip_line(&line,&vertex1,&vertex2))
	}
}

pub fn fold_polygon<N: Num>(poly: &Polygon<N>, vertex1: &Point<N>, vertex2: &Point<N>) -> Polygon<N> {
    
    let mut polyF = Polygon::new(Vec::new());
    
    for edge in poly.edges() {
        
        for line in fold_line( &edge, &vertex1, &vertex2 ) {
            polyF.points.push(line.p1);
            polyF.points.push(line.p2);
        }
    }
    
    polyF
}

pub fn split_polygon<N: Num>(poly: &Polygon<N>, v1: &Point<N>, v2: &Point<N>) -> (Polygon<N>,Polygon<N>) {
    
    let mut poly1 = Polygon::new(Vec::new());
    let mut poly2 = Polygon::new(Vec::new());
    
    let mut vertex1 = v1;
    let mut vertex2 = v2;
    
    
    for edge in poly.edges() {
        
        if edge.coincident(&vertex1) {
            
            poly1.points.push(edge.p1);
//            poly1.push(vertex1.clone()); // don't do. would be added twice
            
            poly1.points.push(vertex1.clone());
//            poly1.push(vertex2.clone()); // don't do. would be added twice
            
            poly2.points.push(vertex1.clone());
//            poly2.push(edge.p2); // don't do. would be added twice

            let (a, b) = (vertex2,vertex1);
            vertex1 = a;
            vertex2 = b;
            
            let (c,d) = (poly2,poly1);
            poly1 = c;
            poly2 = d;
            
        } else {
            poly1.points.push(edge.p1.clone());
        }
        
    }
    
    (poly1,poly2)
}

pub fn can_fold<N: Num>(poly: &Polygon<N>, vertex1: &Point<N>, vertex2: &Point<N>) -> bool {
    
    let mut coincident1 = false;
    let mut coincident2 = false;

    for line in poly.edges() {

        if line.coincident(vertex1){
            coincident1 = true
        }

        if line.coincident(vertex2){
            coincident2 = true
        }

    }
    
    return coincident1 && coincident2
}


pub fn is_convex<N: Num>(l0: &Line<N>, l1: &Line<N>) -> bool {
	dot(l0,l1) > N::zero()
}

pub fn p_distance<N: Num>(p1: &Point<N>, p2: &Point<N>) -> f64 {
	let d = p1 - p2;
	return (d.x.to_f64().powi(2) + d.y.to_f64().powi(2)).sqrt();
}

pub fn v_distance<N: Num>(p: &Point<N>) -> f64 {
	return (p.x.to_f64().powi(2) + p.y.to_f64().powi(2)).sqrt();
}


pub fn normalize_line<N:Num>(start: &Point<N>, dir: &Point<N>) -> Point<N> {
	let ratio = N::from_f64(1.0 / v_distance(dir));
	let scaled = dir.scale(ratio);
	start + &scaled
}

impl<N: Num> Point<N> {
	pub fn to_f64(&self) -> Point<f64> {
		Point{x: self.x.to_f64(), y: self.y.to_f64()}
	}

	pub fn scale(&self, alpha: N) -> Point<N> {
		Point{x: self.x.clone() * alpha.clone(), y: self.y.clone() * alpha}
	}
}

impl<N: Num> Polygon<N> {
	pub fn new(points: Vec<Point<N>>) -> Polygon<N> {
		let (clockwise, area, square, corners) = orient_area(&points);
		// transform is setup to do nothing by default
		// should represent the transformation to go back to unit square
		Polygon{points: points, area: area, square: square, is_hole: clockwise, corners: corners,
			transform: rcarr2(&[
				[N::one(), N::zero(), N::zero()],
				[N::zero(), N::one(), N::zero()],
				[N::zero(), N::zero(), N::one()]
			])
		}
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

	pub fn corners(&self) -> Vec<(Line<N>, Line<N>)> {
		self.corners.clone()
	}

	pub fn edges(&self) -> Vec<Line<N>> {
		let mut edges: Vec<Line<N>> = Vec::new();
		let mut previous = self.points.len() - 1;
		for (i, point) in self.points.iter().enumerate() {
			let edge = Line{p1: self.points[previous].clone(), p2: point.clone()};
			edges.push(edge);
		}
		return edges;
	}

  // Test whether point contained within this polygon
	pub fn contains(&self, test: &Point<N>) -> bool {
		// https://www.ecse.rpi.edu/Homepages/wrf/Research/Short_Notes/pnpoly.html
		let end = self.points.len();
		let mut contains = false;
		for offset in 0..end {
			//println!("contains - offset={}/{}", offset, end);
			let ref p1 = self.points[offset];
			let ref p2 = self.points[(offset+1)%end];
			let intersect = ((p1.y.clone() > test.y.clone()) != (p2.y.clone() > test.y.clone())) &&
				(test.x.clone() < (p2.x.clone() - p1.x.clone())*(test.y.clone() - p1.y.clone()) / (p2.y.clone() - p1.y.clone()) + p1.x.clone());
			if intersect {
				//println!("intersect");
				contains = !contains;
			}
		}

		contains
	}

  // Test whether point coincident on this polygon
	pub fn coincident(&self, test: &Point<N>) -> bool {
		let end = self.points.len();
		for offset in 0..end {
			let l_test = Line::new(self.points[offset].clone(), self.points[(offset+1)%end].clone());
			if l_test.coincident(&test) {
				return true
			}
		}
		false
	}

	// Returns the longest edge of this polygon
	pub fn longest_edge(self) -> (Point<N>, Point<N>) {
		let mut max: f64 = p_distance(&self.points.last().unwrap(), &self.points[0]);
		let mut longest: (Point<N>, Point<N>) = (self.points.last().unwrap().clone(), self.points[0].clone());
		for edge in self.points.windows(2) {
			let distance = p_distance(&edge[0], &edge[1]);
			if distance > max {
				max = distance;
				longest = (Point{x: edge[0].x.clone(), y: edge[0].y.clone()}, Point{x: edge[1].x.clone(), y: edge[1].y.clone()});
			}
		}

		return longest;
	}

	// Return this polygon as a vector of lines
	pub fn to_lines(self) -> Vec<Line<N>> {
		let mut output = Vec::new();
		for edge_p in self.points.windows(2) {
			output.push(Line::new(edge_p[0].clone(), edge_p[1].clone()));
		}

		// n-1 -> 0
		output.push(Line::new(self.points.last().unwrap().clone(), self.points[0].clone()));

		output
	}

	// Return the set of edges of this polygon that slice the provided polygon.
	//
	// An edge qualifies if it
	//  - crosses at least one boundary of the unit square.
	//  - lies wholly within the unit square
	pub fn slicey_edges(self, other: Polygon<f64>) -> Vec<Line<f64>> {
		let mut candidates = Vec::new();

		for edge in self.to_lines() {
		  println!("slicey_edges - considering line {}", edge);
		  println!("  contained {} {}", other.contains(&edge.p1.to_f64()), other.contains(&edge.p2.to_f64()));
			let mut intersection: Option<(Point<f64>, Point<f64>)> = intersect_poly_discrete(edge.clone().to_f64(), other.clone());
			if intersection == None {
				// Line lies wholly within or wholly without the unit square, or straddles the boundary
				if other.contains(&edge.p1.to_f64()) || other.contains(&edge.p2.to_f64()) {
					intersection = intersect_poly_inf(edge.clone().to_f64(), other.clone());
				}
			}
			// Poss. do something with intersection here
			
			if intersection != None {
				candidates.push(Line::new(intersection.clone().unwrap().0, intersection.clone().unwrap().1));
			}
		}

		candidates
	}

	// Return the first vertex where this polygon departs the unit square - the
	// vertex closest to 0,0 that lies on the unit square.
	//
	// The polygon must be convex (no holes)
	//
	// If some element of the polygon lies outside the unit square, we'll still
	// find the vertex closest to 0,0.
	//
	// If no vertex of the polygon lies on the unit square, return None.
	pub fn lowest_vertex(self, unit_sq: Polygon<N>) -> Option<Point<N>> {

		let mut candidates = Vec::new();

		// Search the axes in order of close-ness to 0,0
		for boundary in unit_sq.edges() {
			// Build a list of points coincident to this axis
			for point in self.points.clone() {
				if boundary.coincident(&point) {
					candidates.push(point);
				}
			}

			// If we found any points, don't try any other axes
			if candidates.len() > 0 {
				break;
			}
		}

		// No verticies coincident with the unit square
		if candidates.len() == 0 {
			return None;
		}

		// Pick the closest point from the candidates
		let origin = Point{x: N::zero(), y: N::zero()};
		let mut min = candidates[0].clone();
		for point in candidates {
			if p_distance(&origin, &point) < p_distance(&origin, &min) {
				min = point;
			}
		}

		// I can't get this to work --blinken
		//return candidates.min_by_key(|point| p_distance(&origin, *point));

		Some(min)
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

impl<N: Num> Line<N> {
	pub fn new(p1: Point<N>, p2: Point<N>) -> Line<N> {
		return Line{p1: p1, p2: p2};
	}

	pub fn to_f64(self) -> Line<f64> {
		Line::new(self.p1.to_f64(), self.p2.to_f64())
	}

	// Returns the length of this line
	pub fn len(&self) -> f64 {
		return p_distance(&self.p1, &self.p2);
	}

	// True if point lies on this line
	pub fn coincident(&self, point: &Point<N>) -> bool {
		return p_distance(&self.p1, point) + p_distance(point, &self.p2) == self.len();
	}

	// Returns a point along this line. 0 <= alpha <= 1, else you're extrapolating bro
	pub fn interpolate(&self, alpha: N) -> Point<N> {
		&self.p1 + &(&self.p2 - &self.p1).scale(alpha)
	}

	// Splits this line into two at the specified position along it.
	pub fn split(&self, alpha: N) -> (Line<N>, Line<N>) {
		let mid = self.interpolate(alpha);
		let l1 = Line::new(self.p1.clone(), mid.clone());
		let l2 = Line::new(mid, self.p2.clone());
		(l1, l2)
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

	// Returns the number of lines composing this skeleton
	pub fn len(self) -> usize {
		return self.lines.len();
	}

	// Returns the longest edge in this skeleton
	pub fn longest_edge(self) -> Line<N> {
		let mut longest: Line<N> = self.lines[0].clone();
		for line in self.lines {
			if line.len() > longest.len() {
				longest = line;
			}
		}

		return longest.clone();
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
fn orient_area<N: Num>(points: &Vec<Point<N>>) -> (bool, f64, bool, Vec<(Line<N>, Line<N>)>) {
	let mut corners: Vec<(Line<N>, Line<N>)> = Vec::new();
	let n = points.len();
	let mut square = n == 4;
	// first case
	let mut sum = half_tri_area(&points[n-1], &points[0]);
	let mut edge1 = (&points[n-1], &points[0]);
	let cornerangle = (angle(edge1.0, edge1.1) - angle(&points[n-2], &points[n-1])).abs();
	if cornerangle % 90.0_f64.to_radians() > 0.00001 {
		square = false;
	} else {
		corners.push((
			Line{p1: (*edge1.0).clone(), p2: (*edge1.1).clone()},
			Line{p1: points[n-2].clone(), p2: points[n-1].clone()}
		));
	}
	// rest of polygon
	for segment in points.windows(2) {
		let edge = (&segment[0], &segment[1]);
		let cornerangle = (angle(edge1.0, edge1.1) - angle(edge.0, edge.1)).abs();
		if cornerangle % 90.0_f64.to_radians() > 0.000001 {
			square = false;
		} else {
			corners.push((
				Line{p1: (*edge1.0).clone(), p2: (*edge1.1).clone()},
				Line{p1: (*edge.0).clone(), p2: (*edge.1).clone()}
			));
		}
		edge1 = edge;
		sum = sum + half_tri_area(&segment[0], &segment[1]);
	}
	let f = sum.to_f64();
	return (f >= 0.0, f.abs() / 2.0, square, corners)
}

/*pub fn mirror<N: Num>(shapes: &Vec<Polygon<N>>, axis: Line<N>) -> Vec<Polygon<N>> {
		let mut results: Vec<Polygon<N>> = Vec::new();
		for shape in shapes {
				let new_shape = (*shape).clone();
				results.push(new_shape);
		}
		results
}*/
