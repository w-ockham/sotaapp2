use anyhow::Result;
use chrono::{NaiveDateTime, TimeZone, Utc};
use domain::model::common::activation::Spot;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SOTASpot {
    pub id: i32,
    #[serde(rename = "userID")]
    pub user_id: i32,
    pub time_stamp: String,
    pub comments: Option<String>,
    pub callsign: String,
    pub association_code: String,
    pub summit_code: String,
    pub activator_callsign: String,
    pub activator_name: String,
    pub frequency: String,
    pub mode: String,
    pub summit_details: String,
    pub highlight_color: Option<String>,
}

impl From<SOTASpot> for Result<Spot> {
    fn from(s: SOTASpot) -> Self {
        let naive = NaiveDateTime::parse_from_str(&s.time_stamp, "%Y-%m-%dT%H:%M:%S")?;
        let spot_time = Utc.from_local_datetime(&naive).unwrap();
        Ok(Spot {
            spot_id: s.id,
            reference: s.summit_code,
            reference_detail: s.summit_details,
            activator: s.activator_callsign,
            activator_name: Some(s.activator_name),
            spot_time,
            frequency: s.frequency,
            mode: s.mode,
            spotter: s.callsign,
            comment: s.comments,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct POTASpot {
    pub spot_id: i32,
    pub activator: String,
    pub frequency: String,
    pub mode: String,
    pub reference: String,
    pub park_name: Option<String>,
    pub spot_time: String,
    pub spotter: String,
    pub comments: Option<String>,
    pub source: String,
    pub invalid: Option<String>,
    pub name: String,
    pub location_desc: String,
    pub grid4: String,
    pub grid6: String,
    pub latitude: f64,
    pub longitude: f64,
    pub count: i32,
    pub expire: i32,
}

impl From<POTASpot> for Result<Spot> {
    fn from(s: POTASpot) -> Self {
        let naive = NaiveDateTime::parse_from_str(&s.spot_time, "%Y-%m-%dT%H:%M:%S")?;
        let spot_time = Utc.from_local_datetime(&naive).unwrap();
        Ok(Spot {
            spot_id: s.spot_id,
            reference: s.reference,
            reference_detail: s.name,
            activator: s.activator,
            activator_name: None,
            spot_time,
            frequency: s.frequency,
            mode: s.mode,
            spotter: s.spotter,
            comment: s.comments,
        })
    }
}
