use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

// --------------------
// City
// --------------------
#[derive(Debug, Serialize, FromRow)]
pub struct City {
    pub id: String,
    pub name: String,
}

// --------------------
// Search city query
// --------------------
#[derive(Debug, Deserialize)]
pub struct SearchCityQuery {
    pub q: Option<String>,
}

// --------------------
// Trip (basic table mapping)
// --------------------
#[derive(Debug, Serialize, FromRow)]
pub struct Trip {
    pub id: String,

    #[sqlx(rename = "driverId")]
    pub driver_id: String,

    #[sqlx(rename = "vehicleId")]
    pub vehicle_id: String,

    #[sqlx(rename = "routeId")]
    pub route_id: String,

    #[sqlx(rename = "travelDate")]
    pub travel_date: Option<NaiveDateTime>,

    #[sqlx(rename = "departureTime")]
    pub departure_time: Option<String>,

    #[sqlx(rename = "totalSeats")]
    pub total_seats: Option<i32>,

    #[sqlx(rename = "availableSeats")]
    pub available_seats: Option<i32>,

    #[sqlx(rename = "isActive")]
    pub is_active: Option<bool>,

    #[sqlx(rename = "createdAt")]
    pub created_at: Option<NaiveDateTime>,

    #[sqlx(rename = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
}

// --------------------
// Search trips query
// MUST MATCH FRONTEND / NODE
// --------------------
#[derive(Debug, Deserialize)]
pub struct SearchTripsQuery {
    #[serde(rename = "fromCityId")]
    pub from_city_id: String,

    #[serde(rename = "toCityId")]
    pub to_city_id: String,

    #[serde(rename = "date")]
    pub travel_date: String, // YYYY-MM-DD
}
