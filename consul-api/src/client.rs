use std::fmt;

use futures_core::future::BoxFuture;
use url::Url;

pub use crate::endpoint::{Body, Request, Response};
use crate::endpoint::{Endpoint, EndpointParseResponseError, EndpointRenderRequestError};

const BASE_URL_DEFAULT: &str = "http://127.0.0.1:8500";

pub trait Client<'a, 'async_trait>
where
    'a: 'async_trait,
    Self: Sync + 'async_trait,
{
    type Error: fmt::Debug;

    fn base_url(&self) -> Option<String> {
        None
    }

    fn respond_before(&self, request: Request<Body>) -> Result<Request<Body>, String> {
        Ok(request)
    }
    fn respond_after(&self, _response: &Response<Body>) {}

    //
    fn respond(
        &'a self,
        request: Request<Body>,
    ) -> BoxFuture<'async_trait, Result<Response<Body>, Self::Error>>;

    fn respond_endpoint<EP>(
        &'a self,
        endpoint: &'a EP,
    ) -> BoxFuture<
        'async_trait,
        Result<<EP as Endpoint>::ResponseOkBody, ClientRespondEndpointError<Self::Error>>,
    >
    where
        EP: Endpoint + Send + Sync,
    {
        Box::pin(async move {
            let mut request = endpoint.render_request()?;

            let uri = request.uri();

            let url: Url = self
                .base_url()
                .unwrap_or_else(|| BASE_URL_DEFAULT.to_owned())
                .parse()?;
            let mut url = url.join(uri.path())?;
            url.set_query(uri.query());

            *request.uri_mut() = url.as_str().parse()?;

            let request = self
                .respond_before(request)
                .map_err(ClientRespondEndpointError::UpdateRequestFailed)?;

            let response = self
                .respond(request)
                .await
                .map_err(ClientRespondEndpointError::RespondError)?;

            self.respond_after(&response);

            let response = endpoint.parse_response(response)?;

            Ok(response)
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ClientRespondEndpointError<RE>
where
    RE: fmt::Debug,
{
    #[error("RenderRequestError {0:?}")]
    RenderRequestError(#[from] EndpointRenderRequestError),
    #[error("UrlParseError {0:?}")]
    UrlParseError(#[from] url::ParseError),
    #[error("HttpInvalidUri {0:?}")]
    HttpInvalidUri(#[from] http::uri::InvalidUri),
    #[error("UpdateRequestFailed {0:?}")]
    UpdateRequestFailed(String),
    #[error("RespondError {0:?}")]
    RespondError(RE),
    #[error("ParseResponseError {0:?}")]
    ParseResponseError(#[from] EndpointParseResponseError),
}
