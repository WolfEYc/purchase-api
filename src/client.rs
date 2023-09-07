use std::ops::{Deref, DerefMut};
use accounts::Filter;
use accounts::accounts_service_client::AccountsServiceClient;
use color_eyre::Result;
use tonic::Request;
use tonic::transport::Channel;
use crate::accounts::{self, Account};

#[derive(Default)]
pub struct MyFilter(pub Filter);

impl Deref for MyFilter {
    type Target = Filter;
    
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for MyFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub async fn run_filter(client: &mut AccountsServiceClient<Channel>, filter: Filter) -> Result<Vec<Account>> {
    let res = client.read(Request::new(filter)).await?;
    let accounts = res.into_inner().accounts;

    Ok(accounts)
}

pub async fn create_client() -> Result<AccountsServiceClient<Channel>> {
    Ok(AccountsServiceClient::connect("http://0.0.0.0:50051").await?)
}