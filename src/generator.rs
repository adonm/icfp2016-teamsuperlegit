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
    let s1 = Point{x: line_a.p2.x.clone() - line_a.p1.x.clone(), y: line_a.p2.y.clone() - line_a.p2.x.clone()};
    let s2 = Point{x: line_b.p2.x.clone() - line_b.p1.x.clone(), y: line_b.p2.y.clone() - line_b.p2.x.clone()};

    let s = (- s1.y.clone() * (line_a.p1.x.clone() - line_b.p1.x.clone()) + s1.x.clone() * (line_a.p1.y.clone() - line_b.p1.y.clone())) / (-s2.x.clone() * s1.y.clone() + s1.x.clone() * s2.y.clone());
    let t = ( s2.x.clone() * (line_a.p1.y.clone() - line_b.p1.y.clone()) - s2.y.clone() * (line_a.p1.x.clone() - line_b.p1.x.clone())) / (-s2.x.clone() * s1.y.clone() + s1.x.clone() * s2.y.clone());
    
    if (s.to_f64() >= 0.0) && (s.to_f64() <= 1.0) && (t.to_f64() >= 0.0) && (t.to_f64() <= 1.0) {
        return Some(Point{x: line_a.p1.x.clone() + t.clone()*s1.x.clone(), y: line_a.p1.y.clone() + t.clone()*s1.y.clone()})
    }

    None
}

pub fn union<N: Num>(input_a: &Polygon<N>, input_b: &Polygon<N>) -> Option<Polygon<N>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
	extern crate num;
	use self::num::rational::BigRational;
	use num::Float;

    #[test]
    fn test_intersect_1() {
        let l1 = Line::<f64>{p1: Point::<f64>{x: 0.0, y: 0.0}, p2: Point::<f64>{x: 1.0, y: 1.0}};
        let l2 = Line::<f64>{p1: Point::<f64>{x: 0.0, y: 1.0}, p2: Point::<f64>{x: 1.0, y: 0.0}};
        assert_eq!(intersect(&l1,&l2).unwrap(), Point::<f64>{x: 0.5, y: 0.5});
    }
}
