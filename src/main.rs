use std::fs::read_to_string;
use structopt::StructOpt;
use rayon::prelude::*;

#[derive(StructOpt)]
#[structopt(name = "rsgrep")]
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: Vec<String>,
}

fn grep(state: &GrepArgs, content: String, file_name: &str) {
    let mut line_no = 0;
    for line in content.lines() {
        line_no = line_no + 1;
        if line.contains(state.pattern.as_str()) {
            println!("{}({}): {}", file_name, line_no, line);
        }
    }
}

fn run(state: GrepArgs) {
    state
        .path
        .par_iter()
        .for_each(|file| match read_to_string(file) {
            Ok(content) => grep(&state, content, &file),
            Err(reason) => println!("{}", reason),
        })
}

fn main() {
    run(GrepArgs::from_args())
}
