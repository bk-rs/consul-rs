#[macro_export]
macro_rules! endpoint {
    ($name:ident, $body:ty, $method:expr, $path:literal, $(,)?) => {
        paste! {
            #[derive(Debug, Clone)]
            pub struct [<$name Endpoint>];
            impl crate::endpoint::Endpoint for [<$name Endpoint>] {
                type ResponseOkBody = $body;

                fn method(&self) -> crate::endpoint::http::Method {
                    $method
                }

                fn path(&self) -> String {
                    $path.to_owned()
                }
            }
        }
    };
}
