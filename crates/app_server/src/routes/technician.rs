use crate::AppState;
use app_core::Technician;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{http::StatusCode, Json, Router};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(fetch_all)) //.post(create))
                                             //.route("/{id}", get(get_one).put(update).delete(remove))
}

// async fn create(
//     State(state): State<AppState>,
//     Json(payload): Json<Technician>,
// ) -> impl IntoResponse {
//     match sqlx::query_as("") {
//         Ok(_) => (StatusCode::OK).into_response(),
//         Err(_) => StatusCode::NOT_FOUND.into_response(),
//     }
// }

async fn fetch_all(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query_as!(Technician, "SELECT * FROM technicians")
        .fetch_all(&state.pool)
        .await
    {
        Ok(technicians) => (StatusCode::OK, Json(technicians)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use crate::routes::technician::fetch_all;
    use crate::AppState;
    use app_core::Technician;
    use axum::body::to_bytes;
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::response::Response;
    use leptos::config::LeptosOptions;
    use sqlx::PgPool;

    pub async fn parse_response<T: serde::de::DeserializeOwned>(response: Response) -> T {
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&body).unwrap()
    }
    // sqlx::test spins up a fresh isolated db, runs your migrations,
    // then tears it down after the test. pool is injected automatically.
    #[sqlx::test(migrations = "./migrations")]
    async fn test_fetch_all_empty_returns_ok(pool: PgPool) {
        let state = AppState {
            pool,
            leptos_options: LeptosOptions::builder().output_name("test").build(),
        };
        let response = fetch_all(State(state)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
        let technicians = parse_response::<Vec<Technician>>(response).await;

        assert_eq!(true, technicians.is_empty());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_fetch_all_returns_technicians(pool: PgPool) {
        let state = AppState {
            pool,
            leptos_options: LeptosOptions::builder().output_name("test").build(),
        };

        // seed the isolated db with known data
        sqlx::query("INSERT INTO technicians (name) VALUES ($1)")
            .bind("Alice")
            .execute(&state.pool)
            .await
            .unwrap();

        let response = fetch_all(State(state)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let technicians = parse_response::<Vec<Technician>>(response).await;

        println!("{:#?}", technicians);
        assert_eq!(1, technicians.len());
        assert_eq!("Alice", technicians.first().unwrap().name)
    }
}
