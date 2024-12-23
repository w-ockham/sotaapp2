use chrono::{DateTime, Utc};
use domain::model::common::activation::{Alert, Spot};
use domain::model::AwardProgram;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct AlertImpl {
    pub program: i32,
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
            start_time,
            end_time,
            frequencies,
            comment,
            poster,
        } = value;
        let program: i32 = match program {
            AwardProgram::SOTA => 0,
            AwardProgram::POTA => 1,
            AwardProgram::WWFF => 2,
        };
        Self {
            program,
            alert_id,
            user_id,
            reference,
            reference_detail,
            location,
            activator,
            activator_name,
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
            start_time,
            end_time,
            frequencies,
            comment,
            poster,
        } = value;
        let program = match program {
            0 => AwardProgram::SOTA,
            1 => AwardProgram::POTA,
            _ => AwardProgram::WWFF,
        };
        Self {
            program,
            alert_id,
            user_id,
            reference,
            reference_detail,
            location,
            activator,
            activator_name,
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
    pub program: i32,
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

impl From<Spot> for SpotImpl {
    fn from(value: Spot) -> Self {
        let Spot {
            program,
            spot_id,
            reference,
            reference_detail,
            activator,
            activator_name,
            spot_time,
            frequency,
            mode,
            spotter,
            comment,
        } = value;
        let program: i32 = match program {
            AwardProgram::SOTA => 0,
            AwardProgram::POTA => 1,
            AwardProgram::WWFF => 2,
        };

        Self {
            program,
            spot_id,
            reference,
            reference_detail,
            activator,
            activator_name,
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
            spot_time,
            frequency,
            mode,
            spotter,
            comment,
        } = value;
        let program = match program {
            0 => AwardProgram::SOTA,
            1 => AwardProgram::POTA,
            _ => AwardProgram::WWFF,
        };
        Self {
            program,
            spot_id,
            reference,
            reference_detail,
            activator,
            activator_name,
            spot_time,
            frequency,
            mode,
            spotter,
            comment,
        }
    }
}
