use async_trait::async_trait;

#[allow(unused)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait SuperOracle {
    // connect to god
    async fn connect_to_god(&self, message: &str) -> u32;
}
