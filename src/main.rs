extern crate num;
extern crate rustc_serialize;
extern crate svg;
use std::vec::Vec;
use rustc_serialize::json::Json;

mod core;
mod parse;
mod rendersvg;
mod restapi;

use num::rational::BigRational;

pub const BASEPATH: &'static str = "icfp2016problems";

fn draw_problems(problems: Vec<Json>) -> Vec<Json> {
	for problem in &problems {
		let id = problem.find_path(&["problem_id"]).unwrap().as_i64().unwrap();
		let filename = format!("{:03}.problem.svg", id);
		let file = std::fs::File::open(format!("{}/{:03}.problem.txt", BASEPATH, id)).unwrap();
		let (shape, skeleton) = parse::parse::<BigRational, std::fs::File>(file).unwrap();
		rendersvg::draw_svg(shape, skeleton, &filename)
	}
	return problems
}

fn main() {
	use std::env;
	use std::process;
	// setup directories for outputs
	std::fs::create_dir_all(BASEPATH).unwrap();
	if env::args().len() < 2 {
		println!("Cmds: updatecontest, drawproblems, ...");
		process::exit(1);
	}
	let cmd = env::args().nth(1).unwrap();
	println!("Running {:?}", cmd);
	match cmd.trim() {
		"updatecontest" => {
			// remove cached files
			std::fs::remove_file(format!("{}/contest_list.json", BASEPATH)).unwrap();
			std::fs::remove_file(format!("{}/contest.json", BASEPATH)).unwrap();
			// grab contest snapshots
			let problems = restapi::get_contest_meta();
			// save the problem blobs
			restapi::save_problems(problems);
		},
		"drawproblems" => {
			// draw svgs of each one
			let problems = restapi::get_contest_meta();
			let problems = restapi::save_problems(problems);
			draw_problems(problems);
		},
		"drawproblem" => {
			let id = env::args().nth(2).unwrap().parse::<i64>().unwrap();
			let filename = format!("{:03}.problem.svg", id);
			let file = std::fs::File::open(format!("{}/{:03}.problem.txt", BASEPATH, id)).unwrap();
			let (shape, skeleton) = parse::parse::<BigRational, std::fs::File>(file).unwrap();
			rendersvg::draw_svg(shape, skeleton, &filename)
		},
		_ => {
			println!("Cmds: updatecontest, drawproblem, ...");
			process::exit(1);
		}
	}
}
