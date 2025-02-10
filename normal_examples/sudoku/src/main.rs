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
    for line in sudoku_to_solve.iter() {
        for number in line {
            assert!((*number) as usize <= lines_amount, "No value can be bigger than the line length");
        }
    }
    for i in 0..lines_amount {
        assert!(check_line_validity(sudoku_to_solve, i));
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

fn check_modified_cell_validity(sudoku_to_solve: &Vec<Vec<SudokuCell>>, subsection_size: usize, line: usize, column: usize) {
    
}

fn wrapper_execute_solving(sudoku_to_solve: &Vec<Vec<SudokuCell>>, tried_value: u8, line: usize, column: usize) -> bool {
    match sudoku_to_solve[line][column] {
        SudokuCell::FixedValue(_) => panic!("Tried to set a fixed cell"),
        _ => {},
    }

}

fn execute_solving(sudoku_to_solve: &Vec<Vec<SudokuCell>>, subsection_size: usize) {

}

fn solve_sudoku(sudoku_to_solve: Vec<Vec<u8>>) {
    let subsection_size = validate_initial_sudoku(&sudoku_to_solve);
    let built_sudoku = build_sudoku_with_cells(sudoku_to_solve);
    for line in built_sudoku {
        for value in line {
            
        }
    }
}


fn main() {
    println!("Hello, world!");




}
