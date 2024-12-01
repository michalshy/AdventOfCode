use input::INPUT;
use std::collections::BTreeMap;

mod input;

pub fn decrement_first(mut side: BTreeMap<i32, i32>, key: &i32) {
    side.get_mut(key);
}

fn main() {
    let mut total: i64 = 0;
    let mut left_side: BTreeMap<i64, i64> = BTreeMap::new();
    let mut right_side: BTreeMap<i64, i64> = BTreeMap::new();

    for line in INPUT.split('\n') {
        let mut el = line.split("   ");
        let left = el.next().unwrap().parse::<i64>().unwrap();
        let right = el.next().unwrap().parse::<i64>().unwrap();
        match left_side.get_mut(&left) {
            Some(val) => *val = *val + 1,
            None => {
                left_side.insert(left, 1);
            },
        }
        match right_side.get_mut(&right) {
            Some(val) => *val = *val + 1,
            None => {
                right_side.insert(right, 1);
            },
        }
    }

    for (l_key, mut l_val) in left_side {
        while l_val != 0 {
            if let right = right_side.clone().first_key_value() {
                match right {
                    Some((r_key, r_val)) => {
                        println!("{} - {}", r_key, l_key);
                        total += i64::abs(l_key - r_key);
                        *right_side.get_mut(r_key).unwrap() -= 1;
                        match right_side.get(r_key).unwrap() {
                            0 => {
                                right_side.pop_first(); 
                            },
                            _ => (),
                        }
                    }
                    None => {
                        println!("No values!");
                    }
                }
            }
            l_val -= 1;
        }
    }

    print!("{}", total);
}
