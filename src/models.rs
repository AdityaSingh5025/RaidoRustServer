use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, FromRow)]
pub struct City {
    pub id: String, // using String based on introspection (text)
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchCityQuery {
    pub q: Option<String>,
}

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
    // Introspection showed these exist
    #[sqlx(rename = "createdAt")]
    pub created_at: Option<NaiveDateTime>,
    #[sqlx(rename = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct SearchTripsQuery {
    #[serde(rename = "from")]
    pub from_location: Option<String>, 
    #[serde(rename = "to")]
    pub to_location: Option<String>,
    #[serde(rename = "date")]
    pub travel_date: Option<String>, // Expecting ISO string or similar
}
