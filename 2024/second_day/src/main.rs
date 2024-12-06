use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Modes {
    Unknown,
    Descending,
    Ascending
}

fn main() {
    let mut total_valid = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.flatten() {
            let mut prv: i32 = 0;
            let mut prv_prv: i32 = 0;
            let mut mode: Modes = Modes::Unknown;
            let mut prv_mode = 0;
            let mut skipped = 0;
            let mut valid = true;
            let mut single_tolerant = true;
            let mut virtual_delete = false;
            let mut virtually_deleted = -100;
            for el in line.split(" ") {
                println!("el {}, prv {}, skipped {}, single {}, prv_m {}, prv_prv {}", el, prv, skipped, single_tolerant, prv_mode, prv_prv);
                if skipped > 0 {
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
                                prv_mode = 1;
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
                                prv_mode = 2;
                                virtual_delete = true;
                            }
                        },
                        Modes::Unknown => {
                            if el.parse::<i32>().unwrap() < prv 
                            && prv - el.parse::<i32>().unwrap() < 4 
                            && prv - el.parse::<i32>().unwrap() > 0 
                            && (prv_mode == 2 || prv_mode == 0) { 
                                mode = Modes::Descending 
                            } else if el.parse::<i32>().unwrap() > prv
                            &&  el.parse::<i32>().unwrap() - prv < 4 
                            && el.parse::<i32>().unwrap() - prv > 0 
                            && (prv_mode == 1 || prv_mode == 0) { 
                                mode = Modes::Ascending; 
                            } else if !single_tolerant { 
                                valid = false;
                                break;
                            } else { 
                                match mode {
                                    Modes::Ascending => {
                                        //giving up...
                                    },
                                    Modes::Descending => {

                                    },
                                    Modes::Unknown => (),
                                }
                                single_tolerant = false;
                            }
                        },
                    }
                }
                skipped += 1;
                if !virtual_delete {
                    if skipped > 1 {
                        prv_prv = prv;
                    }
                    prv = el.parse::<i32>().unwrap();
                } else {
                    virtually_deleted = el.parse::<i32>().unwrap();
                }
                virtual_delete = false;
            }
            if valid {
                if !single_tolerant {
                    println!("Line is {} and result {}", line, valid);
                }
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