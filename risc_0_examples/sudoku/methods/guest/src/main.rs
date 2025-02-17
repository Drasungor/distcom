#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    // pub starting_sudoku: Vec<Vec<u8>>,
    pub solved_sudoku: Vec<Vec<u8>>,
}

use std::collections::HashSet;

enum SudokuCell {
    FixedValue(u8),
    AssignedValue(u8)
}

fn subsection_index_to_coordinates(subsection_size: usize, subsection_index: usize) -> (usize, usize) {
    return (subsection_index / subsection_size, subsection_index % subsection_size)
}

fn validate_initial_sudoku(sudoku_to_solve: &Vec<Vec<u8>>) -> usize {
    let lines_amount = sudoku_to_solve.len();
    assert!(lines_amount != 0, "The sudoku must have at least one line");
    let line_length_sqrt = (lines_amount as f64).sqrt().round() as usize;
    assert!(line_length_sqrt * line_length_sqrt  == lines_amount, "The line length must have an integer square root");
    for line in sudoku_to_solve.iter() {
        assert!(line.len() == lines_amount, "All lines must have the same length, equal to the amount of lines")
    }
    let mut zeroes_counter: u32 = 0;
    for line in sudoku_to_solve.iter() {
        for number in line {
            assert!((*number) as usize <= lines_amount, "No value can be bigger than the line length");
            if *number == 0 {
                zeroes_counter += 1;
            }
        }
    }
    assert!(zeroes_counter > 0, "At least one value should be assignable");
    for i in 0..lines_amount {
        assert!(check_line_validity(sudoku_to_solve, i));
        // println!("i: {}", i);
        assert!(check_column_validity(sudoku_to_solve, i));
        let (line, column) = subsection_index_to_coordinates(line_length_sqrt, i);
        assert!(check_subsection_validity(sudoku_to_solve, line_length_sqrt, line, column));
    }
    return line_length_sqrt;
}

fn build_sudoku_with_cells(raw_sudoku: Vec<Vec<u8>>) -> Vec<Vec<SudokuCell>> {
    let mut used_sudoku: Vec<Vec<SudokuCell>> = Vec::new();
    let mut i: usize = 0;
    for line in raw_sudoku {
        used_sudoku.push(Vec::new());
        for number in line {
            if number == 0 {
                used_sudoku[i].push(SudokuCell::AssignedValue(0));
            } else {
                used_sudoku[i].push(SudokuCell::FixedValue(number));
            }
        }
        i += 1;
    }
    return used_sudoku;
}

fn check_line_validity(sudoku_to_solve: &Vec<Vec<u8>>, line: usize) -> bool {
    let mut visited_elements = HashSet::new();
    for element in &sudoku_to_solve[line] {
        let current_value = *element;
        if current_value != 0 {
            if visited_elements.contains(&current_value) {
                return false;
            } else {
                visited_elements.insert(current_value);
            }
        }
    }
    return true;
}

fn check_column_validity(sudoku_to_solve: &Vec<Vec<u8>>, column: usize) -> bool {
    let mut visited_elements = HashSet::new();
    for i in 0..sudoku_to_solve.len() {
        let current_value = sudoku_to_solve[i][column];
        if current_value != 0 {
            if visited_elements.contains(&current_value) {
                return false;
            } else {
                visited_elements.insert(current_value);
            }
        }
    }
    return true;
}

fn check_subsection_validity(sudoku_to_solve: &Vec<Vec<u8>>, subsection_size: usize, line: usize, column: usize) -> bool{
    let mut visited_elements = HashSet::new();
    let initial_column = column - column % subsection_size;
    let initial_line = line - line % subsection_size;
    for i in initial_line..initial_line+subsection_size {
        for j in initial_column..initial_column+subsection_size {
            let current_value = sudoku_to_solve[i][j];
            if current_value != 0 {
                if visited_elements.contains(&current_value) {
                    return false;
                } else {
                    visited_elements.insert(current_value);
                }
            }
        }
    }
    return true;
}

