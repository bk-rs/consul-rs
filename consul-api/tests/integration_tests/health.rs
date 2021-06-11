use std::{collections::BTreeMap, error};

use consul_api::{
    api::health::{ListChecksInState, State},
    Client as _,
};

use super::helpers::{get_client, init_logger};

#[tokio::test]
async fn state() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    let client = get_client()?;

    let mut endpoint = ListChecksInState::new(State::Any);
    endpoint.set_parameter_dc("dc1".to_owned());
    let mut node_meta = BTreeMap::new();
    node_meta.insert("k".to_owned(), "v".into());
    endpoint.set_parameter_node_meta(node_meta);
    let res = client.respond_endpoint(&endpoint).await?;

    println!("state {:?}", res);

    Ok(())
}
