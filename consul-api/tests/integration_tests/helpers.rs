use std::{env, error};

use consul_api::IsahcClient;
use log::debug;

pub(super) fn get_client() -> Result<IsahcClient, Box<dyn error::Error>> {
    let url = env::var("CONSUL_HTTP_URL")?;

    debug!("CONSUL_HTTP_URL {}", url);

    let mut client = IsahcClient::new().unwrap();

    client.set_base_url(url);

    Ok(client)
}

pub(super) fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}
