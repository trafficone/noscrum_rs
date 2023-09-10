use cfg_if::cfg_if;
use leptos::*;
//{LeptosOptions, view, get_configuration};
 cfg_if! {
if #[cfg(feature = "ssr")] {
use lambda_http::{run, http::{Response, StatusCode}, Error};
use axum::{
    extract::{Path, State},
    Router,
    routing::get
};
use leptos_axum::{LeptosRoutes, generate_route_list};
type ServerError = (StatusCode, String);
async fn function_handler(State(_options): State<LeptosOptions>)-> 
    Result<String, ServerError> {
    // Prepare the response
    let resp = "Hello World!".to_string();
    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let routes = generate_route_list(|cx| view! {cx,  <App/> }).await;
    let leptos_options = get_configuration(Some("Cargo.toml")).await.unwrap().leptos_options;

    run(Router::new()
        .route("/",get(function_handler)
        .leptos_routes(&leptos_options, routes, |cx| view! { cx, <App/> })
        .with_state(leptos_options))
    ).await
}
}else {
    pub fn main() {
        println!{"Not implemented"}
    }
}
 }