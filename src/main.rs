// Kill rustc dead code warnings because we have ALL THE DEAD CODE
#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate num;
extern crate rustc_serialize;
extern crate svg;
use std::vec::Vec;
use std::path::Path;
use rustc_serialize::json::Json;

mod core;
mod generator;
mod matrix;
mod solver;
mod parse;
mod rendersvg;
mod restapi;
mod write;

use num::rational::BigRational;

pub const BASEPATH: &'static str = "icfp2016problems";

fn draw_problems(problems: Vec<Json>) -> Vec<Json> {
	for problem in &problems {
		let id = problem.find_path(&["problem_id"]).unwrap().as_i64().unwrap();
		let filename = format!("{:05}.problem.svg", id);
		let file = std::fs::File::open(format!("{}/{:05}.problem.txt", BASEPATH, id)).unwrap();
		let solpath = format!("{}/{:05}.solution.txt", BASEPATH, id);
		let solution = Path::new(&solpath);
		let data = parse::parse::<BigRational, std::fs::File>(file);
		if data.is_ok() && !solution.exists() {
			let (shape, skeleton) = data.unwrap();
			rendersvg::draw_svg(shape, skeleton, &filename);
		} else if !solution.exists() {
			println!("Problem {} can't be parsed", id);
		}
	}
	return problems
}

fn main() {
	use std::env;
	use std::process;
	// setup directories for outputs
	std::fs::create_dir_all(BASEPATH).unwrap();
    let help_string = "Cmds: updatecontest, drawproblem, solveproblem, ...";
	if env::args().len() < 2 {
		println!("{:?}", help_string);
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
			let filename = format!("{:05}.problem.svg", id);
			let file = std::fs::File::open(format!("{}/{:05}.problem.txt", BASEPATH, id)).unwrap();
			let (shape, skeleton) = parse::parse::<BigRational, std::fs::File>(file).unwrap();
			rendersvg::draw_svg(shape, skeleton, &filename)
		},
        "solveproblem" => {
            let id = env::args().nth(2).unwrap().parse::<i64>().unwrap();
			let file = std::fs::File::open(format!("{}/{:05}.problem.txt", BASEPATH, id)).unwrap();
			let (shape, skeleton) = parse::parse::<BigRational, std::fs::File>(file).unwrap();
            solver::solve(shape, skeleton)
        },
		"submit" => {
			restapi::submit(env::args().nth(2).unwrap().parse::<i64>().unwrap())
		},
		_ => {
			println!("{:?}", help_string);
			process::exit(1);
		}
	}
}
