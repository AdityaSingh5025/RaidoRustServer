use axum::{
    routing::get,
    Router,
};
use sqlx::PgPool;
use crate::handlers::{get_all_cities, get_trip_handler, search_cities, search_trips};

pub fn create_router() -> Router<PgPool> {
    Router::new()
        .route("/search_cities", get(search_cities))
        .route("/cities", get(get_all_cities))
        .route("/trips/search", get(search_trips)) // /trips/search before /trips/{id} to avoid conflict
        .route("/trips/{id}", get(get_trip_handler))
}
