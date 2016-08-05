extern crate num;
extern crate rustc_serialize;
extern crate svg;
use rustc_serialize::json::Json;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;
use std::time::Duration;
use std::thread;

mod core;
mod parse;
mod rendersvg;

use num::rational::BigRational;

pub const BASEPATH: &'static str = "icfp2016problems";

fn download(filename: &str, apipathname: &str) {
	use std::process::Command;
	// Get latest snapshot
	let path = format!("{}/{}", BASEPATH, filename);
	let path_arg = path.clone();
	let output = Path::new(&path);
	if !output.exists() {
		println!("{:?}", 
			Command::new("curl").arg("--compressed").arg("-L").arg("-H").arg("Expect:").arg("-H")
			.arg(format!("X-API-Key: 60-d7840e0fce3dc9e9a4e2693153ccd9bc"))
			.arg(format!("http://2016sv.icfpcontest.org/api/{}", &apipathname))
			.arg("-o").arg(path_arg)
			.output()
			.expect("uh-oh")
		);
		thread::sleep(Duration::from_millis(1000));
	}
}

fn get_contest_meta() -> Vec<Json> {
	download("contest_list.json", "snapshot/list");
	let mut file = fs::File::open(format!("{}/contest_list.json", BASEPATH)).unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();
	let json = Json::from_str(&data).unwrap();
	let contest_blobs = json.find_path(&["snapshots"]).unwrap().as_array().unwrap();
	let latest_snapshot = contest_blobs[contest_blobs.len()-1].find_path(&["snapshot_hash"]).unwrap().as_string().unwrap();
	let snapshot_blob = format!("blob/{}", latest_snapshot);
	// grab latest snapshot
	download("contest.json", &snapshot_blob);
	file = fs::File::open(format!("{}/contest.json", BASEPATH)).unwrap();
	data = String::new();
	file.read_to_string(&mut data).unwrap();
	let json = Json::from_str(&data).unwrap();
	return json.find_path(&["problems"]).unwrap().as_array().unwrap().clone();
}

fn save_problems(problems: Vec<Json>) -> Vec<Json> {
	for problem in &problems {
		let hash = problem.find_path(&["problem_spec_hash"]).unwrap().as_string().unwrap();
		let id = problem.find_path(&["problem_id"]).unwrap().as_i64().unwrap();
		let blob = format!("blob/{}", hash);
		let filename = format!("{:03}.problem.txt", id);
		download(&filename, &blob);
	}
	return problems
}

fn draw_problems(problems: Vec<Json>) -> Vec<Json> {

	for problem in &problems {
		let id = problem.find_path(&["problem_id"]).unwrap().as_i64().unwrap();
		let filename = format!("{:03}.problem.svg", id);
		let mut problem_txt = String::new();
		let file = fs::File::open(format!("{}/{:03}.problem.txt", BASEPATH, id)).unwrap();
		let (shape, skeleton) = parse::parse::<BigRational, std::fs::File>(file).unwrap();
		rendersvg::draw_svg(shape, skeleton, &filename)
	}
	return problems
}

fn main() {
	// setup directories for outputs
	fs::create_dir_all(BASEPATH);
	// grab contest snapshots
	let problems = get_contest_meta();
	// save the problem blobs
	let problems = save_problems(problems);
	// draw svgs of each one
	let problems = draw_problems(problems);
	// solve_problems(problems)?
}
