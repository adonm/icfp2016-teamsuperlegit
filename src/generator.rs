extern crate num;
use num::rational::BigRational;
use num::ToPrimitive;

pub use core::*;

pub struct FoldState<N: Num> {
    pub source_facets: Vec<Polygon<N>>,
    pub dest_facets: Vec<Polygon<N>>
}

pub fn slice<N: Num>(input: &Polygon<N>, axis: Line<N>) -> Option<(Polygon<N>, Polygon<N>)> {
    None
}

pub fn fold<N: Num>(input: &FoldState<N>, axis: Line<N>) -> Option<FoldState<N>> {
    None
}


// http://stackoverflow.com/questions/2667748/how-do-i-combine-complex-polygons
pub fn union<N: Num>(a: &Polygon<N>, b: &Polygon<N>) -> Option<Polygon<N>> {
    let mut input_a = a.clone();
    let mut input_b = b.clone();

    let mut points: Vec<Point<N>> = Vec::new();
    let mut graph: Vec<Vec<usize>> = Vec::new();
    
    let len_a = input_a.points.len();
    let len_b = input_b.points.len();
    let mut min_x = input_a.points[0].x.clone();
    let mut min_y = input_a.points[0].y.clone();
    for i in 0..len_a {
        let ref p = input_a.points[((i+1)%len_a)];
        points.push(p.clone());
        graph.push(vec!(i, ((i+2)%len_a)));
        if p.x < min_x {
            min_x = p.x.clone();
        }
        if p.y < min_y {
            min_y = p.y.clone();
        }
    }

    for i in 0..len_b {
        let ref p = input_b.points[((i+1)%len_b)];
        points.push(p.clone());
        graph.push(vec!(i+len_a, ((i+2)%len_b)+len_a));
        if p.x < min_x {
            min_x = p.x.clone();
        }
        if p.y < min_y {
            min_y = p.y.clone();
        }
    }
    
    for i in 0..input_a.points.len() { 
        let line1 = Line{p1: input_a.points[i].clone(), p2: input_a.points[(i+1)%len_a].clone()};
        for j in 0..input_b.points.len() {
            let line2 = Line{p1: input_b.points[j].clone(), p2: input_b.points[(j+1)%len_b].clone()};
            println!("{:?} comp {:?}", line1, line2);
            let join = intersect_discrete(&line1, &line2);
            match join {
                Some(x) => {
                    let point_id = points.len();
                    points.push(x.clone());
                    let conn_points = vec!(i, ((i+1)%len_a), len_a+j, len_a+((j+1)%len_b));
                    graph.push(conn_points.clone());
                    for conn in conn_points {
                        graph[conn].push(point_id);
                    }
                    println!("HIT {:?}", x);
                },
                None => {
                    println!("MISS");
                },
            }
        }
    }
    println!("MINIMA {:?} {:?}", min_x, min_y);
    println!("POINTS KACHING {:?}", points);
    println!("GRAFF {:?}", graph);
    



    None
}

#[cfg(test)]
mod tests {
    use super::*;
	extern crate num;
	use self::num::rational::BigRational;
	use num::Float;

	fn p(x: f64, y: f64) -> Point<f64> {
		Point{x: x, y: y}
	}

    #[test]
    fn test_union_1() {
        let p1 = Polygon::new(vec!(p(0.0, 0.0), p(0.0, 1.0), p(1.0, 1.0), p(1.0, 0.0)));
        let p2 = Polygon::new(vec!(p(0.5, 0.5), p(0.5, 1.5), p(1.5, 1.5), p(1.5, 0.5)));
        let pu = union(&p1, &p2);
        assert_eq!(1, 0);
        //assert_eq!(union(&p1,&p2), 
    }

}
