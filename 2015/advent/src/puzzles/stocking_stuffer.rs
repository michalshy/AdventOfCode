use std::io::Error;
use md5::{self, Digest};

const INPUT: &str = "iwrupvqb";

#[derive(Default)]
pub struct StockStuff;

impl StockStuff {
    pub fn mine(&mut self) -> Result<(i32, bool), Error> {
        let mut overflow = false;
        let mut index: i32 = -1;
        let mut digest: Digest;
        let mut res: String = "".to_owned();
        //while  !res.starts_with("00000") { - FIRST PUZZLE
        //Second puzzle
        while  !res.starts_with("000000") {
            index += 1;
            if overflow {
                break;
            }
            let to_compute: String = INPUT.to_owned() + &index.to_string();
            digest = md5::compute(to_compute.as_str());
            res = format!("{:x}", digest);

            if index == i32::MAX
            {
                overflow = true;
            }
        }
        Ok((index, overflow))
    }
}