extern crate svg;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::fs;
use std::io::Read;
use std::process::Command;
use std::path::Path;
use std::vec::Vec;
use std::time::Duration;
use std::thread;

const BASEPATH: &'static str = "icfp2016problems";

fn save_problem(probnum: i32) {
    println!("{:?}", Command::new("sh")
                        .arg("-c")
                        .arg("echo hello")
                        .output()
                        .expect("failed to execute proces"));
}

fn create_svg(probnum: i32) {
    use svg::Document;
    use svg::node::element::Path;
    use svg::node::element::path::Data;
    let data = Data::new()
                .move_to((10, 10))
                .line_by((0, 50))
                .line_by((50, 0))
                .line_by((0, -50))
                .close();

    let path = Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 3)
                    .set("d", data);

    let document = Document::new()
                            .set("viewBox", (0, 0, 70, 70))
                            .add(path);

    svg::save(format!("{}/{}.svg", BASEPATH, probnum), &document).unwrap();
}

fn download(filename: &str, apipathname: &str) {
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
        let filename = format!("{}.problem.txt", id);
        download(&filename, &blob);
        thread::sleep(Duration::from_millis(1000));
        println!("{:?}", problem);
    }
    return problems
}

fn main() {
    // setup directories for outputs
    fs::create_dir_all(BASEPATH);
    // grab contest snapshots
    let problems = get_contest_meta();
    let problems = save_problems(problems);
    save_problems(problems);

    // save a problem (should be a loop)
    save_problem(1);
    create_svg(1);
}
