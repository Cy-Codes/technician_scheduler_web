use app_core::Technician;
use axum::routing::get;
use axum::{extract::Path, http::StatusCode, Json, Router};

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(get_one).put(update).delete(remove))
}

async fn list() -> Json<Vec<Technician>> {
    Json(vec![])
}

async fn get_one(Path(_id): Path<u32>) -> StatusCode {
    StatusCode::NOT_FOUND
}

async fn create(Json(_body): Json<Technician>) -> StatusCode {
    StatusCode::CREATED
}

async fn update(Path(_id): Path<u32>, Json(_body): Json<Technician>) -> StatusCode {
    StatusCode::OK
}

async fn remove(Path(_id): Path<u32>) -> StatusCode {
    StatusCode::NO_CONTENT
}
