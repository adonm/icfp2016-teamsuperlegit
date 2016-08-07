use ::BASEPATH;
use std;
use std::io::Read;
use std::path::Path;
use std::time::Duration;
use std::thread;
use rustc_serialize::json::Json;
use std::process::Command;

pub fn submit(problem_id: i64) {
	// submit a solution
	// easily submit all uncommitted solutions using e.g: 
	// # git -C icfp2016problems status | grep solution.txt | grep -E -o "[0-9]+" | xargs -n1 cargo run submit
	let path = format!("{}/{:05}.solution.txt", BASEPATH, problem_id);
    println!("{}",path);
	let path_arg = path.clone();
	let output = Path::new(&path);
	if output.exists() {
        println!("Uploading solution {}", path);
		println!("{:?}", 
			Command::new("curl").arg("--compressed").arg("-Ss").arg("-L").arg("-H").arg("Expect:").arg("-H")
			.arg(format!("X-API-Key: 60-d7840e0fce3dc9e9a4e2693153ccd9bc")).arg("-F").arg(format!("problem_id={}", problem_id))
			.arg("-F").arg(format!("solution_spec=@{}", path_arg))
			.arg("http://2016sv.icfpcontest.org/api/solution/submit")
			.output()
			.expect("uh-oh")
		);
		thread::sleep(Duration::from_millis(1000));
	}
}

pub fn submit_problem(problem_id: i64) {
	// submit a problem
	let path = format!("{}/custom/{:05}.solution.txt", BASEPATH, problem_id);
    println!("Taking problem from path: {}",path);
	let path_arg = path.clone();
	let output = Path::new(&path);
	if output.exists() {
        println!("Uploading solution {}", path);
		println!("{:?}", 
			Command::new("curl").arg("--compressed").arg("-Ss").arg("-L").arg("-H").arg("Expect:").arg("-H")
			.arg(format!("X-API-Key: 60-d7840e0fce3dc9e9a4e2693153ccd9bc")).arg("-F").arg(format!("problem_id={}", problem_id))
			.arg("-F").arg(format!("solution_spec=@{}", path_arg))
      .arg("-F").arg("publish_time=1470603600")
			.arg("http://2016sv.icfpcontest.org/api/problem/submit")
			.output()
			.expect("uh-oh")
		);
		thread::sleep(Duration::from_millis(1000));
  } else {
    println!("File doesn't exist");
  }
}

fn download(filename: &str, apipathname: &str) {
	// Get latest snapshot
	let path = format!("{}/{}", BASEPATH, filename);
	let path_arg = path.clone();
	let output = Path::new(&path);
	if !output.exists() {
        println!("Saving to {}", path);
		println!("{:?}", 
			Command::new("curl").arg("--compressed").arg("-Ss").arg("-L").arg("-H").arg("Expect:").arg("-H")
			.arg(format!("X-API-Key: 60-d7840e0fce3dc9e9a4e2693153ccd9bc"))
			.arg(format!("http://2016sv.icfpcontest.org/api/{}", &apipathname))
			.arg("-o").arg(path_arg)
			.output()
			.expect("uh-oh")
		);
		thread::sleep(Duration::from_millis(1000));
	}
}

pub fn get_contest_meta() -> Vec<Json> {
	download("contest_list.json", "snapshot/list");
	let mut file = std::fs::File::open(format!("{}/contest_list.json", BASEPATH)).unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();
	let json = Json::from_str(&data).unwrap();
	let contest_blobs = json.find_path(&["snapshots"]).unwrap().as_array().unwrap();
	let latest_snapshot = contest_blobs[contest_blobs.len()-1].find_path(&["snapshot_hash"]).unwrap().as_string().unwrap();
	let snapshot_blob = format!("blob/{}", latest_snapshot);
	// grab latest snapshot
	download("contest.json", &snapshot_blob);
	file = std::fs::File::open(format!("{}/contest.json", BASEPATH)).unwrap();
	data = String::new();
	file.read_to_string(&mut data).unwrap();
	let json = Json::from_str(&data).unwrap();
	return json.find_path(&["problems"]).unwrap().as_array().unwrap().clone();
}

pub fn save_problems(problems: Vec<Json>) -> Vec<Json> {
	for problem in &problems {
		let hash = problem.find_path(&["problem_spec_hash"]).unwrap().as_string().unwrap();
		let id = problem.find_path(&["problem_id"]).unwrap().as_i64().unwrap();
		let blob = format!("blob/{}", hash);
		let filename = format!("{:05}.problem.txt", id);

		download(&filename, &blob);
	}
	return problems
}
