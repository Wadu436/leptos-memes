#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use axum::routing::post;
    use axum::{Router, Server};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_start::app::*;
    use std::sync::Arc;
    use tower_http::services::ServeDir;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    // Axum setup
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> })
        .fallback_service(ServeDir::new(leptos_options.site_root.clone()))
        .layer(axum::Extension(Arc::new(leptos_options)));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| Box::from(e))
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
