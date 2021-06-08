#[macro_export]
macro_rules! endpoint {
    (
        $name:ident,
        $method:expr,
        $path:literal,
        $response_ok_body:ty $(,)?
    ) => {
        consul_api_endpoint_macro::endpoint!(
            name = $name,
            method = $method,
            path = $path,
            response_ok_body = $response_ok_body,
        );
    };

    (
        $name:ident,
        $alias:ident,
        $res_ok_body:ty,
        $method:expr,
        $path:literal,
        $(;)?
        $( $param_k:ident = $param_type:ty ),* $(,)?
    ) => {
        paste! {
            #[derive(Debug, Clone)]
            pub struct [<$name Endpoint>] {
                params: std::collections::HashMap<String, serde_json::Value>,
            }
            pub type $alias = [<$name Endpoint>];
            impl [<$name Endpoint>] {
                pub fn new() -> Self {
                    Self { params: Default::default() }
                }

                $(
                    pub fn [<set_ $param_k>](&mut self, val: $param_type) {
                        *self.params.entry(stringify!($param_k).to_string()).or_default() = val.into();
                    }
                )*
            }
            impl Default for  [<$name Endpoint>] {
                fn default() -> Self {
                    Self::new()
                }
            }
            impl consul_api_endpoint::Endpoint for [<$name Endpoint>] {
                type RequestBody = ();
                type ResponseOkBody = $res_ok_body;

                fn method(&self) -> ::http::Method {
                    $method
                }

                fn path(&self) -> String {
                    $path.to_owned()
                }
            }
        }
    };
    (
        $name:ident,
        $alias:ident,
        $res_ok_body:ty,
        $method:expr,
        $path:literal,
        $( $path_param_k:ident => $path_param_v:ty ),+ $(,)?
        ;
        $( $param_k:ident = $param_type:ty ),* $(,)?
    ) => {
        paste! {
            #[derive(Debug, Clone)]
            pub struct [<$name Endpoint>] {
                $( $path_param_k: $path_param_v,)*
                params: std::collections::HashMap<String, serde_json::Value>,
            }
            pub type $alias = [<$name Endpoint>];
            impl [<$name Endpoint>] {
                pub fn new($( $path_param_k: $path_param_v,)*) -> Self {
                    Self {$( $path_param_k,)* params: Default::default()}
                }

                $(
                    pub fn [<set_ $param_k>](&mut self, val: $param_type) {
                        *self.params.entry(stringify!($param_k).to_string()).or_default() = val.into();
                    }
                )*
            }
            impl consul_api_endpoint::Endpoint for [<$name Endpoint>] {
                type RequestBody = ();
                type ResponseOkBody = $res_ok_body;

                fn method(&self) -> ::http::Method {
                    $method
                }

                fn path(&self) -> String {
                    format!($path, $( $path_param_k = self.$path_param_k ,)*)
                }
            }
        }
    };
}
