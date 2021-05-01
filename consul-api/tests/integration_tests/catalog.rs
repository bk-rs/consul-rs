use std::error;

use consul_api::{v1::catalog::DatacentersEndpoint, Client as _};

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
