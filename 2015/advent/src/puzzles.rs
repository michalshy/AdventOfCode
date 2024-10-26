use std::io::Error;
use not_quite_lisp::NotQuiteLisp;
use no_math::NoMath;

mod not_quite_lisp;
mod no_math;

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
            //second puzzle solved - 1 DAY
            let res2 = NotQuiteLisp::default().solve_second();
            match res2 {
                Ok(floor) => {
                    println!("number of floor when he enters basement is {floor}");
                },
                _ => (),
            }
        }
        {
            //first pizzle - 2 DAY
            let res1 = NoMath::default().solve_first();
            match res1 {
                Ok(space) => {
                    println!("space needed for gifts is {}", space.size);
                    println!("length needed for gifts is {}", space.length);
                },
                _ => (),
            }
            
            
        }
        Ok(())
    }
}