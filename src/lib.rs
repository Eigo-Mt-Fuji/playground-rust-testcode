mod multiply;
pub use crate::multiply::mul;


pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert!(result == 4);
    }
}


