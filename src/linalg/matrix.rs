#![allow(unused)]

use core::{fmt, panic};
use std::collections::btree_map::Iter;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
use std::ops::{Index, IndexMut};
use std::vec;

use f64;

use crate::optimizer::Optimizer;

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    rows: usize,
    columns: usize,
    entries: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(row: usize, column: usize) -> Self {
        let mut vec: Vec<Vec<f64>> = Vec::new();

        for _i in 0..row {
            vec.push(vec![0.0; column]);
        }

        Matrix {
            rows: row,
            columns: column,
            entries: vec,
        }
    }

    pub fn dimension(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }

    pub fn insert(&mut self, input: Vec<f64>) {
        if self.rows * self.columns != input.len() {
            println!("insert vector does not have the number of elements for this matrix")
        } else {
            let mut counter: usize = 0;

            for i in 0..self.rows {
                for j in 0..self.columns {
                    self.entries[i][j] = input[counter];
                    counter += 1;
                }
            }
        }
    }

    pub fn add_row(mut self, row: &Vec<f64>) {
        let self_dim = self.dimension();
        let input_dim = &row.len();

        if (self.dimension().1 != *input_dim) {
            panic!(
                "coloumn size is different,
                {}!={}",
                self_dim.1, *input_dim
            );
        } else {
            for i in 0..self_dim.0 {
                for j in 0..self_dim.1 {
                    self.entries[i][j] = self.entries[i][j] + row[j];
                }
            }
        }
    }

    fn straighten(self) -> Vec<f64> {
        let mut vec: Vec<f64> = Vec::new();

        for i in 0..self.rows {
            for j in 0..self.columns {
                vec.push(self.entries[i][j]);
            }
        }

        vec
    }

    pub fn transpose(&self) -> Self {
        let mut vec: Vec<Vec<f64>> = Vec::new();

        for i in 0..self.columns {
            vec.push(vec![0.0; self.rows]);
        }

        for i in 0..self.rows {
            for j in 0..self.columns {
                vec[j][i] = self.entries[i][j];
            }
        }
        Matrix {
            rows: self.columns,
            columns: self.rows,
            entries: vec,
        }
    }

    pub fn dot(self, input: &Matrix) -> Matrix {
        let dim1 = self.clone().dimension();
        let dim2 = input.clone().dimension();
        let mut vec: Vec<Vec<f64>> = Vec::new();

        if (dim1.1 != dim2.0) {
            panic!(
                "The Coloumn size of the left Matrix: {}x{} \n 
                    doesn't coincide with \n
                    The Row size of the right Matrix {}x{} \n
                    {} != {}",
                dim1.0, dim1.1, dim2.0, dim2.1, dim1.1, dim2.0
            );
        } else {
            for _i in 0..self.rows {
                vec.push(vec![0.0; input.columns]);
            }

            for i in 0..self.rows {
                for j in 0..input.columns {
                    let mut sum = 0.0;

                    for k in 0..input.rows {
                        sum += self.entries[i][k] * input.entries[k][j];
                    }
                    vec[i][j] = sum;
                }
            }

            Matrix {
                rows: self.rows,
                columns: input.columns,
                entries: vec,
            }
        }
    }

    pub fn eye(dimension: usize) -> Self {
        let mut entries: Vec<Vec<f64>> = Vec::new();

        for _i in 0..dimension {
            let mut vec: Vec<f64> = Vec::new();

            for _j in 0..dimension {
                if _i == _j {
                    vec.push(1.0);
                } else {
                    vec.push(0.0);
                }
            }

            entries.push(vec);
        }

        Matrix {
            rows: dimension,
            columns: dimension,
            entries: entries,
        }
    }

    pub fn visualize(&self) {
        for i in 0..self.rows {
            println!("{:?}", self.entries[i]);
        }
    }

    pub fn map<F>(self, fun: F) -> Matrix
    where
        F: Fn(f64) -> f64,
    {
        let mut entries = self.entries.clone();
        let dimension = self.dimension();

        for i in 0..entries.len() {
            entries[i] = entries[i].iter().map(|x| fun(*x)).collect::<Vec<f64>>();
        }

        Matrix {
            rows: dimension.0,
            columns: dimension.1,
            entries: entries,
        }
    }

    pub fn sum(self, axis: Option<bool>) -> Matrix {
        //row sum
        if (axis.unwrap() == false || axis == None) {
            let mut axis_sum: Vec<Vec<f64>> = Vec::new();

            for i in self.entries {
                axis_sum.push(vec![i.iter().sum()]);
            }

            Matrix {
                rows: axis_sum.len(),
                columns: 1 as usize,
                entries: axis_sum,
            }
        } else {
            //coloumn sum
            let mut axis_sum: Vec<f64> = Vec::new();

            for i in 0..self.dimension().1 {
                let mut sum = 0.0;

                for j in 0..self.dimension().0 {
                    sum += self.entries[j][i];
                }

                axis_sum.push(sum);
            }

            Matrix {
                rows: 1 as usize,
                columns: axis_sum.len(),
                entries: vec![axis_sum],
            }
        }
    }
}

