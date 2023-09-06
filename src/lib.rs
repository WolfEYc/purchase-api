pub mod state;
pub mod client;
pub mod server;

pub mod accounts {
    tonic::include_proto!("accounts");
}