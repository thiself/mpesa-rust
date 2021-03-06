use mpesa::{Mpesa,Environment,CommandId};
use dotenv;
use std::env;

#[test]
fn b2c_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox, // or environment variable
        env::var("INIT_PASSWORD").unwrap(),
    );

    println!("{:#?}", client);

    let b2c_response = client.b2c(
        "testapi496",
        CommandId::BusinessPayment,
        1000,
        "600496",
        "254708374149",
        "gg",
        "https://muriuki.dev",
        "https://muriuki.dev/blog",
        "Test",
    ).unwrap();

    println!("B2c response -> {:#?}", b2c_response);

    assert_eq!(b2c_response.ResponseCode, "0".to_string());
}