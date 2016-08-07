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

fn draw_problems(problems: Vec<Json>, attempts: i64) -> Vec<Json> {
	println!("Attempting to solve {}/{} problems", attempts, problems.len());
	let mut skipped = 0;
	let mut attempted = 0;
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
			attempted += 1;
			if attempted > attempts { break }
		} else if !solution.exists() {
			println!("Problem {} can't be parsed", id);
		} else {
			skipped += 1;
		}
	}
	println!("Skipped {} with existing solutions", skipped);
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
			// draw svgs of each one, attempts is numner to try
			let problems = restapi::get_contest_meta();
			let problems = restapi::save_problems(problems);
			let attempts = env::args().nth(2).unwrap_or("100".to_string()).parse::<i64>().unwrap_or(100);
			draw_problems(problems, attempts);
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
