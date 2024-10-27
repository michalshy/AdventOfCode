use std::io::Error;
use not_quite_lisp::NotQuiteLisp;
use no_math::NoMath;
use perfectly_spherical_houses::PerfectHouses;
use stocking_stuffer::StockStuff;

mod not_quite_lisp;
mod no_math;
mod perfectly_spherical_houses;
mod stocking_stuffer;

#[derive(Default)]
pub struct Puzzles;

impl Puzzles {
    pub fn solve(&mut self) {
        let result = self.puzzles();
        result.unwrap();
    } 

    pub fn puzzles(&mut self) -> Result<(), Error> {
        //first puzzle solved - 1 DAY
        {
            let res1 = NotQuiteLisp::default().solve_first();
            match res1 {
                Ok(floor) => {
                    println!("number of santas floor is {floor}");
                },
                _ => (),
            }
            //second puzzle solved - 1st DAY
            let res2 = NotQuiteLisp::default().solve_second();
            match res2 {
                Ok(floor) => {
                    println!("number of floor when he enters basement is {floor}");
                },
                _ => (),
            }
        }
        {
            //both puzzle - 2nd DAY
            let res = NoMath::default().solve();
            match res {
                Ok(space) => {
                    println!("space needed for gifts is {}", space.size);
                    println!("length needed for gifts is {}", space.length);
                },
                _ => (),
            }
        }
        {
            //puzzles - 3rd DAY
            let res = PerfectHouses::default().solve_puzzle();
            match res {
                Ok(houses) => {
                    println!("number of visited houses first year is {}", houses.0);
                    println!("number of visited houses second year is {}", houses.1);
                },
                _ => (),
            }
        }
        {
            //puzzles - 4th DAY
            let res = StockStuff::default().mine();
            match res {
                Ok(mine) => {
                    if mine.1 == false {
                        println!("lowest number to create md5 hash with 5 zeros is {}", mine.0);
                    }
                    else {
                        println!("lowest number to create md5 hash with 5 was not found");
                    }
                },
                _ => (),
            }
        }
        Ok(())
    }
}