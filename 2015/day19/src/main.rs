use std::{collections::{HashMap, HashSet}, fs};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn main() {
    let mut transitions: HashMap<String, String> = HashMap::new();
    let result: HashSet<String> = HashSet::new();

    let molecule = fs::read_to_string("./molecule.txt")
        .expect("File is right there!");

    if let Ok(lines) = read_lines("./transitions.txt") {
        for line in lines.flatten() {
            let vals = line.split_once(" => ");
            match vals {
                Some(vals) => {
                    transitions.insert(vals.0.into(), vals.1.into());
                },
                _ => (),
            }
        }
    }

    for (k,v) in transitions {
        println!("key: {}, val: {}", k, v);
    }
}
