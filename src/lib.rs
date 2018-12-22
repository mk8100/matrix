//! This is the documentation for `matrix` crate
//! 
//! # Examples
//! 

/// Matrix structure
/// 
use std::ops::{Mul,Add};
use std::fmt::Display;
use std::ops::Rem;
use std::fmt;


// enum MatrixErr{
//   NotEnoughDataInVector("n*m != v.len()"),   
// }


pub struct Matrix<T> 
    where T: MatrixNum<T>{
        nrows: u32,
        ncols: u32,
        data: Vec<T>,
}

pub trait MatrixNum<T>: Mul<T> + Add<T> + Rem<T> + Display {}
impl<T> MatrixNum<T> for T where T: Mul + Add + Rem + Display {}

impl <T> Matrix<T> where T:MatrixNum<T> {
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
/// let m = matrix::Matrix::new(2,3,vec![1,2,3,4,5,6]).unwrap();
/// assert_eq!(m.nrows(),2);
/// assert_eq!(m.ncols(),3);
/// assert_eq!(*m.data().get(3).unwrap(),4);
/// ```
    pub fn new(m:u32, n:u32, v:Vec<T>) -> Result<Matrix<T>, &'static str>{
        if m*n != v.len() as u32 { return Err("") };
        Ok(
            Matrix::<T>{
                nrows : m,
                ncols : n,
                data : v,
            }
        )
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


impl <T> Display for Matrix<T> where T:MatrixNum<T>{
    /// a fmt function for Display Trait implementation
    ///  
    /// # Examples
    /// 
    /// ```
    /// let m = matrix::Matrix::new(2,3,vec![1,1,1,2,2,2]).unwrap();
    /// println!("{}", m);
    /// ```
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c:usize = self.ncols() as usize;
        //let cl = |(idx,el):(usize,&T)| {if idx as u32 % c == 0 {el.to_string() + " |\n"}else{el.to_string()+", "}}; 
        //let w:String = self.data.iter() .enumerate() .map( cl ) .collect::<Vec<String>>() .concat();
        let w = self.data()
                        .chunks(c)
                        .map(|e| prn(e))
                        .collect::<Vec<_>>()
                        .concat();

        write!(f, "\n{}", w)
    }
}

fn prn<T>(v:&[T]) -> String where T:MatrixNum<T>{

    let cl  = |(idx, el):(usize, &T)| {
                    match idx {
                        0 => "| ".to_string() + &el.to_string() + ", ", 
                        i if i == v.len()-1 => el.to_string() + " |\n", 
                        _ => el.to_string() + ", "
                    }
               };

    v.iter()
            .enumerate()
            .map( cl)
            .collect::<Vec<String>>()
            .concat()
}


#[test]
fn display_test(){
    let m = Matrix::new(2,3,vec![1,2,3,4,5,6]).unwrap();
    println!("Matrix = {}",m)
}

#[cfg(test)]
mod utests {

    use super::*;

    #[test]
    fn create_matrix_test() {
        let m = Matrix::new(2,3,vec![1,2,3,4,5,6]).unwrap();

        assert_eq!(m.ncols(), 3 );
    }
}
