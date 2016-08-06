/* vim: set noexpandtab : */

extern crate num;
use num::rational::BigRational;

mod generic;
mod geom;
mod solve;

pub use self::generic::*;
pub use self::geom::*;
pub use self::solve::*;

#[cfg(test)]
mod tests {
	use super::*;
	extern crate num;
	use self::num::rational::BigRational;
	use num::Float;

	fn p(x: i32, y: i32) -> Point<i32> {
		Point{x: x, y: y}
	}

	fn p64(x: f64, y: f64) -> Point<f64> {
		Point{x: x, y: y}
	}
    
    fn pNum<N:Num>(x: N, y:N) -> Point<N> {
        Point{x: x, y: y}
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
	fn flip_point_test(){
        let p1 = pNum(1,1);
        let v1 = pNum(0,0);
        let v2 = pNum(0,1);
        let p2 = flip_point(&p1,&v1,&v2);
        println!("flip_point_test: {:?}",p2);
	}

	#[test]
	fn square_from_corner_test(){
		let l1 = Line{ p1: p64(-1.0/2.0,1.0/2.0), p2: p64(0.0,0.0) };
		let l2 = Line{ p1: p64(0.0,0.0), p2: p64(1.0/2.0,1.0/2.0) };

		let poly = square_from_corner(&l1,&l2);

		println!("{:?}",poly);
	}

	#[test]
	fn test_is_convex_1(){
		let l1 = Line{ p1: p(1,1), p2: p(2,2) };
		let l2 = Line{ p1: p(2,2), p2: p(3,1) };

		assert!(is_convex(&l1,&l2)==false);
	}

	#[test]
	fn test_is_convex_2(){
		let l1 = Line{ p1: p(1,1), p2: p(2,2) };
		let l2 = Line{ p1: p(2,2), p2: p(3,4) };

		assert!(is_convex(&l1,&l2)==true);
	}

	#[test]
	fn test_intersect_lines_1() {
		let l1 = Line::<f64>{p1: p64(0.0, 0.0), p2: p64(1.0, 1.0)};
		let l2 = Line::<f64>{p1: p64(0.0, 1.0), p2: p64(1.0, 0.0)};
		assert_eq!(intersect_lines(&l1,&l2).unwrap(), p64(0.5, 0.5));
	}

	#[test]
	fn test_intersect_lines_2() {
		let l1 = Line::<f64>{p1: p64(0.0, 0.0), p2: p64(0.25, 0.25)};
		let l2 = Line::<f64>{p1: p64(0.0, 1.0), p2: p64(1.0, 0.0)};
		assert_eq!(intersect_lines(&l1,&l2), None);
	}

	#[test]
	fn test_ops() {
		assert_eq!(p(5, 7), p(2, 4) + p(3, 3));
		assert_eq!(p(-1, 1), p(2, 4) - p(3, 3));
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
	fn test_longest() {
		assert_eq!((p(1,0), p(2,2)), Polygon::new(vec!(p(0, 0), p(1, 0), p(2, 2), p(0, 1))).longest_edge());
		assert_eq!(2.0, Line::new(p(0, 0), p(2, 0)).len());
	}

	#[test]
	fn test_iterateedges() {
		let poly = Polygon::new(vec!(p(0, 0), p(1, 0), p(2, 2), p(0, 1)));
		for (i, edge) in poly.edges().iter().enumerate() {
			assert!(i < poly.edges().len());
		}
	}

	#[test]
	fn test_line_coincident() {
		assert!(Line::new(p(0,0), p(0,10)).coincident(&p(0,5)));
		assert!(Line::new(p(0,0), p(0,10)).coincident(&p(0,0)));
		assert!(Line::new(p(0,0), p(0,10)).coincident(&p(0,10)));
		assert!(Line::new(p64(0.0,0.0), p64(0.0,10.0)).coincident(&p64(0.0,0.0)));
		assert!(Line::new(p64(0.0,0.0), p64(0.0,10.0)).coincident(&p64(0.0,10.0)));
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
	fn test_float() {
		assert_eq!(0.5f64, "4328029871649615121465353437184/8656059743299229793415925725865".parse::<BigRational>().unwrap().to_f64());
		assert_eq!(0.25f64, "1/4".parse::<BigRational>().unwrap().to_f64());
		assert_eq!(1.1f64, "11/10".parse::<BigRational>().unwrap().to_f64());
	}

	#[test]
	fn test_lowest_unit_vertex() {
		let a = Polygon::new(vec!(p64(0.0, 0.0), p64(0.5, 0.0), p64(1.0, 0.5), p64(0.5, 0.5)));
		assert_eq!(p64(0.0,0.0), a.lowest_unit_vertex().unwrap());

		let b = Polygon::new(vec!(p64(0.0, 0.7), p64(0.5, 0.0), p64(1.0, 0.5), p64(0.5, 0.9)));
		assert_eq!(p64(0.5,0.0), b.lowest_unit_vertex().unwrap());

		let c = Polygon::new(vec!(p64(0.0, 2.0), p64(1.0, 2.0), p64(1.0, 3.0), p64(0.0, 3.0)));
		assert_eq!(None, c.lowest_unit_vertex());
	}

	#[test]
	fn test_interpolate() {
		let line1 = Line::new(p64(0.0, 0.0), p64(1.0, 3.0));
		assert_eq!(p64(0.5, 1.5), line1.interpolate(0.5));
		assert_eq!(p64(0.125, 0.375), line1.interpolate(0.125));
	}

	#[test]
	fn test_split() {
		let (start, mid, end) = (p64(1.0, 1.5), p64(1.125, 2.0), p64(1.25, 2.5));
		let line1 = Line::new(start.clone(), end.clone());
		assert_eq!((Line::new(start.clone(), mid.clone()), Line::new(mid.clone(), end.clone())), line1.split(0.5))
	}

	#[test]
	fn test_commutivity() {
		let (p1, p2) = (p64(1.0, 1.5), p64(1.25, 2.5));
		assert_eq!(p64(2.25, 4.0), &p1 + &p2);
		assert_eq!(p64(2.25, 4.0), &p2 + &p1);
		assert_eq!(p64(3.5, 6.5), &p1 + &(p2.scale(2.0)));
		assert_eq!(p64(3.25, 5.5), &(p1.scale(2.0)) + &p2);

		assert_eq!(p64(0.25, 1.0), p2 - &p1);
	}

	#[test]
	fn test_intersect_poly() {
		let unit_sq_p = Polygon::new(vec![Point{x: 0.0, y: 0.0}, Point{x: 0.0, y: 1.0}, Point{x:1.0, y: 1.0}, Point{x: 1.0, y: 0.0}]);

		let line1 = Line::new(p64(2.0, 0.0), p64(1.0, 3.0));
		assert_eq!(None, intersect_poly_discrete(line1, unit_sq_p.clone()));

		let line2 = Line::new(p64(0.0, 0.0), p64(1.0, 3.0));
		assert_eq!(Some((Point { x: 0.0, y: 0.0 }, Point { x: 0.3333333333333333, y: 1.0 })), intersect_poly_discrete(line2, unit_sq_p.clone()));

		let line2 = Line::new(p64(0.1, 0.3), p64(0.25, 0.75));
		assert_eq!(Some((Point { x: 0.0, y: 0.6666666666666667 }, Point { x: 1.0, y: 1.0 })), intersect_poly_inf(line2, unit_sq_p.clone()));
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
		let mut a = Polygon::new(vec!(p64(0.0, 0.0), p64(10.0, -10.0), p64(11.0, 0.5), p64(5.0, 5.0))).slicey_edges(base.clone());
		println!("Number of intersecting edges: {}", a.len());
		for edge in a.clone() {
			println!("{}", edge);
		}
		assert_eq!(2, a.len());

    // Unit base
		println!("## Polygon with vertices on unit sq corners/parallel lines");
		let mut a = Polygon::new(vec!(p64(0.0, 0.0), p64(0.5, 0.0), p64(2.0, 0.5), p64(0.5, 0.5))).slicey_edges(unit_sq_p.clone());
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
