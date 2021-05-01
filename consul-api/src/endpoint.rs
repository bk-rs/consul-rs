use http::{header::ACCEPT, Method, Uri, Version};
pub use http::{Request, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub type Body = Vec<u8>;

pub mod prelude {
    pub use http::Method;
}

pub trait Endpoint {
    type ResponseOkBody: DeserializeOwned;

    fn method(&self) -> Method;
    fn path(&self) -> String;

    fn render_request(&self) -> Result<Request<Body>, EndpointRenderRequestError> {
        let uri: Uri = self.path().parse()?;

        let request = Request::builder()
            .method(Method::GET)
            .uri(uri)
            .version(Version::HTTP_11)
            .header(ACCEPT, "application/json")
            .body(vec![])?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ResponseOkBody, EndpointParseResponseError> {
        let body = response.body();
        let status_code = response.status();

        if status_code.is_success() {
            if body.is_empty() {
                return Ok(serde_json::from_value(Value::Null)?);
            }

            Ok(serde_json::from_slice(body)?)
        } else {
            Err(EndpointParseResponseError::Failed(
                status_code,
                body.to_owned(),
            ))
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum EndpointRenderRequestError {
    #[error("HttpInvalidUri {0:?}")]
    HttpInvalidUri(#[from] http::uri::InvalidUri),
    #[error("SerdeJsonError {0:?}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("HttpError {0:?}")]
    HttpError(#[from] http::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum EndpointParseResponseError {
    #[error("SerdeJsonError {0:?}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Failed {0:?} {1:?}")]
    Failed(http::StatusCode, Vec<u8>),
}
