use std::{env, error};

use log::debug;

pub(super) fn get_client() -> Result<(), Box<dyn error::Error>> {
    let url = env::var("CONSUL_HTTP_URL")?;

    debug!("CONSUL_HTTP_URL {}", url);

    Ok(())
}

pub(super) fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}
