use std::error;

use consul_api::{
    v1::catalog::{DatacentersEndpoint, NodesEndpoint, ServicesEndpoint},
    Client as _,
};

use super::helpers::{get_client, init_logger};

#[tokio::test]
async fn datacenters() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    let client = get_client()?;

    let endpoint = DatacentersEndpoint;
    let res = client.respond_endpoint(&endpoint).await?;

    println!("datacenters {:?}", res);

    assert_eq!(res, vec!["dc1"]);

    Ok(())
}

#[tokio::test]
async fn nodes() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    let client = get_client()?;

    let endpoint = NodesEndpoint;
    let res = client.respond_endpoint(&endpoint).await?;

    println!("nodes {:?}", res);

    Ok(())
}

#[tokio::test]
async fn services() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    let client = get_client()?;

    let endpoint = ServicesEndpoint;
    let res = client.respond_endpoint(&endpoint).await?;

    println!("services {:?}", res);

    Ok(())
}