impl From<Vec<Vec<f64>>> for Matrix {
    fn from(item: Vec<Vec<f64>>) -> Self {
        let row = item.len();
        let column = item[0].len();

        if (row == 0 || column == 0) {
            panic!("Cannot create Matrix out of an empty vector")
        } else {
            let expected_elements: usize = row * column;
            let mut actual_elements: usize = 0;

            for i in 0..row {
                for j in 0..column {
                    actual_elements += 1;
                }
            }

            if (expected_elements != actual_elements) {
                panic!(
                    "The amount of elements for a Matrix {}x{}\n
                        do not suffice\n
                        {}!={}\n",
                    row, column, expected_elements, actual_elements
                );
            } else {
                Matrix {
                    rows: row,
                    columns: column,
                    entries: item,
                }
            }
        }
    }
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        let dim1 = self.clone().dimension();
        let dim2 = rhs.clone().dimension();

        if (dim1 == dim2) {
            let mut vec: Vec<Vec<f64>> = Vec::new();

            for _i in 0..self.rows {
                vec.push(vec![0.0; self.columns]);
            }

            for i in 0..self.rows {
                for j in 0..self.columns {
                    vec[i][j] = self.entries[i][j] + rhs.entries[i][j];
                }
            }

            Matrix {
                rows: self.rows,
                columns: self.columns,
                entries: vec,
            }
        } else {
            panic!(
                "not the same dimension for addition: {}x{} != {}x{}",
                dim1.0, dim1.1, dim2.0, dim2.1
            );
        }
    }
}

impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Matrix) -> Self::Output {
        let dim1 = self.clone().dimension();
        let dim2 = rhs.clone().dimension();

        if (dim1 == dim2) {
            let mut vec: Vec<Vec<f64>> = Vec::new();

            for _i in 0..self.rows {
                vec.push(vec![0.0; self.columns]);
            }

            for i in 0..self.rows {
                for j in 0..self.columns {
                    vec[i][j] = self.entries[i][j] - rhs.entries[i][j];
                }
            }

            Matrix {
                rows: self.rows,
                columns: self.columns,
                entries: vec,
            }
        } else {
            panic!(
                "not the same dimension for addition: {}x{} != {}x{}",
                dim1.0, dim1.1, dim2.0, dim2.1
            );
        }
    }
}

impl std::ops::Mul<Matrix> for f64 {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        let mut vec: Vec<Vec<f64>> = Vec::new();

        for _i in 0..rhs.rows {
            vec.push(vec![0.0; rhs.columns]);
        }

        for i in 0..rhs.rows {
            for j in 0..rhs.columns {
                vec[i][j] = rhs.entries[i][j] * self;
            }
        }

        Matrix {
            rows: rhs.rows,
            columns: rhs.columns,
            entries: vec,
        }
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [f64] {
        &mut self.entries[index]
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.entries[index.0][index.1]
    }
}

impl Index<usize> for Matrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &[f64] {
        &self.entries[index]
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix<{}, {}> {{", self.rows, self.columns)?;

        for i in 0..self.rows {
            for j in 0..self.columns {
                write!(f, " {}", self[(i, j)])?;
            }
            writeln!(f)?;
        }

        write!(f, "}}")
    }
}

pub struct MatrixIntoIterator {
    matrix: Matrix,
    index_row: usize,
    index_column: usize,
}

impl Iterator for MatrixIntoIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let dimension = self.matrix.dimension();

        if (self.index_column < dimension.1) {
            let result = self.matrix[self.index_row][self.index_column];
            self.index_column += 1;
            Some(result)
        } else if (self.index_row < dimension.0 - 1) {
            self.index_column = 0;
            self.index_row += 1;
            self.next()
        } else {
            None
        }
    }
}

impl IntoIterator for Matrix {
    type Item = f64;
    type IntoIter = MatrixIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        MatrixIntoIterator {
            matrix: self,
            index_row: 0,
            index_column: 0,
        }
    }
}

//------------------------------

#[cfg(test)]
mod test {
    use super::Matrix;
    use crate::prelude::*;

    #[test]
    fn do_test() {
        let entry = vec![
            vec![-0.1, 0.03, 0.06],
            vec![0.03, -0.16, 0.13],
            vec![0.0, -0.03, 0.02],
        ];

        let m0 = Matrix::from(entry);

        let entry = vec![-0.1, 0.03, 0.06, 0.03, -0.16, 0.13, 0.0, -0.03, 0.02];

        let mut m1 = Matrix::new(3, 3);
        m1.insert(entry);

        assert_eq!(m0, m1);

        let mut m1 = Matrix::eye(4);
        m1[0][0] = 42.0;

        println!("{:?}", m1[0][0]);
        println!("{}", m1);
    }

