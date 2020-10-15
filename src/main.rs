use std::env;
use std::io;
use std::process::exit;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file_to_hash_set(filepath: &str) -> HashSet<String> {
    let mut h = HashSet::new();
    if let Ok(lines) = read_lines(filepath) {
        for line_result in lines {
            if let Ok(line) = line_result {
                h.insert(line.to_string());
            }
        }
    }
    h
}

fn print_intersection(filepath_a: &str, filepath_b: &str) {
    let hash_a = read_file_to_hash_set(filepath_a);
    let hash_b = read_file_to_hash_set(filepath_b);

    for line_a in hash_a.intersection(&hash_b) {
        println!("{}", line_a);
    }
}

fn help() {
    println!("usage: intersect <file a> <file b>");
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let filepath_a = &args[1];
            let filepath_b = &args[2];
            print_intersection(filepath_a, filepath_b);
        }
        _ => help()
    }
}
