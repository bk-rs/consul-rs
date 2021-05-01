use std::error;

use super::helpers::init_logger;

#[tokio::test]
async fn datacenters() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    Ok(())
}
