use std::ops::{Deref, DerefMut};
use accounts::Filter;
use accounts::accounts_service_client::AccountsServiceClient;
use color_eyre::Result;
use tonic::Request;
use tonic::transport::Channel;
use tokio::time;
use tokio::time::Duration;
use crate::accounts::{self, Account};

#[derive(Default)]
pub struct MyFilter(Filter);

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

pub async fn run_filters(client: &mut AccountsServiceClient<Channel>, filters: Vec<MyFilter>) -> Result<Vec<Account>> {
    
    let outbound = async_stream::stream! {
        let mut interval = time::interval(Duration::from_secs(1));
        
        for filter in filters {
            interval.tick().await;

            yield filter.0;
        }
    };

    let response = client.read(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    let mut accounts = Vec::with_capacity(10);

    while let Some(account) = inbound.message().await? {
        accounts.push(account);
    }

    Ok(accounts)
}

pub async fn create_client() -> Result<AccountsServiceClient<Channel>> {
    Ok(AccountsServiceClient::connect("http://0.0.0.0:50051").await?)
}