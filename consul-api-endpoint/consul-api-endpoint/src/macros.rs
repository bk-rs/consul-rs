#[macro_export]
macro_rules! query_endpoint {
    (
        $name:ident,
        $path:literal,
        $response_ok_body:ty $(,)?
    ) => {
        consul_api_endpoint_macro::endpoint!(
            name = $name,
            path = $path,
            response_ok_body = $response_ok_body,
        );
    };

    (
        $name:ident,
        $path:literal,
        { $( $query_option_name:ident ),+ $(,)? },
        $response_ok_body:ty $(,)?
    ) => {
        consul_api_endpoint_macro::endpoint!(
            name = $name,
            path = $path,
            query_option_names = $( $query_option_name ),* , ,
            response_ok_body = $response_ok_body,
        );
    };

    (
        $name:ident,
        $path:literal,
        { $( $path_param_name:ident => $path_param_type:ty ),+ $(,)? },
        { $( $query_option_name:ident ),+ $(,)? },
        $response_ok_body:ty $(,)?
    ) => {
        consul_api_endpoint_macro::endpoint!(
            name = $name,
            path = $path,
            path_params = $( $path_param_name => $path_param_type ),* , ,
            query_option_names = $( $query_option_name ),* , ,
            response_ok_body = $response_ok_body,
        );
    };
}
