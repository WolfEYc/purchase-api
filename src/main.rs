use accounts::MyAccounts;
use accounts::accounts_service_server::AccountsServiceServer;
use tonic::{transport::Server, Request, Response, Status};
use color_eyre::Result;
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

mod queries;
mod accounts;
mod state;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    state::create_appstate().await?;

    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(GreeterServer::new(MyGreeter::default()))
        .add_service(AccountsServiceServer::new(MyAccounts::default()))
        .serve(addr)
        .await?;

    Ok(())
}