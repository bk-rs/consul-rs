#[macro_export]
macro_rules! endpoint {
    (
        $name:ident,
        $res_ok_body:ty,
        $method:expr,
        $path:literal $(,)?
    ) => {
        paste! {
            #[derive(Debug, Clone)]
            pub struct [<$name Endpoint>] {
            }
            impl [<$name Endpoint>] {
                pub fn new() -> Self {
                    Self {}
                }
            }
            impl Default for  [<$name Endpoint>] {
                fn default() -> Self {
                    Self::new()
                }
            }
            impl crate::endpoint::Endpoint for [<$name Endpoint>] {
                type ResponseOkBody = $res_ok_body;

                fn method(&self) -> crate::endpoint::http::Method {
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
        $res_ok_body:ty,
        $method:expr,
        $path:literal,
        $( $path_param_k:ident = $path_param_v:ty ),+ $(,)?
    ) => {
        paste! {
            #[derive(Debug, Clone)]
            pub struct [<$name Endpoint>] {
                $( $path_param_k: $path_param_v,)*
            }
            impl [<$name Endpoint>] {
                pub fn new($( $path_param_k: $path_param_v,)*) -> Self {
                    Self {$( $path_param_k,)*}
                }
            }
            impl crate::endpoint::Endpoint for [<$name Endpoint>] {
                type ResponseOkBody = $res_ok_body;

                fn method(&self) -> crate::endpoint::http::Method {
                    $method
                }

                fn path(&self) -> String {
                    format!($path, $( $path_param_k = self.$path_param_k ,)*)
                }
            }
        }
    };
}
