use std::time::Duration;

use purchase_api::{client::*, accounts::Account, server::create_server};
use color_eyre::Result;
use tokio::time::sleep;

fn jbutts_account() -> Account {
    Account {
        account_number: 26522,
        mobile_number: 7134926037,
        email_address: "jbutt@gmail.com".to_string(),
        ssn: 508608859,
        dob: "1965-07-29".to_string(),
        zip: 70116,
        account_state: "LA".to_string(),
        city: "New Orleans".to_string(),
        unit: None,
        street_address: "6649 N Blue Gum St".to_string(),
        first_name: "Butt".to_string(),
        last_name: "James".to_string(),
        filter_id: 0,
    }
}

fn msmith_account() -> Account {
    Account {
        account_number: 11111,
        mobile_number: 7134921111,
        email_address: "msmith111@gmail.com".to_string(),
        ssn: 108601119,
        dob: "1965-07-29".to_string(),
        zip: 77006,
        account_state: "TX".to_string(),
        city: "Houston".to_string(),
        unit: None,
        street_address: "4306 Yoakum Blvd".to_string(),
        first_name: "Michael".to_string(),
        last_name: "Smith".to_string(),
        filter_id: 0,
    }
}

fn run_test_server() -> tokio::task::JoinHandle<Result<()>> {
    tokio::spawn(async move { create_server().await })
}

async fn e2e_filter_test(filters: Vec<MyFilter>, expected_accounts: Vec<Account>) -> Result<()> {
    let server = run_test_server();
    sleep(Duration::from_secs(1)).await;

    let mut client = create_client().await?;

    let accounts = run_filters(&mut client, filters).await?;
    
    assert_eq!(accounts, expected_accounts);

    Ok(server.abort())
}

#[tokio::test]
async fn account_number() -> Result<()> {
    let mut filter = MyFilter::default();
    filter.account_number = Some(26522);
    filter.id = 0;
    let filters = vec![filter];

    let accounts = vec![
        jbutts_account()
    ];

    e2e_filter_test(filters, accounts).await
}

#[tokio::test]
async fn mobile_number() -> Result<()> {
    let mut filter = MyFilter::default();
    filter.mobile_number = Some(713492);
    filter.id = 0;
    let filters = vec![filter];

    let accounts = vec![
        jbutts_account(),
        msmith_account()
    ];

    e2e_filter_test(filters, accounts).await
}

#[tokio::test]
async fn email_address() -> Result<()> {
    let mut filter = MyFilter::default();
    filter.email_address = Some("jbutt".to_string());
    filter.id = 0;
    let filters = vec![filter];

    let accounts = vec![
        jbutts_account()
    ];

    e2e_filter_test(filters, accounts).await
}

#[tokio::test]
async fn ssn() -> Result<()> {
    let mut filter = MyFilter::default();
    filter.ssn = Some(8859);
    filter.id = 0;
    let filters = vec![filter];

    let accounts = vec![
        jbutts_account()
    ];

    e2e_filter_test(filters, accounts).await
}