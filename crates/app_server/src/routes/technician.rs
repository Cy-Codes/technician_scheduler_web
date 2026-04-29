use app_core::{CreateTechnician, Technician};

use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{http::StatusCode, Json, Router};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(fetch_all).post(create))
    //.route("/{id}", get(get_one).put(update).delete(remove))
}

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateTechnician>,
) -> impl IntoResponse {
    match sqlx::query_as!(
        Technician,
        "INSERT INTO technicians(name) VALUES ($1) RETURNING *",
        payload.name,
    )
    .fetch_one(&state.pool)
    .await
    {
        Ok(technician) => {
            (StatusCode::CREATED, Json(technician)).into_response()
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

async fn fetch_all(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query_as!(Technician, "SELECT * FROM technicians")
        .fetch_all(&state.pool)
        .await
    {
        Ok(technicians) => (StatusCode::OK, Json(technicians)).into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::routes::technician::{create, fetch_all};
    use crate::{parse_response, AppState};
    use app_core::{CreateTechnician, Technician};
    use axum::body::to_bytes;
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::Json;
    use leptos::config::LeptosOptions;
    use sqlx::PgPool;

    async fn test_state(pool: PgPool) -> AppState {
        AppState {
            pool,
            leptos_options: LeptosOptions::builder().output_name("test").build(),
        }
    }
    // sqlx::test spins up a fresh isolated db, runs your migrations,
    // then tears it down after the test. pool is injected automatically.
    #[sqlx::test(migrations = "./migrations")]
    async fn test_fetch_all_empty_returns_ok(pool: PgPool) {
        let state = test_state(pool).await;
        let response = fetch_all(State(state)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
        let technicians = parse_response!(response, Vec<Technician>);
        assert_eq!(true, technicians.is_empty());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_fetch_all_returns_technicians(pool: PgPool) {
        let state = test_state(pool).await;

        // seed the isolated db with known data
        sqlx::query("INSERT INTO technicians (name) VALUES ($1)")
            .bind("Alice".to_string())
            .execute(&state.pool)
            .await
            .unwrap();

        let response = fetch_all(State(state)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
        let technicians = parse_response!(response, Vec<Technician>);
        assert_eq!(technicians.len(), 1);
        assert_eq!(technicians.first().unwrap().name, "Alice")
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_technician(pool: PgPool) {
        let state = test_state(pool).await;

        let payload = CreateTechnician {
            name: "TestTech".to_string(),
        };

        let response = create(State(state), Json(payload)).await.into_response();
        assert_eq!(StatusCode::CREATED, response.status());
        let technician = parse_response!(response, Technician);
        assert_eq!(technician.name, "TestTech");
    }
}
