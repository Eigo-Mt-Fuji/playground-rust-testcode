mod super_oracle;

mod calculator;
pub use crate::calculator::multiply::mul;

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::add;
    use crate::super_oracle::MockSuperOracle;
    use crate::super_oracle::MyWallet;
    use crate::super_oracle::depend_god;
    use mockall::predicate::eq;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert!(result == 4);
    }

    // use tokio for making test functions with async  qualifier.
    #[tokio::test]
    async fn test_awesome_mock() {
        let mut oracle = MockSuperOracle::new();
        oracle.expect_connect_to_god()    .with(eq("Give me 100 coin")).times(1).return_const(100 as u32);

        let mut wallet = MyWallet::new();
        depend_god(&mut wallet, &oracle).await;
        
        println!("wallet.balance={}", &wallet.balance);
    }
}
