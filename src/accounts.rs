use std::pin::Pin;
use accounts_service_server::AccountsService;
use tonic::{Request, Response, Status, Streaming};
use tokio_stream::{StreamExt, Stream};
use crate::queries::account_queries;

tonic::include_proto!("accounts");

#[derive(Debug, Default)]
pub struct MyAccounts {}

#[tonic::async_trait]
impl AccountsService for MyAccounts {
    type readStream = Pin<Box<dyn Stream<Item = Result<Account, Status>> + Send  + 'static>>;

    async fn read(&self, request: Request<Streaming<Filter>>) -> Result<Response<Self::readStream>, Status> {
        let mut stream = request.into_inner();
        
        let output = async_stream::try_stream! {
            while let Some(filter) = stream.next().await {
                let filter = filter?;
                let accounts = account_queries::read(&filter)
                .await
                .map_err(|err| Status::internal(err.to_string()))?;

                for account in accounts {
                    yield account;
                }
            }
        };

        Ok(Response::new(Box::pin(output)))
    }
}