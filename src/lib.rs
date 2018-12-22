
///
///
///
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
}
