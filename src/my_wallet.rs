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
    /// use adder::my_wallet::MyWallet;
    /// let wallet = MyWallet::new(10000);
    /// ```
    pub fn new(balance: u32) -> Self {
        MyWallet{
            // 1万円ください
            balance: balance,
        }
    }
}
