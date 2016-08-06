use super::*;

// l0.p2 and l1.p1 are the same since this is where the lines join
// l0 and l1 must be perpendicular
pub fn square_from_corner<N:Num>(line0: &Line<N>, line1: &Line<N>) -> Polygon<N> {
	let line0sw = Line{p1: line0.p2.clone(), p2: line0.p1.clone()};
	let line1sw = Line{p1: line1.p2.clone(), p2: line1.p1.clone()};
	let l0 = if line0.p1 == line1.p2 { &line0sw } else { line0 };
	let l1 = if line0.p1 == line1.p2 { &line1sw } else { line1 };
	if l0.p2 != l1.p1 {
		panic!("Lines must join {}, {}; {}, {}", l0.p1, l0.p2, l1.p1, l1.p2);
	}

	let p1 = normalize_line(&l0.p2, &(&l0.p1-&l0.p2));
	let p2 = normalize_line(&p1, &(&l1.p2-&l1.p1));
	let poly = Polygon::new(vec!(
		l0.p2.clone(),
		p1,
		p2,
		normalize_line(&l0.p2, &(&l1.p2-&l0.p2))));

	poly
}

// This function figures out the next line to fold along
pub fn get_next_edge_to_fold(base: Polygon<f64>, silhouette: Polygon<f64>) -> (Point<f64>, Point<f64>) {
	let mut candidates: Vec<Line<f64>> = silhouette.slicey_edges(base);

	let mut longest: Line<f64> = candidates[0].clone();
	for line in candidates {
	  println!("get_next_edge_to_fold: considering {}, length {}", line, line.len());
		if line.len() > longest.len() {
			longest = line;
		}
	}

	return (longest.p1.clone(), longest.p2.clone());
}

pub fn fold_origami<N: Num>(state: &Vec<(Polygon<N>)>, vertex1: &Point<N>, vertex2: &Point<N>) -> Vec<Polygon<N>>{

    let mut newState = vec!();
    
    for poly in state {
        if can_fold(&poly, &vertex1, &vertex2){
            
            let (poly1, poly2) = fold_polygon(&poly,&vertex1,&vertex2);
            
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
		let l1 = Line{ p1: p64(-1.0/2.0,1.0/2.0), p2: p64(0.0,0.0) };
		let l2 = Line{ p1: p64(0.0,0.0), p2: p64(1.0/2.0,1.0/2.0) };

		let poly = square_from_corner(&l1,&l2);

		println!("{:?}",poly);
	}

	#[test]
	fn test_get_next_edge_to_fold() {
		println!("## Unit square base, silhouette as above");
		let mut base = Polygon::new(vec![Point{x: 0.0, y: 0.0}, Point{x: 0.0, y: 1.0}, Point{x:1.0, y: 1.0}, Point{x: 1.0, y: 0.0}]);
		let mut a = Polygon::new(vec!(p64(0.0, 0.0), p64(0.5, 0.0), p64(2.0, 0.5), p64(0.5, 0.5)));

		let result: (Point<f64>, Point<f64>) = get_next_edge_to_fold(base, a);
		println!("Folding along edge {} -> {}", result.0, result.1);
		assert_eq!(Point{x: 0.0, y: 0.0}, result.0);
		assert_eq!(Point{x: 1.0, y: 1.0}, result.1);

		println!("## Rotated square base, silhouette as above");
		base = Polygon::new(vec!(p64(-4.0, 0.0), p64(0.0, -4.0), p64(4.0, 0.0), p64(0.0, 4.0)));
		a = Polygon::new(vec!(p64(-1.0, 0.5), p64(1.0, 0.5), p64(1.0, 1.0), p64(-1.0, 1.0)));

		let result: (Point<f64>, Point<f64>) = get_next_edge_to_fold(base, a);
		println!("Folding along edge {} -> {}", result.0, result.1);
		assert_eq!(Point{x: -3.5, y: 0.5}, result.0);
		assert_eq!(Point{x: 3.5, y: 0.5}, result.1);

	}
}
