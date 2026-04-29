mod routes;

use crate::routes::technician;
use app_client::{shell, App};
use axum::Router;
use leptos::config::{get_configuration, LeptosOptions};
use leptos_axum::{generate_route_list, LeptosRoutes};
use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    pool: sqlx::PgPool,
    leptos_options: LeptosOptions,
}

impl axum::extract::FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

pub async fn start() {
    // ToDo: dotenvy().ok() when I get to prod as .env wil not be needed
    dotenvy::dotenv().expect("Missing .env file");
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migration failed");

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let state = AppState {
        pool,
        leptos_options: leptos_options.clone(),
    };

    let app = Router::new()
        .nest("/api/technician", technician::router())
        .leptos_routes(&state, routes, {
            let options = leptos_options.clone();
            move || shell(options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
