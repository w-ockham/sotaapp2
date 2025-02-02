use chrono::{DateTime, Utc};
use domain::model::common::activation::{Alert, Spot};
use domain::model::common::AwardProgram;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct AlertImpl {
    pub program: AwardProgram,
    pub alert_id: i32,
    pub user_id: i32,
    pub reference: String,
    pub reference_detail: String,
    pub location: String,
    pub activator: String,
    pub activator_name: Option<String>,
    pub operator: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub frequencies: String,
    pub comment: Option<String>,
    pub poster: Option<String>,
}

impl From<Alert> for AlertImpl {
    fn from(value: Alert) -> Self {
        let Alert {
            program,
            alert_id,
            user_id,
            reference,
            reference_detail,
            location,
            activator,
            activator_name,
            operator,
            start_time,
            end_time,
            frequencies,
            comment,
            poster,
        } = value;
        Self {
            program,
            alert_id,
            user_id,
            reference,
            reference_detail,
            location,
            activator,
            activator_name,
            operator,
            start_time,
            end_time,
            frequencies,
            comment,
            poster,
        }
    }
}

impl From<AlertImpl> for Alert {
    fn from(value: AlertImpl) -> Self {
        let AlertImpl {
            program,
            alert_id,
            user_id,
            reference,
            reference_detail,
            location,
            activator,
            activator_name,
            operator,
            start_time,
            end_time,
            frequencies,
            comment,
            poster,
        } = value;
        Self {
            program,
            alert_id,
            user_id,
            reference,
            reference_detail,
            location,
            activator,
            activator_name,
            operator,
            start_time,
            end_time,
            frequencies,
            comment,
            poster,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct SpotImpl {
    pub program: AwardProgram,
    pub spot_id: i32,
    pub reference: String,
    pub reference_detail: String,
    pub activator: String,
    pub activator_name: Option<String>,
    pub operator: String,
    pub spot_time: DateTime<Utc>,
    pub frequency: String,
    pub mode: String,
    pub spotter: String,
    pub comment: Option<String>,
}

impl From<Spot> for SpotImpl {
    fn from(value: Spot) -> Self {
        let Spot {
            program,
            spot_id,
            reference,
            reference_detail,
            activator,
            activator_name,
            operator,
            spot_time,
            frequency,
            mode,
            spotter,
            comment,
        } = value;
        Self {
            program,
            spot_id,
            reference,
            reference_detail,
            activator,
            activator_name,
            operator,
            spot_time,
            frequency,
            mode,
            spotter,
            comment,
        }
    }
}

impl From<SpotImpl> for Spot {
    fn from(value: SpotImpl) -> Self {
        let SpotImpl {
            program,
            spot_id,
            reference,
            reference_detail,
            activator,
            activator_name,
            operator,
            spot_time,
            frequency,
            mode,
            spotter,
            comment,
        } = value;
        Self {
            program,
            spot_id,
            reference,
            reference_detail,
            activator,
            activator_name,
            operator,
            spot_time,
            frequency,
            mode,
            spotter,
            comment,
        }
    }
}
