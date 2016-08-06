use super::*;

use super::super::matrix::Matrix33;

#[derive(Debug,Clone,PartialOrd)]
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
	area: f64,
	pub points: Vec<Point<N>>,
	pub transform: Matrix33<N>
}

#[derive(Debug,Clone)]
pub struct Shape<N: Num> {
	pub polys: Vec<Polygon<N>>,
}

#[derive(Debug,Clone)]
pub struct Skeleton<N: Num> {
	pub lines: Vec<Line<N>>,
}

// infinite line intersection. Returns the intersection point or None if the
// lines do not intercept.
//
// An epsilon is used to mark lines that are very close to parallel as parallel.
pub fn intersect_inf<N:Num>(a: &Line<N>, b: &Line<N>) -> Option<Point<N>> {
	let x1 = a.p1.x.clone();
	let y1 = a.p1.y.clone();
	let x2 = a.p2.x.clone();
	let y2 = a.p2.y.clone();
	let x3 = b.p1.x.clone();
	let y3 = b.p1.y.clone();
	let x4 = b.p2.x.clone();
	let y4 = b.p2.y.clone();

  // If the lines are very close to parallel return None
  let d = (x1.clone() - x2.clone())*(y3.clone() - y4.clone()) - (y1.clone() - y2.clone())*(x3.clone() - x4.clone());
  if eq_eps(&d, &N::from_f64(0.0)) {
    return None;
  }

  let x_out = ((x1.clone()*y2.clone() - y1.clone()*x2.clone())*(x3.clone() - x4.clone()) - (x1.clone() - x2.clone())*(x3.clone()*y4.clone() - y3.clone()*x4.clone())) / d.clone();
  let y_out = ((x1.clone()*y2.clone() - y1.clone()*x2.clone())*(y3.clone() - y4.clone()) - (y1.clone() - y2.clone())*(x3.clone()*y4.clone() - y3.clone()*x4.clone())) / d.clone();

  Some(Point{x: x_out, y: y_out})
}

fn cross_scalar<N: Num>(a: &Point<N>, b: &Point<N>) -> N {
	a.x.clone() * b.y.clone() - a.y.clone() * b.x.clone()
}

// http://stackoverflow.com/a/1968345
// discrete line intersection
// 
// Returns the intersection point, or None if the lines do not intercept.
pub fn intersect_discrete<N: Num>(a: &Line<N>, b: &Line<N>) -> Option<Point<N>> {
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
      println!("intersect_poly - adding coincident candidate {}", line.p1);
			candidates.push(line.p1.clone());
		}

		if boundary.coincident(&line.p2) {
      println!("intersect_poly - adding coincident candidate {}", line.p2);
			candidates.push(line.p2.clone());
		}

		// Check normal intersections
		let point: Option<Point<N>>;
		if discrete {
			point = intersect_discrete(&line, &boundary);
		} else {
			point = intersect_inf(&line, &boundary);
		}

		if point != None {
			let point_c = point.unwrap().clone();

      // The proposed intersection must be coincident on the boundary (ie. the
      // discrete segment only). intersect_inf will give us inf intersect for
      // both lines - we only want the input line to be infinite.
      if boundary.coincident(&point_c) {
        println!("intersect_poly - adding candidate {} from intersection of {} and {}", point_c, line, boundary);
        candidates.push(point_c);
      } else {
        println!("intersect_poly - candidate {} is not conincident on boundary {}, skipping", point_c, boundary);
      }
		}
	}

  // !!
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

