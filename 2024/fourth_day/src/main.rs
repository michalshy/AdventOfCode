use std::fs::read_to_string;

const KEY_WORD: &str = "XMAS";

fn main() {
    let mut total_number  = 0;

    let mut input: Vec<Vec<char>> = Vec::new();

    for line in read_to_string("./input.txt").unwrap().lines() {
        let mut line_container: Vec<char> = Vec::new();
        for c in line.chars() {
            line_container.push(c);
        }
        input.push(line_container);
    }

    for (row_index, row) in input.iter().enumerate() {
        for (column_index, el) in row.iter().enumerate() {
            if el == &'X' {
                if column_index >= 3 { //upper search
                    if input[row_index][column_index - 1] == 'M'
                    && input[row_index][column_index - 2] == 'A'
                    && input[row_index][column_index - 3] == 'S' {
                            total_number += 1;
                    }
                    //additional diagonal
                    if row_index >= 3 { //diagonal upper back
                        if input[row_index - 1][column_index - 1] == 'M'
                        && input[row_index - 2][column_index - 2] == 'A'
                        && input[row_index - 3][column_index - 3] == 'S' {
                                total_number += 1;
                        }
                    }
                    if row_index <= input.len() - 4 { //diagonal lower back
                        if input[row_index + 1][column_index - 1] == 'M'
                        && input[row_index + 2][column_index - 2] == 'A'
                        && input[row_index + 3][column_index - 3] == 'S' {
                                total_number += 1;
                        }
                    }
                }
                if column_index <=  input[0].len() - 4 { //front search
                    if input[row_index][column_index + 1] == 'M'
                    && input[row_index][column_index + 2] == 'A'
                    && input[row_index][column_index + 3] == 'S' {
                        total_number += 1;
                    }
                    //additional diagonal
                    if row_index >= 3 { //diagonal upper front
                        if input[row_index - 1][column_index + 1] == 'M'
                        && input[row_index - 2][column_index + 2] == 'A'
                        && input[row_index - 3][column_index + 3] == 'S' {
                                total_number += 1;
                        }
                    }
                    if row_index <= input.len() - 4 { //diagonal lower front
                        if input[row_index + 1][column_index + 1] == 'M'
                        && input[row_index + 2][column_index + 2] == 'A'
                        && input[row_index + 3][column_index + 3] == 'S' {
                                total_number += 1;
                        }
                    }
                }
                if row_index >= 3 { //backward search
                    if input[row_index - 1][column_index] == 'M'
                    && input[row_index - 2][column_index] == 'A'
                    && input[row_index - 3][column_index] == 'S' {
                        total_number += 1;
                    }
                }
                if row_index <= input.len() - 4 { //down search
                    if input[row_index + 1][column_index] == 'M'
                    && input[row_index + 2][column_index] == 'A'
                    && input[row_index + 3][column_index] == 'S' {
                        total_number += 1;
                    }
                }
            }
        }
    }

    println!("Total number is: {}", total_number);
}
