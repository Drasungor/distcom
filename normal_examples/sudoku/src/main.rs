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
                    println!("i: {}, j: {}", i, j);
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

fn process_result(solved_sudoku: &Vec<Vec<SudokuCell>>) {
    for i in 0..solved_sudoku.len() {
        for j in 0..solved_sudoku.len() {
            let current_value = match solved_sudoku[i][j] {
                SudokuCell::AssignedValue(assigned_value) => assigned_value,
                SudokuCell::FixedValue(fixed_value) => fixed_value,
            };
            print!("{current_value}");
        }
        println!("");
    }
}

fn read_sudoku() -> Vec<Vec<u8>> {
    let v: Vec<Vec<u8>> = vec![
        vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
        vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
        vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
        vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
        vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
        vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
        vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
        vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
        vec![0, 0, 0, 0, 8, 0, 0, 7, 9]
    ];
    v
}

fn main() {
    let sudoku_to_solve = read_sudoku();
    let solved_sudoku = solve_sudoku(sudoku_to_solve);
    process_result(&solved_sudoku);
}
