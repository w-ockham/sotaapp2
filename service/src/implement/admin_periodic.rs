use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::config::AppConfig;
use shaku::Component;
use std::sync::Arc;

use common::error::AppResult;

use domain::model::common::activation::{Alert, Spot};
use domain::model::common::event::{DeleteAct, UpdateAct};
use domain::repository::activation::ActivationRepositry;

use crate::services::AdminPeriodicService;

#[derive(Component)]
#[shaku(interface = AdminPeriodicService)]
pub struct AdminPeriodicServiceImpl {
    #[shaku(inject)]
    act_repo: Arc<dyn ActivationRepositry>,
    config: AppConfig,
}

#[async_trait]
impl AdminPeriodicService for AdminPeriodicServiceImpl {
    async fn update_alerts(&self, event: UpdateAct<Alert>) -> AppResult<()> {
        self.act_repo.update_alerts(event).await?;

        let expire: DateTime<Utc> = Utc::now() - self.config.alert_expire;
        self.act_repo
            .delete_alerts(DeleteAct { before: expire })
            .await?;
        Ok(())
    }

    async fn update_spots(&self, event: UpdateAct<Spot>) -> AppResult<()> {
        self.act_repo.update_spots(event).await?;

        let expire: DateTime<Utc> = Utc::now() - self.config.alert_expire;
        self.act_repo
            .delete_spots(DeleteAct { before: expire })
            .await?;
        Ok(())
    }
}
