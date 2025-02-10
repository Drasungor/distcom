use std::collections::HashSet;

enum SudokuCell {
    FixedValue(u8),
    AssignedValue(u8)
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

fn check_line_validity(sudoku_to_solve: &Vec<Vec<SudokuCell>>, line: usize) -> bool {
    let mut visited_elements = HashSet::new();
    for element in &sudoku_to_solve[line] {
        let current_value = match *element {
            SudokuCell::AssignedValue(assigned_value) => assigned_value,
            SudokuCell::FixedValue(fixed_value) => fixed_value,
        };
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

fn check_column_validity(sudoku_to_solve: &Vec<Vec<SudokuCell>>, column: usize) -> bool {
    let mut visited_elements = HashSet::new();
    for i in 0..sudoku_to_solve.len() {
        let current_value = match sudoku_to_solve[i][column] {
            SudokuCell::AssignedValue(assigned_value) => assigned_value,
            SudokuCell::FixedValue(fixed_value) => fixed_value,
        };
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

fn check_subsection_validity(sudoku_to_solve: &Vec<Vec<SudokuCell>>, subsection_size: usize, line: usize, column: usize) -> bool{
    let mut visited_elements = HashSet::new();
    let initial_column = column - column % subsection_size;
    let initial_line = line - line % subsection_size;
    for i in initial_line..initial_line+subsection_size {
        for j in initial_column..initial_column+subsection_size {
            let current_value = match sudoku_to_solve[i][j] {
                SudokuCell::AssignedValue(assigned_value) => assigned_value,
                SudokuCell::FixedValue(fixed_value) => fixed_value,
            };
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
