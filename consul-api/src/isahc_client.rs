use std::time::Duration;

use futures_core::future::BoxFuture;
use isahc::{config::Configurable, AsyncReadResponseExt as _, HttpClient};

pub use crate::client::Client;
use crate::client::{
    Body, ClientRespondEndpointError as BaseClientRespondEndpointError, Request, Response,
};

#[derive(Debug, Clone)]
pub struct IsahcClient {
    http_client: HttpClient,
    base_url: Option<String>,
}

impl IsahcClient {
    pub fn new() -> Result<Self, isahc::Error> {
        Ok(Self::with(
            HttpClient::builder()
                .timeout(Duration::from_secs(5))
                .connect_timeout(Duration::from_secs(5))
                .build()?,
        ))
    }

    pub fn with(http_client: HttpClient) -> Self {
        Self {
            http_client,
            base_url: None,
        }
    }

    pub fn base_url(&mut self, val: impl Into<String>) -> &mut Self {
        self.base_url = Some(val.into());

        self
    }
}

impl<'a, 'async_trait> Client<'a, 'async_trait> for IsahcClient
where
    'a: 'async_trait,
{
    type Error = isahc::Error;

    fn base_url(&self) -> Option<String> {
        self.base_url.to_owned()
    }

    fn respond(
        &'a self,
        request: Request<Body>,
    ) -> BoxFuture<'async_trait, Result<Response<Body>, Self::Error>> {
        Box::pin(async move {
            let resp = self.http_client.send_async(request).await?;
            let (head, body) = resp.into_parts();

            let mut body_buf = Vec::with_capacity(body.len().unwrap_or_else(|| 4 * 1024) as usize);

            let mut resp = Response::from_parts(head, body);
            resp.copy_to(&mut body_buf).await?;

            let (head, _) = resp.into_parts();
            let resp = Response::from_parts(head, body_buf);

            Ok(resp)
        })
    }
}

pub type ClientRespondEndpointError = BaseClientRespondEndpointError<isahc::Error>;
