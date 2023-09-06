use sqlx::{QueryBuilder, Postgres, types::chrono::NaiveDate, FromRow};
use color_eyre::Result;
use crate::{accounts::{Filter, Account}, state::state};

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
            ssn: value.ssn.parse()?,
            email_address: value.email_address.clone(),
            mobile_number: value.mobile_number.parse()?,
            account_number: value.account_number,
            filter_checksum: 0
        })
    }
}

fn db_to_proto(db_accounts: Vec<DBAccount>) -> Result<Vec<Account>> {
    db_accounts.iter().map(|c| Account::try_from(c)).collect()
}

pub async fn read(filter: &Filter) -> Result<Vec<Account>> {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM account WHERE ");

    if let Some(account_number) = filter.account_number {
        query.push("account_number = $1").push_bind(account_number);
        let results: Vec<DBAccount> = query.build_query_as()
            .fetch_all(&state().db)
            .await?;

        return db_to_proto(results);
    };

    let mut seperated = query.separated(" AND ");
    if let Some(zip) = &filter.zip {
        seperated.push("zip = $1").push_bind(zip);
    }
    if let Some(account_state) = &filter.account_state {
        seperated.push("account_state = $2").push_bind(account_state);
    }
    if let Some(mobile_number) = filter.mobile_number {
        seperated.push("mobile_number LIKE $3%").push_bind(mobile_number.to_string());
    }
    if let Some(email_address) = &filter.email_address {
        seperated.push("email_address LIKE %$4%").push_bind(email_address);
    }
    if let Some(ssn) = &filter.ssn {
        seperated.push("ssn LIKE %$5").push_bind(ssn);
    }
    if let Some(city) = &filter.city {
        seperated.push("city LIKE %$6%").push_bind(city);
    }
    if let Some(street_address) = &filter.street_address {
        seperated.push("street_address LIKE %$7%").push_bind(street_address);
    }
    if let Some(last_name) = &filter.last_name {
        seperated.push("last_name LIKE $8%").push_bind(last_name);
    }
    if let Some(first_name) = &filter.first_name {
        seperated.push("first_name LIKE $9%").push_bind(first_name);
    }
    if let Some(unit) = &filter.unit {
        seperated.push("unit = $10%").push_bind(*unit as i16);
    }
    if let Some(dob) = &filter.dob {
        query.push("ORDER BY ABS(EXTRACT(EPOCH FROM dob - $11::timestamp))").push_bind(dob);
    }
    
    let results = query.build_query_as()
        .fetch_all(&state().db)
        .await?;

    db_to_proto(results)
}