pub fn reflect_matrix<N:Num>(vertex1: &Point<N>, vertex2: &Point<N>) -> Matrix33<N> {
    
    let l = Line::new(vertex1.clone(),vertex2.clone());
    
    if vertex1.x.clone() == N::from_f64(0.0) && vertex2.x.clone() == N::from_f64(0.0) {
        Matrix33::rotate(N::from_f64(1.0),N::from_f64(0.0))
            .then_scale(N::from_f64(1.0),N::from_f64(-1.0))
            .then_rotate(N::from_f64(-1.0),N::from_f64(0.0))
    } else {
        let g = gradient(&l);
        let c = vertex1.y.clone() - g.clone() * vertex1.x.clone();
        let d = vertex2 - vertex1;
        
        Matrix33::translate(N::from_f64(0.0),-c.clone())
            .then_rotate( - d.clone().y / v_distance(&d), d.clone().x / v_distance(&d) )
            .then_scale(N::from_f64(1.0),N::from_f64(-1.0))
            .then_rotate( d.clone().y / v_distance(&d), d.clone().x / v_distance(&d) )
            .then_translate(N::from_f64(0.0),c.clone())
    }
}

pub fn flip_point_matrix<N:Num>(p: &Point<N>, vertex1: &Point<N>, vertex2: &Point<N>) -> Point<N> {
    
    reflect_matrix(&vertex1,&vertex2).transform(p.clone())
}
// Inputs of (1,1) / (0,0) (1,0) should give (1,-1)
pub fn flip_point<N: Num>(p: &Point<N>, l1: &Point<N>, l2: &Point<N>) -> Point<N> {
    // y = ax + c
    let a = (l2.y.clone() - l1.y.clone()) / (l2.x.clone() - l1.x.clone());
    println!("{}", a);
    let c = l1.y.clone() - a.clone() * l1.x.clone();

    let d = (p.x.clone() + (p.y.clone() - c.clone()) * a.clone())/(N::from_f64(1.0) + a.clone() * a.clone());

    Point{x: d.clone() + d.clone() - p.x.clone(), y: N::from_f64(2.0) * d.clone() * a.clone() - p.y.clone() + c.clone() + c.clone()}
}

//flips both points of a line on an axis
pub fn flip_line<N:Num>(line: &Line<N>, vertex1: &Point<N>, vertex2: &Point<N>) -> Line<N> {
	Line{ p1: flip_point_matrix(&line.p1,&vertex1,&vertex2), p2: flip_point_matrix(&line.p2,&vertex1,&vertex2) }
}

// If there is an intersection, assume line.p1 is the point that does not get flipped
pub fn fold_line<N:Num>(line: &Line<N>, vertex1: &Point<N>, vertex2: &Point<N>) -> Vec<Line<N>> {
	let intersect = intersect_discrete(&line,&Line{p1:vertex1.clone(),p2:vertex2.clone()});

	match intersect {
		Some(p) => {
			let l1 = Line{p1: p.clone(), p2: line.p1.clone() };
			let l2 = Line{p1: p.clone(), p2: flip_point(&line.p2,&vertex1,&vertex2) };
			vec!(l1,l2)
		}
        None => vec!(flip_line(&line,&vertex1,&vertex2))
	}
}

pub fn flip_polygon<N: Num>(poly: &Polygon<N>, vertex1: &Point<N>, vertex2: &Point<N>) -> Polygon<N> {
    let mut poly_f = Vec::new();
    
    for pt in poly.clone().points {
        poly_f.push( flip_point( &pt, &vertex1, &vertex2 ) );
    }
    poly_f.reverse();
    let mut ret = Polygon::new(poly_f);
    ret.transform = poly.clone().transform * reflect_matrix(&vertex1,&vertex2);
    ret
}

pub fn fold_polygon<N: Num>(poly: &Polygon<N>, vertex1: &Point<N>, vertex2: &Point<N>) -> (Polygon<N>,Polygon<N>) {
    let (poly1, poly2Old) = split_polygon(&poly,&vertex1,&vertex2);
    let poly2 = flip_polygon(&poly2Old, &vertex1, &vertex2);
    
    (poly1,poly2)
}

