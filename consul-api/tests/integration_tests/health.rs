use std::error;

use consul_api::{
    v1::health::{State, StateEndpoint},
    Client as _,
};

use super::helpers::{get_client, init_logger};

#[tokio::test]
async fn state() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    let client = get_client()?;

    let endpoint = StateEndpoint::new(State::Any);
    let res = client.respond_endpoint(&endpoint).await?;

    println!("state {:?}", res);

    Ok(())
}
