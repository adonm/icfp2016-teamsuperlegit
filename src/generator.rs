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

// http://stackoverflow.com/a/1968345
pub fn intersect<N: Num>(line_a: &Line<N>, line_b: &Line<N>) -> Option<Point<N>> {
    let s1 = Point{x: line_a.p2.x.clone() - line_a.p1.x.clone(), y: line_a.p2.y.clone() - line_a.p1.y.clone()};
    let s2 = Point{x: line_b.p2.x.clone() - line_b.p1.x.clone(), y: line_b.p2.y.clone() - line_b.p1.y.clone()};

    let s = (- s1.y.clone() * (line_a.p1.x.clone() - line_b.p1.x.clone()) + s1.x.clone() * (line_a.p1.y.clone() - line_b.p1.y.clone())) / (-s2.x.clone() * s1.y.clone() + s1.x.clone() * s2.y.clone());
    let t = ( s2.x.clone() * (line_a.p1.y.clone() - line_b.p1.y.clone()) - s2.y.clone() * (line_a.p1.x.clone() - line_b.p1.x.clone())) / (-s2.x.clone() * s1.y.clone() + s1.x.clone() * s2.y.clone());
    
    println!("{:?} {:?} {:?} {:?}", s1, s2, s, t);

    if (s.to_f64() >= 0.0) && (s.to_f64() <= 1.0) && (t.to_f64() >= 0.0) && (t.to_f64() <= 1.0) {
        return Some(Point{x: line_a.p1.x.clone() + t.clone()*s1.x.clone(), y: line_a.p1.y.clone() + t.clone()*s1.y.clone()})
    }

    None
}

pub fn union<N: Num>(input_a: &Polygon<N>, input_b: &Polygon<N>) -> Option<Polygon<N>> {
    for i in 0..input_a.points.len() { 
        let line1 = Line{p1: input_a.points[i].clone(), p2: input_a.points[(i+1)%input_a.points.len()].clone()};
        for j in 0..input_b.points.len() {
            let line2 = Line{p1: input_b.points[j].clone(), p2: input_b.points[(j+1)%input_b.points.len()].clone()};
            println!("{:?} comp {:?}", line1, line2);
            let join = intersect(&line1, &line2);
            match join {
                Some(x) => {println!("HIT {:?}", x);},
                None => {println!("MISS");},
            }
        }
    }
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
    fn test_intersect_1() {
        let l1 = Line::<f64>{p1: p(0.0, 0.0), p2: p(1.0, 1.0)};
        let l2 = Line::<f64>{p1: p(0.0, 1.0), p2: p(1.0, 0.0)};
        assert_eq!(intersect(&l1,&l2).unwrap(), p(0.5, 0.5));
    }

    #[test]
    fn test_intersect_2() {
        let l1 = Line::<f64>{p1: p(0.0, 0.0), p2: p(0.25, 0.25)};
        let l2 = Line::<f64>{p1: p(0.0, 1.0), p2: p(1.0, 0.0)};
        assert_eq!(intersect(&l1,&l2), None);
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
