use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Modes {
    Fail,
    Unknown,
    Descending,
    Ascending
}

fn main() {
    let mut total_valid = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.flatten() {
            let mut prv: i32 = 0;
            let mut mode: Modes = Modes::Unknown;
            let mut skipped_first = false;
            let mut valid = true;
            let mut single_tolerant = true;
            let mut virtual_delete = false;
            println!("Line is {}", line);
            for el in line.split(" ") {
                if skipped_first {
                    match mode {
                        Modes::Ascending => {
                            if el.parse::<i32>().unwrap() > prv
                            &&  el.parse::<i32>().unwrap() - prv < 4 {
                                
                            }
                            else if !single_tolerant{
                                valid = false;
                                break;
                            }
                            else {
                                single_tolerant = false;
                                mode = Modes::Unknown;
                                virtual_delete = true;
                            }
                        },
                        Modes::Descending => {
                            if el.parse::<i32>().unwrap() < prv 
                            && prv - el.parse::<i32>().unwrap() < 4 {
                                
                            }
                            else if !single_tolerant{
                                valid = false;
                                break;
                            }
                            else {
                                single_tolerant = false;
                                mode = Modes::Unknown;
                                virtual_delete = true;
                            }
                        },
                        Modes::Unknown => {
                            mode = if el.parse::<i32>().unwrap() < prv 
                                    && prv - el.parse::<i32>().unwrap() < 4 
                                    { Modes::Descending } 
                                    else if el.parse::<i32>().unwrap() > prv
                                    &&  el.parse::<i32>().unwrap() - prv < 4
                                    { Modes::Ascending }
                                    else { Modes::Fail }
                        },
                        Modes::Fail  => {
                            if !single_tolerant {
                                valid = false;
                                break;
                            }
                            else {
                                single_tolerant = false;
                                virtual_delete = true;
                            }
                        }
                    }
                } else {
                    skipped_first = true;
                }
                if !virtual_delete {
                    prv = el.parse::<i32>().unwrap();
                }
                virtual_delete = false;

            }
            println!("Result was: {}", valid);
            if valid {
                total_valid += 1;
            }
        }
    }

    println!("Total valid: {}", total_valid);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}