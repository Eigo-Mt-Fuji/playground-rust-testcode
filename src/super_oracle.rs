#![crate_name = "adder"]

use async_trait::async_trait;
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait SuperOracle {
    // connect to god
    async fn connect_to_god(&self, message: &str) -> u32;
}

/// A wallet must have a balance
/// お財布はお金が入っていてこそ意味がある。
pub struct MyWallet {
    pub balance: u32,
}
impl MyWallet {

    /// Returns a wallet instance
    /// 新しいMyWalletを返します。
    /// dont forget pub qualifier  if not so, error E0425 private associated function
    /// # Arguments
    ///
    /// * `balance` - how much your wallet contains a money(unit: JPY)
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate adder;
    /// use adder::super_oracle::MyWallet;
    /// let wallet = MyWallet::new(10000);
    /// ```
    pub fn new(balance: u32) -> Self {
        MyWallet{
            // 1万円ください
            balance: balance,
        }
    }
}

pub async fn depend_god(wallet: &mut MyWallet, oracle: &dyn SuperOracle) { 
    let blessed_rain = oracle.connect_to_god("Give me 100 coin").await;
    wallet.balance += blessed_rain;
    println!("After connecting god, your wallet balance is {}", &wallet.balance);
}
