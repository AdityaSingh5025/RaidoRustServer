use axum::{
    routing::get,
    Router,
};
use sqlx::PgPool;

use crate::handlers::{
    get_all_cities,
    get_trip_handler,
    search_cities,
    search_trips,
};

pub fn create_router() -> Router<PgPool> {
    Router::new()
        // Cities
        .route("/cities/search", get(search_cities))
        .route("/cities", get(get_all_cities))

        // Trips
        // IMPORTANT: search must come before :id
        .route("/trips/search", get(search_trips))
        .route("/trips/:id", get(get_trip_handler))
}
