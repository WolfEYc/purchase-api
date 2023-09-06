use std::pin::Pin;
use accounts_service_server::AccountsService;
use purchase_api::accounts::Account;
use purchase_api::accounts::Filter;
use purchase_api::accounts::accounts_service_server;
use purchase_api::state::create_appstate;
use sqlx::Execute;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status, Streaming};
use tokio_stream::Stream;
use color_eyre::Result;
use sqlx::FromRow;
use sqlx::types::chrono::NaiveDate;
use sqlx::Postgres;
use sqlx::QueryBuilder;
use purchase_api::state::state;
use tonic::transport::Server;
use accounts_service_server::AccountsServiceServer;

#[derive(Debug, FromRow)]
pub struct DBAccount {
    pub last_name: String,
    pub first_name: String,
    pub street_address: String,
    pub unit: Option<i16>,
    pub city: String,
    pub account_state: String,
    pub zip: i32,
    pub dob: NaiveDate,
    pub ssn: String,
    pub email_address: String,
    pub mobile_number: String,
    pub account_number: i64,
}

impl TryFrom<&DBAccount> for Account {
    type Error = color_eyre::Report;
    
    fn try_from(value: &DBAccount) -> Result<Self, Self::Error> {
        Ok(Account {
            last_name: value.last_name.clone(),
            first_name: value.first_name.clone(),
            street_address: value.street_address.clone(),
            unit: value.unit.map(|u| u as i32),
            city: value.city.clone(),
            account_state: value.account_state.clone(),
            zip: value.zip,
            dob: value.dob.to_string(),
            ssn: value.ssn.replace("-", "").parse()?,
            email_address: value.email_address.clone(),
            mobile_number: value.mobile_number.parse()?,
            account_number: value.account_number,
            filter_id: 0
        })
    }
}

fn db_to_proto(db_accounts: Vec<DBAccount>, filter: Filter) -> Result<Vec<Account>> {
    
    let mut accounts = Vec::with_capacity(db_accounts.len());
    for db_account in db_accounts {
        let mut account = Account::try_from(&db_account)?;
        account.filter_id = filter.id;
        accounts.push(account);
    }
    
    Ok(accounts)
}

pub async fn read(filter: Filter) -> Result<Vec<Account>> {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM account WHERE ");

    if let Some(account_number) = filter.account_number {
        query.push("account_number = ").push_bind(account_number);
        let query = query.build_query_as();
        println!("{:?}", query.sql());

        let results = query.fetch_all(&state().db).await?;

        return db_to_proto(results, filter);
    };

    let mut seperated = query.separated(" AND ");
    if let Some(zip) = &filter.zip {
        seperated.push("zip = ").push_bind_unseparated(zip);
    }
    if let Some(unit) = &filter.unit {
        seperated.push("unit = ").push_bind_unseparated(*unit as i16);
    }
    if let Some(account_state) = &filter.account_state {
        seperated.push("account_state = ").push_bind_unseparated(account_state);
    }
    if let Some(mobile_number) = filter.mobile_number {
        seperated.push("mobile_number LIKE ").push_bind_unseparated(mobile_number.to_string()).push_unseparated(" || '%'");
    }
    if let Some(email_address) = &filter.email_address {
        seperated.push("email_address LIKE '%' || ").push_bind_unseparated(email_address).push_unseparated(" || '%'");
    }
    if let Some(ssn) = &filter.ssn {
        seperated.push("ssn LIKE '%' || ").push_bind_unseparated(ssn.to_string()).push_unseparated(" || '%'");
    }
    if let Some(city) = &filter.city {
        seperated.push("city LIKE '%' || ").push_bind_unseparated(city).push_unseparated(" || '%'");
    }
    if let Some(street_address) = &filter.street_address {
        seperated.push("street_address LIKE '%' || ").push_bind_unseparated(street_address).push_unseparated("|| '%'");
    }
    if let Some(last_name) = &filter.last_name {
        seperated.push("last_name LIKE ").push_bind_unseparated(last_name).push_unseparated(" || '%'");
    }
    if let Some(first_name) = &filter.first_name {
        seperated.push("first_name LIKE ").push_bind_unseparated(first_name).push_unseparated(" || '%'");
    }
    if let Some(dob) = &filter.dob {
        query.push("\nORDER BY ABS(EXTRACT(EPOCH FROM dob - ").push_bind(dob).push("::timestamp))");
    }
    query.push(" LIMIT 10");

    let query = query.build_query_as();
    println!("{:?}", query.sql());

    let results = query
        .fetch_all(&state().db)
        .await?;

    db_to_proto(results, filter)
}

#[derive(Debug, Default)]
pub struct MyAccounts {}

#[tonic::async_trait]
impl AccountsService for MyAccounts {
    type ReadStream = Pin<Box<dyn Stream<Item = Result<Account, Status>> + Send  + 'static>>;

    async fn read(&self, request: Request<Streaming<Filter>>) -> Result<Response<Self::ReadStream>, Status> {
        let mut stream = request.into_inner();
        
        let output = async_stream::try_stream! {
            while let Some(filter) = stream.next().await {
                let filter = filter?;
                println!("{:#?}", filter);
                let accounts = read(filter)
                .await
                .map_err(|err| Status::internal(err.to_string()))?;
                
                for account in accounts {
                    println!("{:#?}", account);
                    yield account;
                }
            }
        };

        Ok(Response::new(Box::pin(output)))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    create_appstate().await?;

    let addr = "0.0.0.0:50051".parse()?;

    Server::builder()
        .add_service(AccountsServiceServer::new(MyAccounts::default()))
        .serve(addr)
        .await?;

    Ok(())
}