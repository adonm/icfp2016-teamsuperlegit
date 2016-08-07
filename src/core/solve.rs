use super::*;
use super::super::matrix::Matrix33;

// l0.p2 and l1.p1 are the same since this is where the lines join
// l0 and l1 must be perpendicular
pub fn square_from_corner<N:Num>(line0: &Line<N>, line1: &Line<N>) -> Polygon<N> {
	// lets start with a unit square, and rotate by angle of one line
	// then we can translate so origin matches
	let unit_sq_p = Polygon::new(vec![
		Point{x:N::zero(), y:N::zero()}, Point{x:N::zero(), y:N::one()},
		Point{x:N::one(), y:N::one()}, Point{x:N::one(), y:N::zero()}
	]);
	// swap coords on lines around to make them touch at expected point
	let line0sw = Line{p1: line0.p2.clone(), p2: line0.p1.clone()};
	let line1sw = Line{p1: line1.p2.clone(), p2: line1.p1.clone()};
	let l0 = if line0.p1 == line1.p2 { &line0sw } else { line0 };
	let l1 = if line0.p1 == line1.p2 { &line1sw } else { line1 };
	if l0.p2 != l1.p1 {
		panic!("Lines must join {}, {}; {}, {}", l0.p1, l0.p2, l1.p1, l1.p2);
	}
	let mut transform = Matrix33::translate(l0.p2.clone().x, l0.p2.clone().y);
	let o = l0.p1.clone().x - l0.p2.clone().x;
	let a = l0.p1.clone().y - l0.p2.clone().y;
	let h = N::from_f64((o.clone() * o.clone() + a.clone() * a.clone()).to_f64().sqrt());
	transform *= Matrix33::rotate(- o.clone()/h.clone(), a.clone()/h.clone());
	let mut points = Vec::new();
	for point in unit_sq_p.points {
		points.push(transform.transform(point));
	}
	let mut poly = Polygon::new(points);
	poly.transform = transform;
	return poly;
}

// This function figures out the next line to fold along
pub fn get_next_edge_to_fold<N: Num>(base: Polygon<N>, silhouette: Polygon<N>) -> Result<Line<N>, bool> {
	let candidates: Vec<Line<N>> = silhouette.slicey_edges(base.clone());

	if candidates.len() == 0 { return Err(false) }
	let mut longest: Result<Line<N>, bool>  = Err(false);// = candidates[0].clone();
	for line in candidates {
		println!("get_next_edge_to_fold: considering {}, length {}", line, line.len());
		if (longest.clone().is_err() || line.len() > longest.clone().unwrap().len()) && ( base.clone().points.contains(&line.p1)==false || base.clone().points.contains(&line.p2)==false ) {
			longest = Ok(line.clone());
            println!("did make longest");
		}
	}
    
    return longest;
}

pub fn fold_origami<N: Num>(state: &Vec<(Polygon<N>)>, vertex1: &Point<N>, vertex2: &Point<N>, anchor: &Point<N>) -> Vec<Polygon<N>>{

    let mut newState = vec!();
    
    for poly in state {
        if can_fold(&poly, &vertex1, &vertex2){
            
            let (poly1, poly2) = fold_polygon(&poly,&vertex1,&vertex2,&anchor);
            
            newState.push(poly1);
            newState.push(poly2);
            
        } else {
            newState.push(poly.clone());
        }
    }
    
    newState
}



#[cfg(test)]
mod tests {
	use super::*;
	use super::super::geom::*;
	use super::super::tests::*;

	#[test]
	fn square_from_corner_test(){
		let l1 = Line{ p1: p(-1.0/2.0,1.0/2.0), p2: p(0.0,0.0) };
		let l2 = Line{ p1: p(0.0,0.0), p2: p(1.0/2.0,1.0/2.0) };

		let poly = square_from_corner(&l1,&l2);

		println!("{:?}",poly);
	}

	#[test]
	fn test_get_next_edge_to_fold() {
		println!("## Unit square base, silhouette as above");
		let mut base = Polygon::new(vec![Point{x: 0.0, y: 0.0}, Point{x: 0.0, y: 1.0}, Point{x:1.0, y: 1.0}, Point{x: 1.0, y: 0.0}]);
		let mut a = Polygon::new(vec!(p(0.0, 0.0), p(0.5, 0.0), p(2.0, 0.5), p(0.5, 0.5)));

		let result: Line<f64> = get_next_edge_to_fold(base, a).unwrap();
		println!("Folding along edge {} -> {}", result.p1, result.p2);
		assert_eq!(Point{x: 0.0, y: 0.5}, result.p1);
		assert_eq!(Point{x: 1.0, y: 0.5}, result.p2);

		println!("## Rotated square base, silhouette as above");
		base = Polygon::new(vec!(p(-4.0, 0.0), p(0.0, -4.0), p(4.0, 0.0), p(0.0, 4.0)));
		a = Polygon::new(vec!(p(-1.0, 0.5), p(1.0, 0.5), p(1.0, 1.0), p(-1.0, 1.0)));

		let result: Line<f64> = get_next_edge_to_fold(base, a).unwrap();
		println!("Folding along edge {} -> {}", result.p2, result.p2);
		assert_eq!(Point{x: -3.5, y: 0.5}, result.p1);
		assert_eq!(Point{x: 3.5, y: 0.5}, result.p2);

	}
}
