use super::super_oracle::SuperOracle;
use super::my_wallet::MyWallet;

pub async fn depend_god(wallet: &mut MyWallet, oracle: &dyn SuperOracle) { 
    let blessed_rain = oracle.connect_to_god("Give me 100 coin").await;
    wallet.balance += blessed_rain;
    println!("After connecting god, your wallet balance is {}", &wallet.balance);
}

#[cfg(test)]
mod tests {
    use crate::super_oracle::MockSuperOracle;
    use crate::super_oracle_depend_god::depend_god;
    use crate::my_wallet::MyWallet;
    use mockall::predicate::eq;

    // use tokio for making test functions with async  qualifier.
    #[tokio::test]
    async fn test_awesome_mock() {
        let mut oracle = MockSuperOracle::new();
        oracle.expect_connect_to_god()    .with(eq("Give me 100 coin")).times(1).return_const(100 as u32);

        let mut wallet = MyWallet::new(10000);
        depend_god(&mut wallet, &oracle).await;
        assert_eq!(wallet.balance, 10100);
        println!("wallet.balance={}", &wallet.balance);
    }
}
