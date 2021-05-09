use std::error;

use consul_api::{
    api::health::{ListChecksInState, State},
    Client as _,
};
use serde_json::Map;

use super::helpers::{get_client, init_logger};

#[tokio::test]
async fn state() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    let client = get_client()?;

    let mut endpoint = ListChecksInState::new(State::Any);
    endpoint.set_ns("dc1".to_owned());
    let mut node_meta = Map::new();
    node_meta.insert("k".to_owned(), "v".into());
    endpoint.set_node_meta(node_meta);
    let res = client.respond_endpoint(&endpoint).await?;

    println!("state {:?}", res);

    Ok(())
}
