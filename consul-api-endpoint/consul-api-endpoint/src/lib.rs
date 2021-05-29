use std::collections::BTreeMap;

pub use http::{self, Request, Response};
use http::{
    header::{HeaderMap, ACCEPT},
    uri::PathAndQuery,
    Method, Version,
};
use percent_encoding::{percent_encode, AsciiSet, NON_ALPHANUMERIC};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub type Body = Vec<u8>;
const URL_PERCENT_ENCODE_ASCII_SET: &AsciiSet = &NON_ALPHANUMERIC.remove(b':');

#[cfg(feature = "with-derive")]
mod macros;
#[cfg(feature = "with-derive")]
pub use macros::*;

pub trait Endpoint {
    type RequestBody: Serialize;
    type ResponseOkBody: DeserializeOwned;

    fn method(&self) -> Method;
    fn path(&self) -> String;
    fn query(&self) -> Option<BTreeMap<String, Vec<String>>> {
        None
    }
    fn header(&self) -> Option<HeaderMap> {
        None
    }
    fn body(&self) -> Option<Self::RequestBody> {
        None
    }

    fn render_request(&self) -> Result<Request<Body>, EndpointRenderRequestError> {
        let mut path_and_query: PathAndQuery = self.path().parse()?;

        if let Some(query) = &self.query() {
            let query = query
                .iter()
                .map(|x| {
                    x.1.iter()
                        .map(|v| {
                            format!(
                                "{}={}",
                                x.0,
                                percent_encode(v.as_bytes(), URL_PERCENT_ENCODE_ASCII_SET)
                                    .to_string()
                            )
                        })
                        .collect::<Vec<String>>()
                })
                .flatten()
                .collect::<Vec<String>>()
                .join("&");

            if let Some(query_exists) = path_and_query.query() {
                path_and_query =
                    format!("{}?{}&{}", path_and_query.path(), query_exists, query).parse()?
            } else {
                path_and_query = format!("{}?{}", path_and_query.path(), query).parse()?
            }
        }

        let mut request_builder = Request::builder()
            .method(self.method())
            .uri(path_and_query.as_str())
            .version(Version::HTTP_11)
            .header(ACCEPT, "application/json");

        if let Some(header) = &self.header() {
            request_builder = header
                .into_iter()
                .fold(request_builder, |builder, (k, v)| builder.header(k, v));
        }

        let body = if let Some(body) = self.body() {
            serde_json::to_vec(&body)?
        } else {
            vec![]
        };
        let request = request_builder.body(body)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_request() {
        struct MyEndpoint;
        impl Endpoint for MyEndpoint {
            type RequestBody = ();
            type ResponseOkBody = ();

            fn method(&self) -> Method {
                Method::GET
            }
            fn path(&self) -> String {
                "/v1/catalog/nodes".to_owned()
            }
            fn query(&self) -> Option<BTreeMap<String, Vec<String>>> {
                let map = vec![
                    ("dc".to_owned(), vec!["foo".to_owned()]),
                    (
                        "node-meta".to_owned(),
                        vec!["k1:v1".to_owned(), "k2:v2".to_owned()],
                    ),
                ]
                .into_iter()
                .collect();
                Some(map)
            }
            fn header(&self) -> Option<HeaderMap> {
                let mut map = HeaderMap::new();
                map.insert("X-Consul-Token", "xxx".parse().unwrap());
                Some(map)
            }
        }

        let my_endpoint = MyEndpoint;
        let req = my_endpoint.render_request().unwrap();

        assert_eq!(req.method(), Method::GET);
        assert_eq!(
            req.uri(),
            "/v1/catalog/nodes?dc=foo&node-meta=k1:v1&node-meta=k2:v2"
        );
        assert_eq!(req.version(), Version::HTTP_11);
        assert_eq!(req.headers().get("Accept").unwrap(), &"application/json");
        assert_eq!(req.headers().get("X-Consul-Token").unwrap(), &"xxx");
        assert_eq!(req.body(), b"");
    }
}
