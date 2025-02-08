enum SudokuCell {
    FixedValue(u8),
    AssignedValue(u8)
}

fn validate_sudoku(sudoku_to_solve: &Vec<Vec<u8>>) {
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

fn solve_sudoku(sudoku_to_solve: Vec<Vec<u8>>) {
    validate_sudoku(&sudoku_to_solve);
}


fn main() {
    println!("Hello, world!");




}