    #[test]
    fn transpose() {
        let mut m1 = Matrix::new(3, 2);
        m1.insert(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let m2 = m1.transpose();

        let mut m3 = Matrix::new(2, 3);
        m3.insert(vec![1.0, 3.0, 5.0, 2.0, 4.0, 6.0]);

        assert_eq!(m2, m3);

        let mut m01 = Matrix::new(1, 1);
        m1.insert(vec![1.0]);

        let m02 = m01.transpose();

        assert_eq!(m01, m02);
    }

    #[test]
    fn dot() {
        let mut m1 = Matrix::new(2, 3);
        m1.insert(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let mut m2 = Matrix::new(3, 2);
        m2.insert(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let mut mdot = Matrix::new(2, 2);
        mdot.insert(vec![22.0, 28.0, 49.0, 64.0]);

        let mtest = m1.dot(&m2);

        assert_eq!(mdot, mtest);
    }

    #[test]
    fn add() {
        let mut m1 = Matrix::new(2, 2);
        m1.insert(vec![1.0, 2.0, 3.0, 4.0]);

        let mut m2 = Matrix::new(2, 2);
        m2.insert(vec![5.0, 6.0, 7.0, 8.0]);

        let m3 = m1 + m2;

        let mut m4 = Matrix::new(2, 2);
        m4.insert(vec![6.0, 8.0, 10.0, 12.0]);

        assert_eq!(m3, m4);
    }

    #[test]
    fn mul() {
        let mut m1 = Matrix::new(2, 4);
        m1.insert(vec![1.0, 2.0, 3.0, 4.0]);

        let m2 = 3.0 * m1;

        let mut m3 = Matrix::new(2, 4);
        m3.insert(vec![3.0, 6.0, 9.0, 12.0]);

        assert_eq!(m2, m3);
    }

    #[test]
    fn sub() {
        let mut m1 = Matrix::new(2, 2);
        m1.insert(vec![5.0, 5.0, 5.0, 5.0]);

        let mut m2 = Matrix::new(2, 2);
        m2.insert(vec![4.0, 3.0, 2.0, 1.0]);

        let m3 = m1 - m2;

        let mut m4 = Matrix::new(2, 2);
        m4.insert(vec![1.0, 2.0, 3.0, 4.0]);

        assert_eq!(m3, m4);
    }

    #[test]
    fn eye() {
        let mut m1 = Matrix::eye(3);

        let mut m2 = Matrix::new(3, 3);
        m2.insert(vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);

        assert_eq!(m1, m2);
    }

    #[test]
    fn map() {
        let mut m1 = Matrix::new(2, 2);
        m1.insert(vec![1.0, 2.0, 3.0, 4.0]);

        let m2 = m1.map(|x| x * 2.0);

        let mut m3 = Matrix::new(2, 2);
        m3.insert(vec![2.0, 4.0, 6.0, 8.0]);

        assert_eq!(m2, m3);

        let mut m1 = Matrix::new(2, 2);
        m1.insert(vec![-1.0, 2.0, -3.0, 4.0]);

        let m2 = m1.map(|x| f64::max(x, 0.0));

        let mut m3 = Matrix::new(2, 2);
        m3.insert(vec![0.0, 2.0, 0.0, 4.0]);

        assert_eq!(m2, m3);

        fn add(i: f64) -> f64 {
            i + 1.0
        }

        let mut m1 = Matrix::new(2, 2);
        m1.insert(vec![1.0, 2.0, 3.0, 4.0]);

        let m2 = m1.map(add);

        let mut m3 = Matrix::new(2, 2);
        m3.insert(vec![2.0, 3.0, 4.0, 5.0]);

        assert_eq!(m2, m3);
    }

    #[test]
    fn iter() {
        let m1 = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0],
        ]);

        for i in m1.into_iter() {
            println!("{:?}", i);
        }

        let m1 = Matrix::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

        let mut iter = m1.into_iter();

        assert_eq!(Some(1.0), iter.next());
        assert_eq!(Some(2.0), iter.next());
        assert_eq!(Some(3.0), iter.next());
        assert_eq!(Some(4.0), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn axis_sum() {
        let m1 = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0],
        ]);

        let m2 = Matrix::from(vec![vec![21.0], vec![57.0]]);

        assert_eq!(m2, m1.sum(Some(false)));

        let m1 = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0],
        ]);

        let m2 = Matrix::from(vec![vec![8.0, 10.0, 12.0, 14.0, 16.0, 18.0]]);

        assert_eq!(m2, m1.sum(Some(true)));
    }
}
