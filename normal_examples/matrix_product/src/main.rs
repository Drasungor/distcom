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
        assert!(row < self.rows, "Invalid row");
        assert!(column < self.columns, "Invalid row");
        return self.data[row * self.columns + column];
    }

    pub fn multiply(&self, other_matrix: &Matrix) -> Matrix {
        let mut result_vec = Vec::<i64>::new();
        for i in 0..self.rows {
            for j in 0..self.columns {
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

fn get_matrixes_for_product() -> Vec<Matrix> {
    let mut matrixes = Vec::<Matrix>::new();
    let mut dimensions_array = Vec::<(usize, usize)>::new();



    for i in 1..dimensions_array.len() {
        // rows_amount = current_dimensions.0
        // columns_amount = current_dimensions.1
        assert!(dimensions_array[i].0 == dimensions_array[i - 1].1);
    }
    return matrixes;
}

fn get_final_matrix(matrixes: &Vec<Matrix>) -> Matrix {
    // let mut last_matrix = matrixes[0].clone();
    let mut last_matrix = matrixes[0].multiply(&matrixes[1]);
    if matrixes.len() > 2 {
        for i in 2..matrixes.len() {
            last_matrix = last_matrix.multiply(&matrixes[i]);
        }
    }
    return last_matrix;
}

fn process_final_matrix(matrix: &Matrix) {
    let dimensions = matrix.get_dimensions();
    for i in 0..dimensions.0 {
        for j in 0..dimensions.1 {
            println!("{} ", matrix.get_value(i, j));
        }
        println!("");
    }
}

fn main() {
    let product_matrixes = get_matrixes_for_product();
    let final_matrix = get_final_matrix(&product_matrixes);
    process_final_matrix(&final_matrix);
}
