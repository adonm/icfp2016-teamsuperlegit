use svg;
use svg::Document;
use svg::node::element;

use ::BASEPATH;
use core::*;
use write::from_polys;
use std;
use std::io::Write;

fn draw_polygon<N: Num>(polygon: &Polygon<N>, fill: &str) -> element::Polygon {
	let mut points = String::from("");
	for point in polygon.clone().points.iter() {
		let coord = format!("{},{} ", point.x.to_f64(), point.y.to_f64());
		points.push_str(&coord);
	}
	let poly = element::Polygon::new()
		.set("fill", fill).set("fill-opacity", "0.5")
		.set("stroke", "black").set("stroke-opacity", "0.5")
		.set("stroke-width", 0.005)
		.set("points", points.trim());
	return poly;
}

pub fn draw_svg<N: Num>(shape: Shape<N>, skel: Skeleton<N>, filename: &str) {
	let filename = format!("{}/{}", BASEPATH, filename);
	/* Draw shapes as areas and skeletons as lines */
	let mut document = Document::new().set("viewBox", (-1, -1, 3, 3))
		.set("width", "600px").set("height", "600px"); // scale stuff nice
	let mut defs = element::Definitions::new();
	let marker_path_start = element::Path::new()
		.set("fill", "crimson")
		.set("d",  "M 0.0,0.0 L 5.0,-5.0 L -12.5,0.0 L 5.0,5.0 L 0.0,0.0 z")
		.set("transform", "scale(0.2) rotate(180) translate(10,0)");
	let marker_path_end = element::Path::new()
		.set("fill", "crimson")
		.set("d",  "M 0.0,0.0 L 5.0,-5.0 L -12.5,0.0 L 5.0,5.0 L 0.0,0.0 z")
		.set("transform", "scale(0.2) rotate(180) translate(10,0)");
	let marker_start = element::Marker::new()
		.set("style", "overflow:visible")
		.set("orient", "auto")
		.set("id", "ArrowStart")
		.add(marker_path_start);
	let marker_end = element::Marker::new()
		.set("style", "overflow:visible")
		.set("orient", "auto")
		.set("id", "ArrowEnd")
		.add(marker_path_end);
	defs = defs.add(marker_start);
	defs = defs.add(marker_end);
	document = document.add(defs);

	// draw silhouette
	let mut silhouette = element::Group::new();
	let mut corners = element::Group::new();
	let mut anchorcnr: Result<(Line<N>, Line<N>), bool> = Err(false);
	let mut anchorlength = 0.0_f64;
	let mut psquare: Result<Polygon<N>, bool> = Err(false);
	for polygon in shape.clone().polys {
		let poly = if polygon.is_hole() {
			// holes are green
			draw_polygon(&polygon, "#2dff47")
		} else {
			// silhouettes are pink
			draw_polygon(&polygon, "#ff2df7")
		};
		if polygon.square() {
			psquare = Ok(polygon.clone());
		}
		silhouette = silhouette.add(poly);
		// highlight corners
		for corner in polygon.corners() {
			let mut length = 0.0_f64;
			let (p1, p2) = (corner.0.p1.clone(), corner.0.p2.clone());
			let line1 = element::Line::new()
				.set("x1", p1.x.to_f64()).set("y1", p1.y.to_f64())
				.set("x2", p2.x.to_f64()).set("y2", p2.y.to_f64())
				.set("stroke", "#00ff00").set("stroke-opacity", 0.5).set("stroke-width", 0.007);
			length += p_distance(&p1, &p2);
			corners = corners.add(line1);
			let (p1, p2) = (corner.1.p1.clone(), corner.1.p2.clone());
			let line2 = element::Line::new()
				.set("x1", p1.x.to_f64()).set("y1", p1.y.to_f64())
				.set("x2", p2.x.to_f64()).set("y2", p2.y.to_f64())
				.set("stroke", "#00ff00").set("stroke-opacity", 0.5).set("stroke-width", 0.007);
			length += p_distance(&p1, &p2);
			corners = corners.add(line2);
			if length > anchorlength {
				anchorcnr = Ok(corner.clone());
				anchorlength = length;
			}
		}
	}
	document = document.add(silhouette);

	// draw skeleton
	let mut skeleton = element::Group::new();
	for bone in skel.lines() {
		let skel_data = element::path::Data::new()
			.move_to((bone.p1.x.to_f64(), bone.p1.y.to_f64()))
			.line_to((bone.p2.x.to_f64(), bone.p2.y.to_f64()));
		let skel_path = element::Path::new()
			.set("fill", "none")
			.set("stroke", "crimson")
			.set("stroke-width", 0.003)
			.set("stroke-dasharray", "0.01,0.01")
			.set("marker-start", "url(#ArrowStart)")
			.set("marker-end", "url(#ArrowEnd)")
			.set("d", skel_data);
		skeleton = skeleton.add(skel_path);
	}
	document = document.add(skeleton);

	// corners ontop looks nicer
	document = document.add(corners);
	if anchorcnr != Err(false) {
		let (l1, l2) = anchorcnr.unwrap();
		let unitsquare = square_from_corner(&l1, &l2);

		// draw intersect vertices
		let ref poly = shape.polys[0];
		let foldedge = get_next_edge_to_fold(unitsquare.clone(), poly.clone());
		if foldedge.is_ok() {
			let (p1, p2) = foldedge.unwrap();
			for p in [p1.clone(), p2.clone()].iter() {
				let intersect = element::Circle::new()
					.set("cx", p.x.to_f64()).set("cy", p.y.to_f64())
					.set("fill", "#f00").set("fill-opacity", "0.5")
					.set("r", "0.01");
				document = document.add(intersect);
			}
			let mut state = Vec::new();
			let mut statepolys = element::Group::new();
			state.push(unitsquare.clone());
			for polygon in state.clone() {
				statepolys = statepolys.add(draw_polygon(&polygon, "#ff0"));
			}
			let folded = fold_origami(&state, &p1, &p2, &unitsquare.clone().points[0]);
			println!("folded {} into {}", filename, folded.len());
			for polygon in folded.clone() {
				statepolys = statepolys.add(draw_polygon(&polygon, "#000"));
			}
			document = document.add(statepolys);
			let writer = std::fs::File::create(filename.clone().replace("problem.svg", "test-solution.txt")).unwrap();
			let unfolded = from_polys(writer, folded).unwrap();
			let mut unfoldedpolys = element::Group::new();
			for polygon in unfolded.clone() {
				unfoldedpolys = unfoldedpolys.add(draw_polygon(&polygon, "#00f"));
			}
			document = document.add(unfoldedpolys);
		} else {
			// just draw anchored square
			document = document.add(draw_polygon(&unitsquare, "#000"));
		}

		if psquare != Err(false) {
			let p = psquare.unwrap().clone();
			if p.area() == unitsquare.area() {
				let points = format!("{}\n{}\n{}\n{}", p.points[0], p.points[1], p.points[2], p.points[3]);
				println!("Simple solution found for {}, saving", filename);
				let mut f = std::fs::File::create(filename.clone().replace("problem.svg", "solution.txt")).unwrap();
				f.write_all(format!("4\n0,0\n1,0\n1,1\n0,1\n1\n4 0 1 2 3\n{}", points).as_bytes()).unwrap();
			}
		}
	}

	// save to file
	svg::save(filename, &document).unwrap();
}
