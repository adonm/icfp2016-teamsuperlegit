use svg;
use svg::Document;
use svg::node::element;

use ::BASEPATH;
use core::*;

pub fn draw_svg<T, U>(shape: T, skel: U, filename: &str) {
	/* Draw shapes as areas and skeletons as lines */
	let data = element::path::Data::new()
					.move_to((10, 10))
					.line_by((0, 50))
					.line_by((50, 0))
					.line_by((0, -50))
					.close();
	let path = element::Path::new()
					.set("fill", "none")
					.set("stroke", "black")
					.set("stroke-width", 3)
					.set("d", data);
	let document = Document::new()
							.set("viewBox", (0, 0, 70, 70))
							.add(path);
	// only save when float coords done ok
	// svg::save(format!("{}/{}", BASEPATH, filename), &document).unwrap();
}