use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use sqlx::PgPool;
use crate::models::{City, SearchCityQuery, Trip, SearchTripsQuery};

// searchCities equivalent
pub async fn search_cities(
    State(pool): State<PgPool>,
    Query(params): Query<SearchCityQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = params.q.unwrap_or_default();

    if query.trim().is_empty() {
        return Ok(Json(json!({
            "success": true,
            "data": []
        })));
    }

    let cities = sqlx::query_as::<_, City>(
        "SELECT * FROM \"City\" 
         WHERE name ILIKE $1 
         ORDER BY name ASC 
         LIMIT 10"
    )
    .bind(format!("%{}%", query))
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "success": true,
        "data": cities
    })))
}

// getAllCities equivalent
pub async fn get_all_cities(
    State(pool): State<PgPool>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let cities = sqlx::query_as::<_, City>(
        "SELECT * FROM \"City\" ORDER BY name ASC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "success": true,
        "data": cities
    })))
}

// getTripHandler equivalent
pub async fn get_trip_handler(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let trip = sqlx::query_as::<_, Trip>(
        "SELECT * FROM \"Trip\" WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match trip {
        Some(t) => Ok(Json(json!({
            "success": true,
            "data": t
        }))),
        None => Err((StatusCode::NOT_FOUND, "Trip not found".to_string())),
    }
}

pub async fn search_trips(
    State(pool): State<PgPool>,
    Query(params): Query<SearchTripsQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Basic implementation: returning recent active trips.
    // NOTE: Real implementation would join with Route/City tables to filter by from/to.
    
    let sql = "SELECT * FROM \"Trip\" WHERE \"isActive\" = true".to_string();
    
    if let Some(_date) = &params.travel_date {
        // Very basic date filter - assuming exact match or ignoring for safety if format is complex
        // In a real app, parse the date and use strict comparison
    }

    let trips = sqlx::query_as::<_, Trip>(&sql)
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "success": true,
        "data": trips
    })))
}

// aaaa