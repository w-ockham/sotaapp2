use chrono::{DateTime, Utc};

pub struct Alert {
    pub alert_id: i32,
    pub user_id: i32,
    pub reference: String,
    pub reference_detail: String,
    pub location: String,
    pub activator: String,
    pub activator_name: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub frequencies: String,
    pub comment: Option<String>,
    pub poster: Option<String>,
}

pub struct Spot {
    pub spot_id: i32,
    pub reference: String,
    pub reference_detail: String,
    pub activator: String,
    pub activator_name: Option<String>,
    pub spot_time: DateTime<Utc>,
    pub frequency: String,
    pub mode: String,
    pub spotter: String,
    pub comment: Option<String>,
}