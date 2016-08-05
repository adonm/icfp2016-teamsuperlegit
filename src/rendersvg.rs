use svg;
use svg::Document;
use svg::node::element;

use ::BASEPATH;
use core::*;

pub fn draw_svg<N: Num>(shape: Shape<N>, skel: Skeleton<N>, filename: &str) {
	/* Draw shapes as areas and skeletons as lines */
	let mut document = Document::new().set("viewBox", (0, 0, 1, 1));
	for polygon in shape {
		let mut iter = polygon.points.iter();		
		let startpoint = iter.next().unwrap();
		let mut data = element::path::Data::new().move_to((startpoint.x.to_f64(), startpoint.y.to_f64()));
		println!("{:?}, {:?}", startpoint.x.to_f64(), startpoint.y.to_f64());
		// path.move_to(shape.points[0] (as float))
		for point in iter {
			println!("{:?}, {:?}", point.x.to_f64(), point.y.to_f64());
			data = data.line_to((point.x.to_f64(), point.y.to_f64()));
		}
        data = data.close();
		let path = element::Path::new()
				.set("fill", "none")
				.set("stroke", "black")
				.set("stroke-width", 0.02)
				.set("d", data);
		document = document.add(path);
	}
    for bone in skel {
        let line = element::Line::new()
                .set("fill", "none")
                .set("stroke", "crimson")
                .set("stroke-width", 0.01)
                .set("stroke-dasharray", "0.01,0.01")
                .set("x1", bone.p1.x.to_f64())
                .set("y1", bone.p1.y.to_f64())
                .set("x2", bone.p2.x.to_f64())
                .set("y2", bone.p2.y.to_f64());
        document = document.add(line);
    }
	// only save when float coords done ok
	svg::save(format!("{}/{}", BASEPATH, filename), &document).unwrap();
}