fn check_line_validity_for_value(sudoku_to_solve: &Vec<Vec<SudokuCell>>, checked_value: u8, line: usize) -> bool {
    let mut found_value = false;
    for current_cell in &sudoku_to_solve[line] {
        let current_value = match *current_cell {
            SudokuCell::AssignedValue(value) => value,
            SudokuCell::FixedValue(value) => value,
        };
        if current_value == checked_value {
            if found_value {
                return false
            } else {
                found_value = true;
            }
        }
    }
    return true;
}

fn check_column_validity_for_value(sudoku_to_solve: &Vec<Vec<SudokuCell>>, checked_value: u8, column: usize) -> bool {
    let mut found_value = false;
    for i in 0..sudoku_to_solve.len() {
        let current_cell = &sudoku_to_solve[i][column];
        let current_value = match *current_cell {
            SudokuCell::AssignedValue(value) => value,
            SudokuCell::FixedValue(value) => value,
        };
        if current_value == checked_value {
            if found_value {
                return false
            } else {
                found_value = true;
            }
        }
    }
    return true;
}

fn check_subsection_validity_for_value(sudoku_to_solve: &Vec<Vec<SudokuCell>>, subsection_size: usize, checked_value: u8, line: usize, column: usize) -> bool{
    let mut found_value = false;
    let initial_column = column - column % subsection_size;
    let initial_line = line - line % subsection_size;
    for i in initial_line..initial_line+subsection_size {
        for j in initial_column..initial_column+subsection_size {
            let current_cell = &sudoku_to_solve[i][j];
            let current_value = match *current_cell {
                SudokuCell::AssignedValue(value) => value,
                SudokuCell::FixedValue(value) => value,
            };
            if current_value == checked_value {
                if found_value {
                    return false
                } else {
                    found_value = true;
                }
            }
        }
    }
    return true;
}

fn check_modified_cell_validity(sudoku_to_solve: &Vec<Vec<SudokuCell>>, subsection_size: usize, line: usize, column: usize) -> bool {
    let value = match sudoku_to_solve[line][column] {
        SudokuCell::AssignedValue(aux_value) => aux_value,
        SudokuCell::FixedValue(aux_value) => aux_value,
    };
    return check_line_validity_for_value(sudoku_to_solve, value, line) &&
        check_column_validity_for_value(sudoku_to_solve, value, column) &&
        check_subsection_validity_for_value(sudoku_to_solve, subsection_size, value, line, column);
}

fn get_next_assignable_value(sudoku_to_solve: &mut Vec<Vec<SudokuCell>>, starting_line: usize, starting_column: usize) -> (usize, usize) {
    let mut next_cell_column = starting_column + 1;
    let mut next_cell_line = starting_line;
    if next_cell_column == sudoku_to_solve.len() {
        next_cell_column = 0;
        next_cell_line += 1;
    }
    let mut next_cell_to_assign: (usize, usize) = (usize::MAX, usize::MAX);
    for i in next_cell_line..sudoku_to_solve.len() {
        for j in 0..sudoku_to_solve.len() {
            if i != next_cell_line || j >= next_cell_column {
                if let SudokuCell::AssignedValue(_) = sudoku_to_solve[i][j] {
                    next_cell_to_assign = (i, j);
                    return next_cell_to_assign;
                }
            }
        }
    }
    // if we reach this point then next_cell_to_assign = (usize::MAX, usize::MAX)
    next_cell_to_assign
}

