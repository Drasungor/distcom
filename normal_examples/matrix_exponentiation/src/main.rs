#[derive(Clone)]
pub struct Matrix {
    data: Vec<i64>,
    rows: usize,
    columns: usize,
}

impl Matrix {
    pub fn new(data: Vec<i64>, rows: usize, columns: usize) -> Matrix {
        assert!(data.len() == rows * columns, "Invalid matrix size and values amount");
        return Matrix {
            data,
            rows,
            columns,
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        return (self.rows, self.columns);
    }

    pub fn get_value(&self, row: usize, column: usize) -> i64 {
        assert!(row < self.rows, "Invalid row: {}, max rows: {}", row, self.rows);
        assert!(column < self.columns, "Invalid column: {}, max columns: {}", column, self.columns);
        return self.data[row * self.columns + column];
    }

    pub fn multiply(&self, other_matrix: &Matrix) -> Matrix {
        let mut result_vec = Vec::<i64>::new();
        for i in 0..self.rows {
            // for j in 0..self.columns {
            for j in 0..other_matrix.columns {
                let mut cum_sum = 0;
                for k in 0..self.columns {
                    cum_sum += self.get_value(i, k) * other_matrix.get_value(k, j);
                }
                result_vec.push(cum_sum);
            }
        }
        return Matrix::new(result_vec, self.rows, other_matrix.columns);
    }
}

fn get_matrix_for_exponentiation() -> Matrix {
    let matrix = Matrix::new(
        vec![
            1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
            1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
            1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
            1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
            1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
            1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
            1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
            1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
            1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
            1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000
            ], 10, 10);
    return matrix;
}

fn get_final_matrix(matrix: &Matrix, exponent: usize) -> Matrix {
    let mut aux_matrix = matrix.clone();
    if exponent > 1 {
        for _ in 0..exponent - 1 {
            aux_matrix = aux_matrix.multiply(matrix);
        }
    }
    return aux_matrix;
}

fn process_final_matrix(matrix: &Matrix) {
    let dimensions = matrix.get_dimensions();
    for i in 0..dimensions.0 {
        for j in 0..dimensions.1 {
            print!("{} ", matrix.get_value(i, j));
        }
        println!("");
    }
}

fn main() {
    let matrix = get_matrix_for_exponentiation();
    let final_matrix = get_final_matrix(&matrix, 100);
    process_final_matrix(&final_matrix);
}