pub fn split_polygon<N: Num>(poly: &Polygon<N>, v1: &Point<N>, v2: &Point<N>) -> (Polygon<N>,Polygon<N>) {
    
    let mut poly1 = Vec::new();
    let mut poly2 = Vec::new();
    
    let mut vertex1 = v1;
    let mut vertex2 = v2;
    
    for edge in poly.edges() {
        poly1.push(edge.clone().p1);
        
        let co = if edge.clone().coincident(&vertex1) {
            Some(vertex1)
        } else if edge.clone().coincident(&vertex2) {
            Some(vertex2)
        } else {
            None
        };
        
        match co {
            Some(v)=> {
                poly1.push(v.clone());
                
                poly2.push(v.clone());
                
                let (a, b) = (vertex2,vertex1);
                vertex1 = a;
                vertex2 = b;

                let (c,d) = (poly2,poly1);
                poly1 = c;
                poly2 = d;
            }
            None => ()
        }
    }
    
    
    (Polygon::new(poly1),Polygon::new(poly2))
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

pub fn p_distance<N: Num>(p1: &Point<N>, p2: &Point<N>) -> f64 {
	let d = p1 - p2;
	return (d.x.to_f64().powi(2) + d.y.to_f64().powi(2)).sqrt();
}

pub fn v_distance<N: Num>(p: &Point<N>) -> N {
	return N::from_f64((p.x.to_f64().powi(2) + p.y.to_f64().powi(2)).sqrt());
}


pub fn normalize_line<N:Num>(start: &Point<N>, dir: &Point<N>) -> Point<N> {
	let ratio = N::from_f64(1.0) / v_distance(dir);
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
		let (clockwise, area) = orient_area(&points);
		// transform is setup to do nothing by default
		// should represent the transformation to go back to unit square
		Polygon{points: points, area: area, is_hole: clockwise, transform: Matrix33::identity()}
	}

	pub fn is_hole(&self) -> bool {
		self.is_hole
	}

	pub fn square(&self) -> bool {
		if self.corners().len() == 4 {
			if self.edges().len() == 4{
				return true
			}
		}
		return false
	}

	pub fn area(&self) -> f64 {
		self.area
	}

	pub fn corners(&self) -> Vec<(Line<N>, Line<N>)> {
		let edges = self.edges();
		let mut corners: Vec<(Line<N>, Line<N>)> = Vec::new();
		let mut previous = edges.len() - 1;
		for (i, edge) in edges.iter().enumerate() {
			let edge1 = edges[previous].clone();
			let cornerangle = (angle(&edge1.p1, &edge1.p2) - angle(&edge.p1, &edge.p2)).abs();
			if cornerangle % 90.0_f64.to_radians() < 0.00001 {
				corners.push((edge1.clone(), edge.clone()));
			}
			previous = i;
		}
		return corners;
	}

	pub fn edges(&self) -> Vec<Line<N>> {
		let mut edges: Vec<Line<N>> = Vec::new();
		let mut previous = self.points.len() - 1;
		for (i, point) in self.points.iter().enumerate() {
			let edge = Line{p1: self.points[previous].clone(), p2: point.clone()};
			edges.push(edge);
            previous = i;
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
		return eq_eps(&(p_distance(&self.p1, point) + p_distance(point, &self.p2)), &self.len());
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
    
    pub fn length(&self) -> f64{
        p_distance(&self.p1,&self.p2)
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
fn orient_area<N: Num>(points: &Vec<Point<N>>) -> (bool, f64) {
	let mut corners: Vec<(Line<N>, Line<N>)> = Vec::new();
	let n = points.len();
	let mut square = n == 4;
	// first case
	let mut sum = half_tri_area(&points[n-1], &points[0]);
	// rest of polygon
	for segment in points.windows(2) {
		sum = sum + half_tri_area(&segment[0], &segment[1]);
	}
	let f = sum.to_f64();
	return (f >= 0.0, f.abs() / 2.0)
}

/*pub fn mirror<N: Num>(shapes: &Vec<Polygon<N>>, axis: Line<N>) -> Vec<Polygon<N>> {
		let mut results: Vec<Polygon<N>> = Vec::new();
		for shape in shapes {
				let new_shape = (*shape).clone();
				results.push(new_shape);
		}
		results
}*/


#[cfg(test)]
mod tests {
	use super::*;
	use super::super::tests::*;

	#[test]
	fn test_point_eq(){
    assert_eq!(pNum(0,0), pNum(0,0));
    assert_eq!(pNum(0.0,0.0), pNum(0.0,0.0));
    assert_eq!(pNum(0.0,0.0), pNum(0.0000000001,0.0000000001));
    assert_eq!(pNum(1.0,1.0), pNum(1.0000000001,1.0000000001));

    assert!(!(pNum(1,1) == pNum(0,0)));
    assert!(!(pNum(1.0,1.0) == pNum(0.0000000001,0.0000000001)));
  }

	#[test]
	fn gradient_test(){
        let p1 = pNum(1,1);
        let p2 = pNum(0,0);
        let g = gradient(&Line{p1:p1,p2:p2});
        println!("gradient_test: {:?}",g);
        assert_eq!(g,1);
	}

	#[test]
	fn test_flip_point(){
        let mut p2 = flip_point(&pNum(1.0,1.0), &pNum(0.0,0.0), &pNum(1.0,0.0));
        println!("flip_point_test: {:?}",p2);
        assert_eq!(pNum(1.0, -1.0), p2);

        p2 = flip_point(&pNum(1.0,0.0), &pNum(0.0,0.0), &pNum(3.0,3.0));
        println!("flip_point_test: {:?}",p2);
        assert_eq!(pNum(0.0, 1.0), p2);

        p2 = flip_point(&pNum(1.0,0.0), &pNum(0.0,0.0), &pNum(0.866025403784439,0.5)); // unit vector along x, 30 deg line. Result should be unit vector 60 degrees to the x axis
        println!("flip_point_test: {:?}",p2); 

        // Compare with an epsilon
        assert!(p2.x - 0.5 < 0.000001);
        assert!(p2.y - 0.866025403784439 < 0.000001);
	}

	#[test]
	fn test_flip_point_matrix(){
        
        let mut p2 = flip_point_matrix(&pNum(1.0,1.0), &pNum(0.0,0.0), &pNum(1.0,0.0));
        println!("flip_point_test: {:?}",p2);
        assert_eq!(pNum(1.0, -1.0), p2);

        p2 = flip_point_matrix(&pNum(1.0,0.0), &pNum(0.0,0.0), &pNum(3.0,3.0));
        println!("flip_point_test: {:?}",p2);
        assert_eq!(pNum(0.0, 1.0), p2);

        p2 = flip_point_matrix(&pNum(1.0,0.0), &pNum(0.0,0.0), &pNum(0.866025403784439,0.5)); // unit vector along x, 30 deg line. Result should be unit vector 60 degrees to the x axis
        println!("flip_point_test: {:?}",p2); 
        
        
        p2 = flip_point_matrix(&pNum(-1.0,1.0), &pNum(0.0,0.0), &pNum(0.0,3.0));
        println!("flip_point_test: {:?}",p2);
        assert_eq!(pNum(1.0, 1.0), p2);
	}

	#[test]
	fn flip_line_test(){
        let l1 = Line::new(pNum(0.0,2.0),pNum(0.0,3.0));
        let v1 = pNum(1.0,1.0);
        let v2 = pNum(2.0,2.0);
        let l2 = flip_line(&l1,&v1,&v2);
        println!("flip_line_test: {:?}",l2);
        
        assert_eq!(Line::new(pNum(2.0,0.0),pNum(3.0,0.0)), l2);
	}
    #[test]
    fn fold_line_test(){
        
        let l1 = Line::new(pNum(0.0,2.0),pNum(2.0,0.0));
        let v1 = pNum(0.0,0.0);
        let v2 = pNum(2.0,2.0);
        let l2 = fold_line(&l1,&v1,&v2);
        println!("fold_line_test: {:?}",l2);
        
        assert_eq!(vec!(Line::new(pNum(1.0,1.0),pNum(0.0,2.0)),Line::new(pNum(1.0,1.0),pNum(0.0,2.0))), l2);
    }

    #[test]
    fn fold_polygon_test(){
        let poly = Polygon::new(vec!( pNum(0.0,0.0),pNum(2.0,0.0),pNum(2.0,2.0),pNum(0.0,2.0) ));
        let v1 = pNum(0.0,1.0);
        let v2 = pNum(2.0,1.0);
        let ret = fold_polygon(&poly,&v1,&v2);
        
        println!("fold_polygon_test: {:?}",ret);
        
        let ans = Polygon::new(vec!( pNum(0.0,1.0),pNum(0.0,2.0),pNum(2.0,2.0),pNum(2.0,1.0) ));
        
//        assert_eq!( ret.0.points, ans.points );
//        assert_eq!( ret.1.points, ans.points );
    }
    
	#[test]
	fn test_intersect_discrete_1() {
		let l1 = Line::<f64>{p1: p64(0.0, 0.0), p2: p64(1.0, 1.0)};
		let l2 = Line::<f64>{p1: p64(0.0, 1.0), p2: p64(1.0, 0.0)};
		assert_eq!(intersect_discrete(&l1,&l2).unwrap(), p64(0.5, 0.5));
	}

	#[test]
	fn test_intersect_discrete_2() {
		let l1 = Line::<f64>{p1: p64(0.0, 0.0), p2: p64(0.25, 0.25)};
		let l2 = Line::<f64>{p1: p64(0.0, 1.0), p2: p64(1.0, 0.0)};
		assert_eq!(intersect_discrete(&l1,&l2), None);
	}

	#[test]
	fn test_intersect_infinite() {
    let l1 = Line::new(pNum(0.1, 0.3), pNum(0.25, 0.75));
    let l2 = Line::new(pNum(1.0, 0.0), pNum(1.0, 1.0));
		assert_eq!(intersect_inf(&l1,&l2).unwrap(), pNum(1.0, 3.0));

    let l1 = Line::new(pNum(2.0, 0.3), pNum(2.0, 0.75));
    let l2 = Line::new(pNum(1.0, 0.0), pNum(1.0, 1.0));
		assert_eq!(intersect_inf(&l1,&l2), None);
	}

	#[test]
	fn test_angle() {
		assert_eq!(45.0.to_radians(), angle(&p(0, 0), &p(1, 1)));
		assert_eq!(-135.0.to_radians(), angle(&p(1, 1), &p(0, 0)));
	}

	#[test]
	fn test_clockwise() {
		assert!(!Polygon::new(vec!(p(0, 0), p(1, 0), p(1, 1), p(0, 1))).is_hole());
		assert!(Polygon::new(vec!(p(0, 0), p(0, 1), p(1, 1), p(1, 0))).is_hole());
		assert!(Polygon::new(vec!(p(1, 1), p(1, 2), p(2, 2), p(2, 1))).is_hole());
	}

	#[test]
	fn test_area() {
		assert_eq!(1.0, Polygon::new(vec!(p(0, 0), p(1, 0), p(1, 1), p(0, 1))).area());
		assert_eq!(1.0, Polygon::new(vec!(p(0, 0), p(0, 1), p(1, 1), p(1, 0))).area());
		let p22 = Polygon::new(vec!(p(0, 0), p(2, 0), p(2, 2), p(0, 2)));
		assert_eq!(4.0, p22.area());
		let p44 = Polygon::new(vec!(p(0, 0), p(4, 0), p(4, 4), p(0, 4)));
		let hole12 = Polygon::new(vec!(p(1, 1), p(1, 2), p(2, 2), p(2, 1)));
		assert!(hole12.is_hole());
		assert_eq!(15.0, Shape::new(vec!(p44, hole12)).area());
	}

	#[test]
	fn test_line_coincident() {
		assert!(Line::new(p(0,0), p(0,10)).coincident(&p(0,5)));
		assert!(Line::new(p(0,0), p(0,10)).coincident(&p(0,0)));
		assert!(Line::new(p(0,0), p(0,10)).coincident(&p(0,10)));
		assert!(Line::new(p64(0.0,0.0), p64(0.0,10.0)).coincident(&p64(0.0,0.0)));
		assert!(Line::new(p64(0.0,0.0), p64(0.0,10.0)).coincident(&p64(0.0,10.0)));
		assert!(Line::new(p64(-4.0,0.0), p64(0.0,-4.0)).coincident(&p64(-2.875,-1.125)));
		assert!(!Line::new(p(0,0), p(0,10)).coincident(&p(1,5)));
		assert!(!Line::new(p(0,0), p(0,10)).coincident(&p(0,11)));
	}

	#[test]
	fn test_poly_contains() {
		assert!(Polygon::new(vec!(p(0, 0), p(2, 0), p(2, 2), p(0, 2))).contains(&p(0,0)));
		assert!(Polygon::new(vec!(p(0, 0), p(2, 0), p(2, 2), p(0, 2))).contains(&p(1,0)));
		assert!(Polygon::new(vec!(p(0, 0), p(2, 0), p(2, 2), p(0, 2))).contains(&p(1,1)));
		assert!(!Polygon::new(vec!(p(0, 0), p(2, 0), p(2, 2), p(0, 2))).contains(&p(3,3)));

		assert!(Polygon::new(vec!(p64(0.0, 0.0), p64(1.0, 0.0), p64(1.0, 1.0), p64(0.0, 1.0))).contains(&p64(0.2,0.2)));
		assert!(Polygon::new(vec!(p64(0.0, 0.0), p64(1.0, 0.0), p64(1.0, 1.0), p64(0.0, 1.0))).contains(&p64(0.2,0.7)));
		assert!(Polygon::new(vec!(p64(0.0, 0.0), p64(1.0, 0.0), p64(1.0, 1.0), p64(0.0, 1.0))).contains(&p64(0.7,0.7)));
		assert!(!Polygon::new(vec!(p64(0.0, 0.0), p64(1.0, 0.0), p64(1.0, 1.0), p64(0.0, 1.0))).contains(&p64(1.3,0.7)));
	}

	#[test]
	fn test_poly_coincident() {
		assert!(Polygon::new(vec!(p(0, 0), p(2, 0), p(2, 2), p(0, 2))).coincident(&p(0,0)));
		assert!(Polygon::new(vec!(p(0, 0), p(2, 0), p(2, 2), p(0, 2))).coincident(&p(1,0)));
		assert!(!Polygon::new(vec!(p(0, 0), p(2, 0), p(2, 2), p(0, 2))).coincident(&p(1,1)));
		assert!(!Polygon::new(vec!(p(0, 0), p(2, 0), p(2, 2), p(0, 2))).coincident(&p(3,3)));
	}

	#[test]
	fn test_contains_3() {
		assert!(!Polygon::new(vec!(p(0, 0), p(2, 0), p(2, 2), p(0, 2))).contains(&p(3,3)));
	}

	#[test]
	fn test_split() {
		let (start, mid, end) = (p64(1.0, 1.5), p64(1.125, 2.0), p64(1.25, 2.5));
		let line1 = Line::new(start.clone(), end.clone());
		assert_eq!((Line::new(start.clone(), mid.clone()), Line::new(mid.clone(), end.clone())), line1.split(0.5))
	}

	#[test]
	fn test_interpolate() {
		let line1 = Line::new(p64(0.0, 0.0), p64(1.0, 3.0));
		assert_eq!(p64(0.5, 1.5), line1.interpolate(0.5));
		assert_eq!(p64(0.125, 0.375), line1.interpolate(0.125));
	}

	#[test]
  // also exercises intersect_inf and intersect_discrete
	fn test_intersect_poly() {
		let unit_sq_p = Polygon::new(vec![Point{x: 0.0, y: 0.0}, Point{x: 0.0, y: 1.0}, Point{x:1.0, y: 1.0}, Point{x: 1.0, y: 0.0}]);

		let line1 = Line::new(p64(2.0, 0.0), p64(1.0, 3.0));
		assert_eq!(None, intersect_poly_discrete(line1, unit_sq_p.clone()));

		let line2 = Line::new(p64(0.0, 0.0), p64(1.0, 3.0));
		assert_eq!(Some((Point { x: 0.0, y: 0.0 }, Point { x: 0.3333333333333333, y: 1.0 })), intersect_poly_discrete(line2, unit_sq_p.clone()));

		let line3 = Line::new(p64(0.1, 0.3), p64(0.25, 0.75));
		assert_eq!(Some((Point { x: 0.0, y: 0.0 }, Point { x: 0.33333333333333337, y: 1.0 })), intersect_poly_inf(line3, unit_sq_p.clone()));

		let line4 = Line::new(p64(0.0, 0.0), p64(1.0, 3.0));
		assert_eq!(Some((Point { x: 0.0, y: 0.0 }, Point { x: 0.3333333333333333, y: 1.0 })), intersect_poly_inf(line4, unit_sq_p.clone()));

		let line5 = Line::new(p64(2.0, 0.0), p64(1.0, 3.0));
		assert_eq!(None, intersect_poly_inf(line5, unit_sq_p.clone()));

	}

	#[test]
	fn test_slicey_edges() {
		let unit_sq_p = Polygon::new(vec![Point{x: 0.0, y: 0.0}, Point{x: 0.0, y: 1.0}, Point{x:1.0, y: 1.0}, Point{x: 1.0, y: 0.0}]);
		let base = Polygon::new(vec!(p64(-4.0, 0.0), p64(0.0, -4.0), p64(4.0, 0.0), p64(0.0, 4.0)));

		println!("## Rotated square base, silhouette as above");
		let mut a = Polygon::new(vec!(p64(0.0, 0.0), p64(0.5, 0.0), p64(2.0, 0.5), p64(0.5, 0.5))).slicey_edges(base.clone());
		println!("Number of intersecting edges: {}", a.len());
		for edge in a.clone() {
			println!("{}", edge);
		}
		assert_eq!(4, a.len());

		println!("## Rotated square base, some inside some out");
		a = Polygon::new(vec!(p64(0.0, 0.0), p64(10.0, -10.0), p64(11.0, 0.5), p64(5.0, 5.0))).slicey_edges(base.clone());
		println!("Number of intersecting edges: {}", a.len());
		for edge in a.clone() {
			println!("{}", edge);
		}
		assert_eq!(2, a.len());

    // Unit base
		println!("## Polygon with vertices on unit sq corners/parallel lines");
		a = Polygon::new(vec!(p64(0.0, 0.0), p64(0.5, 0.0), p64(2.0, 0.5), p64(0.5, 0.5))).slicey_edges(unit_sq_p.clone());
		println!("Number of intersecting edges: {}", a.len());
		for edge in a.clone() {
			println!("{}", edge);
		}
		assert_eq!(4, a.len());

		println!("## 'normal' polygon, some inside some out");
		a = Polygon::new(vec!(p64(-1.3, -1.2), p64(0.5, -0.5), p64(2.0, 0.5), p64(0.5, 0.5))).slicey_edges(unit_sq_p.clone());
		println!("Number of intersecting edges: {}", a.len());
		for edge in a.clone() {
			println!("{}", edge);
		}
		assert_eq!(2, a.len());

		println!("## Polygon surrounds the unit sq");
		a = Polygon::new(vec!(p64(-1.0, -1.0), p64(1.5, -0.5), p64(1.5, 1.5), p64(-1.0, 1.5))).slicey_edges(unit_sq_p.clone());
		println!("Number of intersecting edges: {}", a.len());
		for edge in a.clone() {
			println!("{}", edge);
		}
		assert_eq!(0, a.len());

		println!("## Polygon contained within the unit");
		a = Polygon::new(vec!(p64(0.2, 0.2), p64(0.7, 0.2), p64(0.7, 0.7), p64(0.2, 0.7))).slicey_edges(unit_sq_p.clone());
		println!("Number of intersecting edges: {}", a.len());
		for edge in a.clone() {
			println!("{}", edge);
		}
		assert_eq!(4, a.len());
	}

	#[test]
	fn test_iterateedges() {
		let poly = Polygon::new(vec!(p(0, 0), p(1, 0), p(2, 2), p(0, 1)));
		for (i, _) in poly.edges().iter().enumerate() {
			assert!(i < poly.edges().len());
		}
	}
}
