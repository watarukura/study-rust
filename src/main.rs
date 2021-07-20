use std::env::args;
use std::fs::read_to_string;

fn grep(pattern: String, content: String) {
    for line in content.lines() {
        if line.contains(pattern.as_str()) {
            println!("{}", line);
        }
    }
}

struct GrepArgs {
    pattern: String,
    path: String,
}

impl GrepArgs {
    fn new(path: String, pattern: String) -> GrepArgs {
        GrepArgs { path, pattern }
    }
}

fn run(state: GrepArgs) {
    match read_to_string(state.path) {
        Ok(content) => grep(state.pattern, content),
        Err(reason) => println!("{}", reason),
    }
}

fn main() {
    let pattern = args().nth(1);
    let path = args().nth(2);
    match (pattern, path) {
        (Some(pattern), Some(path)) => run(GrepArgs::new(path, pattern)),
        _ => println!("pattern or path is not specified!"),
    }
}
