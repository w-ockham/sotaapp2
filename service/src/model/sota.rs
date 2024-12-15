use domain::model::sota::SOTAReference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SOTACSVFile {
    pub summit_code: String,
    pub association_name: String,
    pub region_name: String,
    pub summit_name: String,
    pub alt_m: i32,
    pub alt_ft: i32,
    pub grid_ref1: String,
    pub grid_ref2: String,
    pub longitude: Option<f64>,
    pub lattitude: Option<f64>,
    pub points: i32,
    pub bonus_points: i32,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub activation_count: i32,
    pub activation_date: Option<String>,
    pub activation_call: Option<String>,
}

impl From<SOTACSVFile> for SOTAReference {
    fn from(csv: SOTACSVFile) -> SOTAReference {
        let SOTACSVFile {
            summit_code,
            association_name,
            region_name,
            summit_name,
            alt_m,
            alt_ft,
            grid_ref1,
            grid_ref2,
            longitude,
            lattitude,
            points,
            bonus_points,
            valid_from,
            valid_to,
            activation_count,
            activation_date,
            activation_call,
        } = csv;
        Self {
            summit_code,
            association_name,
            region_name,
            summit_name,
            summit_name_j: None,
            city: None,
            city_j: None,
            alt_m,
            alt_ft,
            grid_ref1,
            grid_ref2,
            longitude,
            lattitude,
            points,
            bonus_points,
            valid_from,
            valid_to,
            activation_count,
            activation_date,
            activation_call,
        }
    }
}