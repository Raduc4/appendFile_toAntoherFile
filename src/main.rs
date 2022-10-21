use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead};

fn append(line: String, file_to_append: &str) -> () {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_to_append)
        .unwrap();

    if let Err(e) = writeln!(
        file,
        "F  {}                                                                               $",
        line
    ) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn main() {
    let file = File::open("src/data/bar.txt").unwrap();
    let lines = io::BufReader::new(file);
    for line in lines.lines() {
        match line {
            Ok(l) => append(l, "src/data/foo.txt"),
            Err(e) => eprintln!("Couldn't write to file: {}", e),
        }
    }
}
