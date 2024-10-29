use std::io::{Error,Read};
use std::fs::File;

#[derive(Default)]
pub struct InternElves {
    data_content: String,
}

impl InternElves {
    pub fn solve_puzzles(&mut self) -> Result<i32, Error> {
        let mut nice_strings: i32 = 0;
        self.read_file()?;
        for line in self.data_content.lines() {
            nice_strings += self.check_if_nice(line) as i32;
        }
        Ok(nice_strings)
    }    
    fn read_file(&mut self) -> Result<(), Error> {
        let mut data_file = File::open("files/strings.txt")?;
        data_file.read_to_string(&mut self.data_content)?;
        Ok(())
    }
    fn check_if_nice(&self, line: &str) -> i8 {
        if self.first_req(line) && self.second_req(line) && self.third_req(line) {
            1
        }
        else {
            0
        }
    }
    fn first_req(&self, line: &str) -> bool {
        let mut occurances = 0;
        let vowels = "aeiou";
        for c in vowels.chars() {
            occurances += line.chars().filter(|x| *x == c).count();
        }
        if occurances >= 3 {
            return true;
        }
        else {
            return false;
        }
        
    }
    fn second_req(&self, line: &str) -> bool {
        for c in line.chars() {
            let mut combined: String = c.to_string();
            combined.push(c);
            if line.find(&combined) != None {
                return true;
            }
        }
        return false;
    }
    fn third_req(&self, line: &str) -> bool {
        if line.find("ab") != None || line.find("cd") != None 
            || line.find("pq") != None || line.find("xy") != None {
            return false;
        }
        else {
            return true;
        }
    }
}