use rand;
use std;

pub struct Matrix {
    n_rows: usize,
    n_cols: usize,
    n_values: usize,
    values: Vec<f64>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum MatrixError {
    IndexOutOfRange,
}


impl Matrix {
    pub fn new(n_rows: usize, n_cols: usize) -> Self {
        Matrix {
            n_rows: n_rows,
            n_cols: n_cols,
            n_values: n_rows * n_cols,
            values: std::iter::repeat(0.0).take(n_rows * n_cols).collect::<Vec<f64>>(),
        }
    }

    pub fn new_randomized(n_rows: usize, n_cols: usize) -> Self {
        let mut values = Vec::<f64>::with_capacity(n_rows * n_cols);
        for _ in 0..n_rows * n_cols {
            values.push(rand::random::<f64>());
        }
        Matrix {
            n_rows: n_rows,
            n_cols: n_cols,
            n_values: n_rows * n_cols,
            values: values,
        }
    }

    pub fn n_rows(&self) -> usize {
        self.n_rows
    }

    pub fn n_cols(&self) -> usize {
        self.n_cols
    }


    pub fn transposed(&self) -> Self {
        let mut transposed_values = std::iter::repeat(0.0)
                                               .take(self.n_values)
                                               .collect::<Vec<f64>>();
        for i in 0..self.n_rows {
            for j in 0..self.n_cols {
                let transposed_index = (j * self.n_rows) + i;
                transposed_values[transposed_index] = self.unchecked_at(i, j);
            }
        }
        Matrix {
            n_rows: self.n_cols,
            n_cols: self.n_rows,
            n_values: self.n_values,
            values: transposed_values,
        }
    }

    pub fn at(&self, row: usize, col: usize) -> Result<f64, MatrixError> {
        let index = (self.n_cols * row) + col;
        if index < self.n_values {
            Ok(self.values[index])
        } else {
            Err(MatrixError::IndexOutOfRange)
        }
    }

    pub fn mutable_at(&mut self, row: usize, col: usize) -> Result<&mut f64, MatrixError> {
        let index = (self.n_cols * row) + col;
        if index < self.n_values {
            Ok(&mut self.values[index])
        } else {
            Err(MatrixError::IndexOutOfRange)
        }
    }

    pub fn unchecked_at(&self, row: usize, col: usize) -> f64 {
        self.values[(self.n_cols * row) + col]
    }

    pub fn unchecked_mutable_at(&mut self, row: usize, col: usize) -> &mut f64 {
        &mut self.values[(self.n_cols * row) + col]
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.n_rows {
            for j in 0..self.n_cols {
                write!(f, "{} ", self.unchecked_at(i, j))?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}