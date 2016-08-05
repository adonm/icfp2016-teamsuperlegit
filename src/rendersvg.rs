use svg;
use svg::Document;
use svg::node::element;

use ::BASEPATH;
use core::*;

pub fn draw_svg<T: Num>(shape: Shape<T>, skel: Skeleton<T>, filename: &str) {
	/* Draw shapes as areas and skeletons as lines */
	let mut document = Document::new().set("viewBox", (0, 0, 1, 1));
	for polygon in shape {
		let mut iter = polygon.points.iter();		
		let startpoint = iter.next().unwrap();
		let mut data = element::path::Data::new().move_to((startpoint.x.to_f64(), startpoint.y.to_f64()));
		// path.move_to(shape.points[0] (as float))
		for point in iter {
			println!("{:?}, {:?}", point.x.to_f64(), point.y.to_f64());
			data = data.line_by((point.x.to_f64(), point.y.to_f64()));
		}
		let path = element::Path::new()
				.set("fill", "none")
				.set("stroke", "black")
				.set("stroke-width", 3)
				.set("d", data);
		document = document.add(path);
	}
	// only save when float coords done ok
	svg::save(format!("{}/{}", BASEPATH, filename), &document).unwrap();
}