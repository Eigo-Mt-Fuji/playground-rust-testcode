#![crate_name = "adder"]

// pub qualifierないと、テストコードやrust docのexampleから参照できないので注意
pub mod super_oracle;
pub mod super_oracle_depend_god;
pub mod my_wallet;

mod calculator;
pub use crate::calculator::multiply::mul;

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod tests {

    use crate::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert!(result == 4);
    }
}
