enum SudokuCell {
    FixedValue(u8),
    AssignedValue(u8)
}

fn validate_initial_sudoku(sudoku_to_solve: &Vec<Vec<u8>>) {
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

fn solve_sudoku(sudoku_to_solve: Vec<Vec<u8>>) {
    validate_initial_sudoku(&sudoku_to_solve);
    let built_sudoku = build_sudoku_with_cells(sudoku_to_solve);

}


fn main() {
    println!("Hello, world!");




}
