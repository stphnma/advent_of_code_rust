use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Matrix {
    values: Vec<Vec<i32>>,
    nrows: usize,
    ncols: usize,
}

impl Matrix {
    fn new(values: Vec<Vec<i32>>) -> Matrix {
        let nrows = values.len();
        let ncols = values[0].len();
        Matrix {
            values: values,
            nrows: nrows,
            ncols: ncols,
        }
    }

    fn get(&self, row_idx: usize, col_idx: usize) -> Option<i32> {
        Some(*self.values.get(row_idx)?.get(col_idx)?)
    }
}

struct Tree {
    num_above: i32,
    num_below: i32,
    num_left: i32,
    num_right: i32,
}

fn count_visible(
    row_indices: Vec<usize>,
    col_indices: Vec<usize>,
    matrix: &Matrix,
    visible: &mut HashSet<[usize; 2]>,
) {
    let mut max_val: i32 = -1;
    for i in row_indices {
        for j in &col_indices {
            if let Some(val) = matrix.get(i, *j) {
                if val > max_val {
                    visible.insert([i, *j]);
                    max_val = val;
                }
            };
        }
    }
}

pub fn main() {
    let contents = fs::read_to_string("problem8_input.txt").unwrap();

    let matrix_values = contents
        .lines()
        .map(|l| l.chars().map(|a| a as i32).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let matrix = Matrix::new(matrix_values);

    println!("{}, {}", matrix.nrows, matrix.ncols);

    let mut visible: HashSet<[usize; 2]> = HashSet::new();

    for i in 0..matrix.nrows {
        count_visible(
            vec![i],
            (0..matrix.ncols).collect::<Vec<usize>>(),
            &matrix,
            &mut visible,
        );
        count_visible(
            vec![i],
            (0..matrix.ncols).rev().collect::<Vec<usize>>(),
            &matrix,
            &mut visible,
        );
    }

    for j in 0..matrix.ncols {
        count_visible(
            (0..matrix.nrows).collect::<Vec<usize>>(),
            vec![j],
            &matrix,
            &mut visible,
        );
        count_visible(
            (0..matrix.nrows).rev().collect::<Vec<usize>>(),
            vec![j],
            &matrix,
            &mut visible,
        );
    }
    println!("{:?}", visible.len());
}
