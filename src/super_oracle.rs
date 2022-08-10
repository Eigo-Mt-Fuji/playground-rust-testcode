use async_trait::async_trait;
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait SuperOracle {
    // connect to god
    async fn connect_to_god(&self, message: &str) -> u32;
}

pub struct MyWallet {
    pub balance: u32,
}
impl MyWallet {
    // dont forget pub qualifier  if not so, error E0425 private associated function
    pub fn new() -> Self {
        MyWallet{
            // 1万円ください
            balance: 10000,
        }
    }
}

pub async fn depend_god(wallet: &mut MyWallet, oracle: &dyn SuperOracle) { 
    let blessed_rain = oracle.connect_to_god("Give me 100 coin").await;
    wallet.balance += blessed_rain;
    println!("After connecting god, your wallet balance is {}", &wallet.balance);
}