fn wrapper_execute_solving(sudoku_to_solve: &mut Vec<Vec<SudokuCell>>, subsection_size: usize, starting_line: usize, starting_column: usize) -> bool {
    if let SudokuCell::FixedValue(_) = sudoku_to_solve[starting_line][starting_column] {
        panic!("Tried to set a fixed cell");
    }
    let next_cell_to_assign = get_next_assignable_value(sudoku_to_solve, starting_line, starting_column);
    for current_value in 1..sudoku_to_solve.len() + 1 {
        let cast_current_value = current_value as u8;
        sudoku_to_solve[starting_line][starting_column] = SudokuCell::AssignedValue(cast_current_value);
        let is_cell_valid = check_line_validity_for_value(sudoku_to_solve, cast_current_value, starting_line) &&
            check_column_validity_for_value(sudoku_to_solve, cast_current_value, starting_column) &&
            check_subsection_validity_for_value(sudoku_to_solve, subsection_size, cast_current_value, starting_line, starting_column);
        if is_cell_valid {
            if let (usize::MAX, usize::MAX) = next_cell_to_assign {
                return true;
            } else if wrapper_execute_solving(sudoku_to_solve, subsection_size, next_cell_to_assign.0, next_cell_to_assign.1) {
                return true;
            }
        }
        // println!("Set value: {}", current_value);
    }
    sudoku_to_solve[starting_line][starting_column] = SudokuCell::AssignedValue(0);
    return false;
}

fn execute_solving(sudoku_to_solve: &mut Vec<Vec<SudokuCell>>, subsection_size: usize) -> bool {
    for i in 0..sudoku_to_solve.len() {
        for j in 0..sudoku_to_solve.len() {
            if let SudokuCell::AssignedValue(_) = sudoku_to_solve[i][j] {
                return wrapper_execute_solving(sudoku_to_solve, subsection_size, i, j)
            }
        }
    }
    panic!("No value to set was found");
}

fn solve_sudoku(sudoku_to_solve: Vec<Vec<u8>>) -> Vec<Vec<SudokuCell>> {
    let subsection_size = validate_initial_sudoku(&sudoku_to_solve);
    let mut built_sudoku = build_sudoku_with_cells(sudoku_to_solve);
    assert!(execute_solving(&mut built_sudoku, subsection_size), "No solution was found");
    return built_sudoku;
}

fn process_result(solved_sudoku: &Vec<Vec<SudokuCell>>) -> Vec<Vec<u8>> {
    let mut v: Vec<Vec<u8>> = Vec::new();
    for i in 0..solved_sudoku.len() {
        v.push(Vec::new());
        for j in 0..solved_sudoku.len() {
            let current_value = match solved_sudoku[i][j] {
                SudokuCell::AssignedValue(assigned_value) => assigned_value,
                SudokuCell::FixedValue(fixed_value) => fixed_value,
            };
            // if current_value < 10 {
            //     print!("  {current_value}");
            // } else {
            //     print!(" {current_value}");
            // }
            v[i].push(current_value);
        }
        println!("");
    }
    return v;
}

