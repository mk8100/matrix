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

pub trait MatrixNum<T>: Mul<T> + Add<T> + Rem<T> + Display {}
impl<T> MatrixNum<T> for T where T: Mul + Add + Rem + Display {}

impl <T> Matrix<T> where T:MatrixNum<T> {
//impl <T> Matrix<T> where T:Mul+Add+Rem+Display {

/// funtion for creating a new Matrix structure
/// 
/// # Arguments
/// 
/// * `m:u32` - rows number
/// * `n:u32` - columns number 
/// * `v:Vec<T>` - vector of data (numbers) that will be moved into a Matrix struct
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

use std::fmt::Display;
use std::ops::Rem;
use std::fmt;

impl <T> Display for Matrix<T> where T:Mul+Add+Rem+Display{
    /// a fmt function for Display Trait implementation
    ///  
    /// # Examples
    /// 
    /// ```
    /// let m = matrix::Matrix::new(2,3,vec![1,1,1,2,2,2]);
    /// println!("{}", m);
    /// ```
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c:u32 = self.ncols();
        let cl = |(idx,el):(usize,&T)| {if idx as u32 % c == 0 {el.to_string() + " |\n"}else{el.to_string()+", "}}; 
        let w:String = self.data.iter()
                        .enumerate()
                        .map( cl )
                        .collect::<Vec<String>>()
                        .concat();
        write!(f, "\n{}", w)
    }
}


#[test]
fn display_test(){
    let m = Matrix::new(2,3,vec![1,2,3,4,5,6]);
    println!("Matrix = {}",m)
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
