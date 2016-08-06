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
            
            let (poly1, poly2Old) = split_polygon(&poly,&vertex1,&vertex2);
            
            let poly2 = fold_polygon(&poly2Old, &vertex1, &vertex2);
            
            newState.push(poly1);
            newState.push(poly2);
            
        } else {
            newState.push(poly.clone());
        }
    }
    
    newState
}