fn read_sudoku() -> Vec<Vec<u8>> {
    // let v: Vec<Vec<u8>> = vec![
    //     vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
    //     vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
    //     vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
    //     vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
    //     vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
    //     vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
    //     vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
    //     vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
    //     vec![0, 0, 0, 0, 8, 0, 0, 7, 9]
    // ];

    // let v: Vec<Vec<u8>> = vec![
    //     vec![0, 0, 7, 4, 9, 1, 6, 0, 5],
    //     vec![2, 0, 0, 0, 6, 0, 3, 0, 9],
    //     vec![0, 0, 0, 0, 0, 7, 0, 1, 0],
    //     vec![0, 5, 8, 6, 0, 0, 0, 0, 4],
    //     vec![0, 0, 3, 0, 0, 0, 0, 9, 0],
    //     vec![0, 0, 6, 2, 0, 0, 1, 8, 7],
    //     vec![9, 0, 4, 0, 7, 0, 0, 0, 2],
    //     vec![6, 7, 0, 8, 3, 0, 0, 0, 0],
    //     vec![8, 1, 0, 0, 4, 5, 0, 0, 0]
    // ];

    // let v: Vec<Vec<u8>> = vec![
    //     vec![ 0,  0,  0,  3,  0,  0, 16,  1,  2, 15,  0,  0, 10,  0,  0,  0],
    //     vec![ 0,  2,  0,  0, 12,  0,  3,  0,  0, 14,  0, 13,  0,  0, 15,  0],
    //     vec![ 0,  5,  0,  0,  0,  6, 11,  0,  0,  9,  1,  0,  0,  0,  2,  0],
    //     vec![ 0,  8,  0, 16,  0,  0, 15,  0,  0,  4,  0,  0,  3,  0,  1,  0],

    //     vec![ 0,  0,  0, 12,  0, 11,  0, 13,  8,  0,  5,  0,  1,  0,  0,  0],
    //     vec![ 0,  1,  0, 11,  0,  0,  0,  5, 13,  0,  0,  0,  7,  0,  9,  0],
    //     vec![ 0, 10,  0,  0,  0, 15,  0,  7,  3,  0, 14,  0,  0,  0, 16,  0],
    //     vec![ 0, 15, 16,  2,  0,  0,  0,  4, 10,  0,  0,  0, 11,  6, 14,  0],

    //     vec![13,  0,  2,  0,  4, 12,  0,  0,  0,  0, 16,  1,  0,  8,  0,  3],
    //     vec![16,  0, 15,  0,  3,  0,  0,  0,  0,  0,  0,  6,  0, 10,  0,  4],
    //     vec![11,  0,  6,  0, 16,  0,  0,  0,  0,  0,  0,  5,  0,  1,  0,  2],
    //     vec![12,  0,  0,  0, 10,  8,  0,  0,  0,  0,  3,  2,  0,  0,  0, 11],

    //     vec![15,  0, 11, 14,  0,  0,  4,  0,  0,  8,  0,  0,  6,  5,  0, 12],
    //     vec![ 0,  0,  0,  0, 15,  0,  0,  3, 12,  0,  0, 11,  0,  0,  0,  0],
    //     vec![ 6,  0, 10,  5,  0,  0,  0,  0,  0,  0,  0,  0, 15,  9,  0,  1],
    //     vec![ 0,  0,  0,  0,  5,  0, 13, 10,  1, 16,  0,  4,  0,  0,  0,  0],
    // ];

    // let v: Vec<Vec<u8>> = vec![
    //     vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
    //     vec![0, 0, 9, 2, 0, 0, 0, 0, 3],
    //     vec![5, 7, 0, 0, 8, 1, 0, 0, 0],
    //     vec![0, 0, 0, 0, 0, 0, 0, 6, 0],
    //     vec![0, 0, 0, 0, 9, 6, 8, 0, 4],
    //     vec![0, 4, 0, 7, 5, 0, 0, 2, 0],
    //     vec![6, 5, 0, 0, 0, 0, 0, 9, 0],
    //     vec![9, 0, 0, 0, 0, 3, 2, 0, 0],
    //     vec![0, 2, 0, 9, 0, 0, 0, 0, 0]
    // ];

    // let v: Vec<Vec<u8>> = vec![
    //     vec![13, 15,  6, 10,  2,  3, 14,  4,  1,  8,  5,  9, 16, 12,  7, 11],
    //     vec![ 3,  2,  1,  7, 12,  5, 10, 16,  6, 13, 11, 14,  8,  9, 15,  4],
    //     vec![ 8,  5, 14,  4, 11,  1,  9, 13, 16, 15,  7, 12,  3, 10,  2,  6],
    //     vec![ 9, 12, 16, 11,  8,  7,  6, 15,  4,  2,  3, 10, 13,  1, 14,  5],
        
    //     vec![14,  9, 12, 13,  5, 16, 15,  3, 11,  1,  6,  2,  4,  7,  8, 10],
    //     vec![ 4,  1,  5, 15, 14,  6, 11,  2, 10,  7,  8, 16,  9,  3, 12, 13],
    //     vec![ 2, 16,  7,  6, 13,  4,  8, 10,  9, 12, 14,  3,  5, 11,  1, 15],
    //     vec![11, 10,  8,  3,  1, 12,  7,  9, 15,  5, 13,  4,  6,  2, 16, 14],

    //     vec![16, 14,  2,  8, 15,  9,  1,  6, 13,  3,  4, 11, 12,  5, 10,  7],
    //     vec![15,  6,  3, 12,  4,  8, 13,  5,  7, 14, 10,  1, 11, 16,  9,  2],
    //     vec![10,  4,  9,  1,  7, 14,  2, 11,  8, 16, 12,  5, 15, 13,  6,  3],
    //     vec![ 7, 11, 13,  5, 16, 10,  3, 12,  2,  9, 15,  6,  1, 14,  4,  8],
        
    //     vec![ 5,  7, 15,  9,  3,  2, 12,  8, 14,  6,  1, 13, 10,  4, 11, 16],
    //     vec![12,  8, 11,  2, 10, 13, 16, 14,  3,  4,  9, 15,  7,  6,  5,  1],
    //     vec![ 6,  3, 10, 16,  9, 15,  4,  1,  5, 11,  2,  7, 14,  8, 13, 12],
    //     vec![ 1, 13,  4, 14,  6, 11,  5,  7, 12, 10, 16,  8,  2, 15,  3,  9],
    // ];

    let v: Vec<Vec<u8>> = vec![
        vec![ 0, 15,  6,  0,  2,  3, 14,  4,  0,  8,  5,  9,  0,  0,  7, 11],
        vec![ 3,  0,  1,  7,  0,  0, 10,  0,  6,  0,  0, 14,  8,  9, 15,  0],
        vec![ 0,  5, 14,  4,  0,  1,  9, 13, 16, 15,  0,  0,  0, 10,  0,  6],
        vec![ 9,  0,  0,  0,  8,  7,  0, 15,  0,  2,  0, 10,  0,  1,  0,  5],
        
        vec![ 0,  9,  0, 13,  5,  0, 15,  3,  0,  0,  0,  2,  0,  7,  0, 10],
        vec![ 0,  0,  5,  0,  0,  0,  0,  2, 10,  0,  8,  0,  0,  3, 12,  0],
        vec![ 2,  0,  7,  6,  0,  4,  0,  0,  9,  0, 14,  3,  5,  0,  0, 15],
        vec![ 0,  0,  0,  0,  1, 12,  0,  0,  0,  0,  0,  4,  6,  2,  0,  0],

        vec![ 0, 14,  2,  0,  0,  0,  1,  6,  0,  0,  4,  0,  0,  5, 10,  0],
        vec![ 0,  6,  3,  0,  4,  8,  0,  0,  0,  0, 10,  1,  0,  0,  9,  0],
        vec![10,  0,  9,  0,  0, 14,  0, 11,  0,  0,  0,  5,  0, 13,  6,  0],
        vec![ 0,  0,  0,  0, 16, 10,  0, 12,  2,  9,  0,  0,  0,  0,  4,  8],
        
        vec![ 5,  7, 15,  9,  0,  0, 12,  8, 14,  6,  1,  0,  0,  4, 11,  0],
        vec![ 0,  8,  0,  2, 10, 13,  0,  0,  0,  0,  0, 15,  7,  6,  0,  1],
        vec![ 6,  0, 10,  0,  9,  0,  0,  1,  0, 11,  2,  7,  0,  8,  0,  0],
        vec![ 0,  0,  4, 14,  0,  0,  0,  7, 12,  0, 16,  8,  2,  0,  3,  0],
    ];

    v
}

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let input: Vec<u8> = env::read();

    let sudoku_to_solve = read_sudoku();
    let solved_sudoku = solve_sudoku(sudoku_to_solve);
    let processed_sudoku = process_result(&solved_sudoku);

    let outputs: Outputs = Outputs {
        // starting_sudoku: sudoku_to_solve,
        solved_sudoku: processed_sudoku,
    }; 
    let serialized_outputs = to_string(&outputs).expect("Error in struct serialization");
    env::commit(&serialized_outputs);
}