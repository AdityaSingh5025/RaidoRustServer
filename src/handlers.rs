use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::models::{
    City,
    SearchCityQuery,
    Trip,
    SearchTripsQuery,
};

// --------------------------------------------------
// SEARCH CITIES
// --------------------------------------------------
pub async fn search_cities(
    State(pool): State<PgPool>,
    Query(params): Query<SearchCityQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let q = params.q.unwrap_or_default().trim().to_string();

    if q.is_empty() {
        return Ok(Json(json!({ "success": true, "data": [] })));
    }

    let cities = sqlx::query_as::<_, City>(
        r#"
        SELECT *
        FROM "City"
        WHERE name ILIKE $1
        ORDER BY name ASC
        LIMIT 10
        "#
    )
    .bind(format!("%{}%", q))
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "success": true, "data": cities })))
}

// --------------------------------------------------
// GET ALL CITIES
// --------------------------------------------------
pub async fn get_all_cities(
    State(pool): State<PgPool>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let cities = sqlx::query_as::<_, City>(
        r#"
        SELECT *
        FROM "City"
        ORDER BY name ASC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "success": true, "data": cities })))
}

// --------------------------------------------------
// GET TRIP BY ID (404 if not found)
// --------------------------------------------------
pub async fn get_trip_handler(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let trip = sqlx::query_as::<_, Trip>(
        r#"
        SELECT *
        FROM "Trip"
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match trip {
        Some(t) => Ok(Json(json!({ "success": true, "data": t }))),
        None => Err((StatusCode::NOT_FOUND, "Trip not found".into())),
    }
}

// --------------------------------------------------
// SEARCH TRIPS (MATCHES NODE RESPONSE SEMANTICS)
// --------------------------------------------------
pub async fn search_trips(
    State(pool): State<PgPool>,
    Query(params): Query<SearchTripsQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {

    let date = NaiveDate::parse_from_str(&params.travel_date, "%Y-%m-%d")
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid date format".into()))?;

    let rows = sqlx::query!(
        r#"
        SELECT
          t.id                         AS trip_id,
          t."driverId",
          t."vehicleId",
          t."routeId",
          t."travelDate",
          t."departureTime",
          t."totalSeats",
          t."availableSeats",
          t."isActive",
          t."createdAt",
          t."updatedAt",

          r."sourceCityId",
          r."destinationCityId",
          r.checkpoints,

          sc.id   AS source_city_id,
          sc.name AS source_city_name,

          dc.id   AS destination_city_id,
          dc.name AS destination_city_name,

          v.id            AS vehicle_id,
          v."numberPlate",
          v."totalSeats"  AS vehicle_total_seats,
          v.type          AS vehicle_type,

          d.id        AS driver_id,
          d."userId",
          d."isActive" AS driver_active,

          u.id    AS user_id,
          u.name,
          u.phone,
          u.role,
          u."languagePreference"

        FROM "Trip" t
        JOIN "Route"   r ON r.id = t."routeId"
        JOIN "City"    sc ON sc.id = r."sourceCityId"
        JOIN "City"    dc ON dc.id = r."destinationCityId"
        JOIN "Vehicle" v ON v.id = t."vehicleId"
        JOIN "Driver"  d ON d.id = t."driverId"
        JOIN "User"    u ON u.id = d."userId"

        WHERE
          t."isActive" = true
          AND r."sourceCityId" = $1
          AND r."destinationCityId" = $2
          AND DATE(t."travelDate") = $3

        ORDER BY t."departureTime" ASC
        "#,
        params.from_city_id,
        params.to_city_id,
        date
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let data = rows.into_iter().map(|r| {
        json!({
          "id": r.trip_id,
          "driverId": r.driverId,
          "vehicleId": r.vehicleId,
          "routeId": r.routeId,
          "travelDate": r.travelDate,
          "departureTime": r.departureTime,
          "totalSeats": r.totalSeats,
          "availableSeats": r.availableSeats,
          "isActive": r.isActive,
          "createdAt": r.createdAt,
          "updatedAt": r.updatedAt,
          "boardingCityId": r.sourceCityId,
          "destinationCityId": r.destinationCityId,

          "route": {
            "id": r.routeId,
            "checkpoints": r.checkpoints,
            "sourceCityId": r.sourceCityId,
            "destinationCityId": r.destinationCityId,
            "sourceCity": {
              "id": r.source_city_id,
              "name": r.source_city_name
            },
            "destinationCity": {
              "id": r.destination_city_id,
              "name": r.destination_city_name
            }
          },

          "vehicle": {
            "id": r.vehicle_id,
            "type": r.vehicle_type,
            "totalSeats": r.vehicle_total_seats,
            "numberPlate": r.numberPlate,
            "driver": {
              "id": r.driver_id,
              "userId": r.userId,
              "isActive": r.driver_active,
              "user": {
                "id": r.user_id,
                "name": r.name,
                "phone": r.phone,
                "role": r.role,
                "languagePreference": r.languagePreference
              }
            }
          },

          "driver": {
            "id": r.driver_id,
            "userId": r.userId,
            "isActive": r.driver_active,
            "user": {
              "id": r.user_id,
              "name": r.name,
              "phone": r.phone,
              "role": r.role,
              "languagePreference": r.languagePreference
            }
          }
        })
    });

    Ok(Json(json!({ "success": true, "data": data })))
}
