#![allow(unused)]

use core::fmt;
use std::ops::{Add, Mul};
use std::ops::{Index, IndexMut};
use std::vec;

use f64;

enum dataTypes{

    float64(f64),
    uint64(u64),
    int64(i64),

}

#[derive(Clone)]
pub struct Matrix{

    rows: usize,
    columns: usize,
    entries: Vec<Vec<f64>>,


}

impl Matrix{

    pub fn new(row: usize, column: usize)->Self{

        let mut vec:Vec<Vec<f64>> = Vec::new();
        
        for _i in 0..row {

            vec.push(vec![0.0; column]);
            
        }

        Matrix { rows: row, columns: column, entries: vec }

    }

    pub fn insert(&mut self, input: Vec<f64>){

        if self.rows * self.columns != input.len(){

            println!("insert vector does not have the number of elements for this matrix")

        }else{

            let mut counter:usize = 0;

            for i in 0..self.rows{

                for j in 0..self.columns{

                    self.entries[i][j] = input[counter];
                    counter += 1;
                
                }
            }
        }
    }

    fn straighten(self) -> Vec<f64>{

        let mut vec:Vec<f64> = Vec::new();

        for i in 0..self.rows{

            for j in 0..self.columns{

                vec.push(self.entries[i][j]);
            
            }

        }
        
        vec

    }

    pub fn T(&self) -> Self{

        let mut vec:Vec<Vec<f64>> = Vec::new();
        
        for i in 0..self.columns{

            vec.push(vec![0.0; self.rows]);
            
        }

        for i in 0..self.rows{
            for j in 0..self.columns{

                vec[j][i] = self.entries[i][j];

            }
        }
        Matrix { rows: self.columns, columns: self.rows, entries: vec }
    }

    pub fn dot(self, input: &Matrix) -> Matrix {

        let mut vec:Vec<Vec<f64>> = Vec::new();
        
        for _i in 0..self.rows {

            vec.push(vec![0.0; input.columns]);
            
        }

        for i in 0..self.rows{

            for j in 0..input.columns{

                let mut sum = 0.0;

                for k in 0..input.rows{
                    sum += self.entries[i][k] * input.entries[k][j];
                }
                vec[i][j] = sum;
            }

        }

        Matrix { rows: self.rows, columns: input.columns, entries: vec }

    }


    pub fn eye(dimension: usize) -> Self {

        let mut entries:Vec<Vec<f64>> = Vec::new();
        
        for _i in 0..dimension{

            let mut vec: Vec<f64> = Vec::new();

            for _j in 0..dimension{

                if _i == _j {

                    vec.push(1.0);

                }else{

                    vec.push(0.0);

                }

            }
            
            entries.push(vec);
            
        }

        Matrix { rows: dimension, columns: dimension, entries: entries}

    }

    pub fn visualize(&self) {
        for i in 0..self.rows {
            println!("{:?}", self.entries[i]);

        }
    }

}

impl Add for Matrix{
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        
        let mut vec:Vec<Vec<f64>> = Vec::new();
        
        for _i in 0..self.rows{

            vec.push(vec![0.0; self.columns]);
            
        }

        for i in 0..self.rows{

            for j in 0..self.columns{

                vec[i][j]= self.entries[i][j] + rhs.entries[i][j];
            
            }

        }

        Matrix { rows: self.rows, columns: self.columns, entries: vec }

    }
}

impl std::ops::Mul<Matrix> for f64 {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {

        let mut vec:Vec<Vec<f64>> = Vec::new();
        
        for _i in 0..rhs.rows{

            vec.push(vec![0.0; rhs.columns]);
            
        }

        for i in 0..rhs.rows{

            for j in 0..rhs.columns{

                vec[i][j]= rhs.entries[i][j] * self;
            
            }

        }

        Matrix { rows: rhs.rows, columns: rhs.columns, entries: vec }
        
    }
}

impl IndexMut<usize> for Matrix {
    
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Vec<f64> {

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
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Vec<f64> {
        
        &self.entries[index]

    }
}

#[cfg(test)]
mod test {
    use super::Matrix;

    #[test]
    fn do_test(){

        let mut m1 = Matrix::new(3, 2);
        m1.visualize();
        println!("\n");
        
        let input = vec![1.0 , 2.0, 3.0, 4.0, 5.0, 6.0,];
        m1.insert(input);
        m1.visualize();
        println!("\n");
        
        let m2 = m1.T();
        m2.visualize();
        println!("\n");

        let mut m3 = Matrix::new(2, 2);
        m3.insert(vec![1.0,2.0, 3.0,4.0]);

        let mut m4 = Matrix::new(2, 2);
        m4.insert(vec![5.0,6.0, 7.0,8.0]);

        let m5 = m3 + m4;
        m5.visualize();
        println!("\n");

        let m6 = 3.0 * m1;
        m6.visualize();
        println!("\n");


        //-----------------------------------
        let mut m7 = Matrix::new(2, 3);
        m7.insert(vec![1.0 , 2.0, 3.0, 4.0, 5.0, 6.0,]);
        m7.visualize();
        println!("\n");

        let mut m8 = Matrix::new(3, 2);
        m8.insert(vec![1.0 , 2.0, 3.0, 4.0, 5.0, 6.0,]);
        m8.visualize();
        println!("\n");

        let m9 = m7.dot(&m8);
        m9.visualize();
        println!("\n");
        //-----------------------------------

        let m10 = Matrix::eye(4);
        m10.visualize();
        println!("\n"); 
        //-----------------------------------

        println!("{:?}", m10[0][0]);

    }


}