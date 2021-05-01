#[macro_export]
macro_rules! endpoint {
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
                params: Vec<(String, String)>,
            }
            pub type $alias = [<$name Endpoint>];
            impl [<$name Endpoint>] {
                pub fn new() -> Self {
                    Self { params: vec![] }
                }

                $(
                    pub fn [<set_ $param_k>](&mut self, val: $param_type) {
                        self.params.push((stringify!($param_k).to_string(), val.to_string()));
                    }
                )*
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
                params: Vec<(String, String)>,
            }
            pub type $alias = [<$name Endpoint>];
            impl [<$name Endpoint>] {
                pub fn new($( $path_param_k: $path_param_v,)*) -> Self {
                    Self {$( $path_param_k,)* params: vec![]}
                }

                $(
                    pub fn [<set_ $param_k>](&mut self, val: $param_type) {
                        self.params.push((stringify!($param_k).to_string(), val.to_string()));
                    }
                )*
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
