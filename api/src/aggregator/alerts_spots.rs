use anyhow::Result;
use common::config::AppConfig;
use reqwest;

use crate::model::alerts::{POTAAlert, SOTAAlert};
use crate::model::spots::{POTASpot, SOTASpot};
use domain::model::common::activation::{Alert, Spot};
use domain::model::common::event::UpdateAct;

use registry::{AppRegistry, AppState};
use service::services::AdminPeriodicService;
use shaku::HasComponent;
use std::sync::Arc;

#[derive(Clone)]
pub struct UpdateAlerts {
    config: AppConfig,
    registry: Arc<AppRegistry>,
}

impl UpdateAlerts {
    pub fn new(config: &AppConfig, state: &AppState) -> Self {
        Self {
            config: config.clone(),
            registry: state.into(),
        }
    }
    pub async fn update(&self) -> Result<()> {
        self.update_sota_alerts().await?;
        self.update_pota_alerts().await?;
        Ok(())
    }

    async fn update_sota_alerts(&self) -> Result<()> {
        let service: &dyn AdminPeriodicService = self.registry.resolve_ref();
        let endpoint = self.config.sota_alert_endpoint.clone();

        let response = reqwest::get(&endpoint)
            .await?
            .json::<Vec<SOTAAlert>>()
            .await?;

        let requests: Vec<Alert> = response
            .into_iter()
            .filter_map(|sa| Result::<Alert>::from(sa).ok())
            .collect();

        let event = UpdateAct { requests };
        service.update_sota_alert(event).await?;

        Ok(())
    }

    async fn update_pota_alerts(&self) -> Result<()> {
        let service: &dyn AdminPeriodicService = self.registry.resolve_ref();
        let endpoint = self.config.pota_alert_endpoint.clone();

        let response = reqwest::get(&endpoint)
            .await?
            .json::<Vec<POTAAlert>>()
            .await?;

        let requests: Vec<Alert> = response
            .into_iter()
            .filter_map(|pa| Result::<Alert>::from(pa).ok())
            .collect();

        let event = UpdateAct { requests };
        service.update_pota_alert(event).await?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct UpdateSpots {
    config: AppConfig,
    registry: Arc<AppRegistry>,
}

impl UpdateSpots {
    pub fn new(config: &AppConfig, state: &AppState) -> Self {
        Self {
            config: config.clone(),
            registry: state.into(),
        }
    }
    pub async fn update(&self) -> Result<()> {
        self.update_sota_spots().await?;
        self.update_pota_spots().await?;
        Ok(())
    }

    async fn update_sota_spots(&self) -> Result<()> {
        let service: &dyn AdminPeriodicService = self.registry.resolve_ref();
        let endpoint = self.config.sota_spot_endpoint.clone();

        let response = reqwest::get(&endpoint)
            .await?
            .json::<Vec<SOTASpot>>()
            .await?;

        let requests: Vec<Spot> = response
            .into_iter()
            .filter_map(|ss| Result::<Spot>::from(ss).ok())
            .collect();

        let event = UpdateAct { requests };
        service.update_sota_spot(event).await?;
        Ok(())
    }

    async fn update_pota_spots(&self) -> Result<()> {
        let service: &dyn AdminPeriodicService = self.registry.resolve_ref();
        let endpoint = self.config.pota_spot_endpoint.clone();

        let response = reqwest::get(&endpoint)
            .await?
            .json::<Vec<POTASpot>>()
            .await?;

        let requests: Vec<Spot> = response
            .into_iter()
            .filter_map(|ss| Result::<Spot>::from(ss).ok())
            .collect();

        let event = UpdateAct { requests };
        service.update_pota_spot(event).await?;
        Ok(())
    }
}
