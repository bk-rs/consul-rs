use std::error;

use consul_api::{
    v1::health::{ListChecksInState, State},
    Client as _,
};

use super::helpers::{get_client, init_logger};

#[tokio::test]
async fn state() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    let client = get_client()?;

    let mut endpoint = ListChecksInState::new(State::Any);
    endpoint.set_ns("dc1".to_owned());
    let res = client.respond_endpoint(&endpoint).await?;

    println!("state {:?}", res);

    Ok(())
}
