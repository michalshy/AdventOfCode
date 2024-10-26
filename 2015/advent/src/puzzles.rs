use std::io::Error;
use not_quite_lisp::NotQuiteLisp;

mod not_quite_lisp;

#[derive(Default)]
pub struct Puzzles;

impl Puzzles {
    pub fn solve(&mut self) {
        let result = self.puzzles();
        result.unwrap();
    } 

    pub fn puzzles(&mut self) -> Result<(), Error> {
        //first puzzle solved
        let res1 = NotQuiteLisp::default().solve_first();
        match res1 {
            Ok(floor) => {
                println!("number of santas floor is {floor}");
            },
            _ => (),
        }
        //second puzzle solved
        let res2 = NotQuiteLisp::default().solve_second();
        match res2 {
            Ok(floor) => {
                println!("number of floor when he enters basement is {floor}");
            },
            _ => (),
        }
        Ok(())
    }
}