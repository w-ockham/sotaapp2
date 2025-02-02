use anyhow::Result;
use chrono::Duration;

#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub migration_path: String,
    pub cors_origin: Option<String>,
    pub log_level: String,
    pub sota_alert_endpoint: String,
    pub sota_spot_endpoint: String,
    pub pota_alert_endpoint: String,
    pub pota_spot_endpoint: String,
    pub sota_summitlist_endpoint: String,
    pub sota_summitlist_update_schedule: String,
    pub import_all_at_startup: bool,
    pub geomag_endpoint: String,
    pub geomag_update_schedule: String,
    pub mapcode_endpoint: String,
    pub alert_update_schedule: String,
    pub alert_expire: Duration,
    pub spot_update_schedule: String,
    pub spot_expire: Duration,
    pub pota_log_expire: Duration,
    pub aprs_host: String,
    pub aprs_user: String,
    pub aprs_password: String,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        Ok(Self {
            host: std::env::var("HOST")?,
            port: std::env::var("PORT")?.parse::<u16>()?,
            log_level: std::env::var("LOG_LEVEL")?,
            database: std::env::var("DATABASE_URL")?,
            migration_path: std::env::var("MIGRATION_PATH")?,
            cors_origin: std::env::var("CORS_ORIGIN").ok(),
            sota_alert_endpoint: std::env::var("SOTA_ALERT_ENDPOINT")?,
            sota_spot_endpoint: std::env::var("SOTA_SPOT_ENDPOINT")?,
            sota_summitlist_endpoint: std::env::var("SOTA_SUMMITLIST_ENDPOINT")?,
            sota_summitlist_update_schedule: std::env::var("SUMMITLIST_SCHEDULE")?,
            import_all_at_startup: std::env::var("IMPORT_STARTUP")?.parse::<bool>()?,
            pota_alert_endpoint: std::env::var("POTA_ALERT_ENDPOINT")?,
            pota_spot_endpoint: std::env::var("POTA_SPOT_ENDPOINT")?,
            geomag_endpoint: std::env::var("GEOMAG_ENDPOINT")?,
            geomag_update_schedule: std::env::var("GEOMAG_SCHEDULE")?,
            mapcode_endpoint: std::env::var("MAPCODE_ENDPOINT")?,
            alert_update_schedule: std::env::var("ALERT_SCHEDULE")?,
            spot_update_schedule: std::env::var("SPOT_SCHEDULE")?,
            alert_expire: Duration::hours(std::env::var("ALERT_EXPIRE")?.parse::<i64>()?),
            spot_expire: Duration::hours(std::env::var("SPOT_EXPIRE")?.parse::<i64>()?),
            pota_log_expire: Duration::days(std::env::var("POTA_LOG_EXPIRE")?.parse::<i64>()?),
            aprs_host: std::env::var("APRSHOST")?,
            aprs_user: std::env::var("APRSUSER")?,
            aprs_password: std::env::var("APRSPASSWORD")?,
        })
    }
}
