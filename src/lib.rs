//! This is the documentation for `matrix` crate
//! 
//! # Examples
//! 

/// Matrix structure
/// 
use std::ops::{Mul,Add};

pub struct Matrix<T> 
    where T: Mul + Add{
        nrows: u32,
        ncols: u32,
        data: Vec<T>,
}

impl <T> Matrix<T> where T:Mul+Add {

/// funtion for creating a new Matrix structure
/// 
/// # Arguments
/// 
/// * `m:u32` -- rows number
/// * `n:u32` -- columns number 
/// 
/// # Examples
/// ```
/// let m = matrix::Matrix::new(2,3,vec![1,2,3,4,5,6]);
/// assert_eq!(m.nrows(),2);
/// assert_eq!(m.ncols(),3);
/// assert_eq!(*m.data().get(3).unwrap(),4);
/// ```
    pub fn new(m:u32, n:u32, v:Vec<T>) -> Matrix<T>{
        Matrix::<T>{
            nrows : m,
            ncols : n,
            data : v,
        }
    }

    pub fn ncols(&self) -> u32 {
        self.ncols
    }
    pub fn nrows(&self) -> u32 {
        self.nrows
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
}

/// A function multipliing two numbers
///
/// # Arguments
/// 
/// * `a:u32` - first number
/// * `b:u32` - second number
/// 
/// # Remarks
/// 
/// This is an example function
///
/// # Examples
///  
/// ```
/// assert_eq!(matrix::mul(2,4),8);
/// ```
pub fn mul(a:u32, b:u32) -> u32{
 a*b
}

#[cfg(test)]
mod utests {

    use super::*;

    #[test]
    fn mul_test() {
        assert_eq!(mul(2,3), 6);
    }

    #[test]
    fn create_matrix_test() {
        let m = Matrix::new(2,3,vec![1,2,3,4,5,6]);

        assert_eq!(m.ncols(), 3 );
    }
}
