use async_trait::async_trait;
use shaku::Interface;

use domain::model::common::activation::{Alert, Spot};
use domain::model::common::event::{
    DeleteRef, FindAct, FindAppResult, FindRef, FindResult, UpdateAct, UpdateRef,
};
use domain::model::pota::{POTAReference, ParkCode};
use domain::model::sota::{SOTAReference, SummitCode};

use crate::model::pota::UploadPOTACSV;
use crate::model::sota::{UploadSOTACSV, UploadSOTAOptCSV};
use common::error::AppResult;

#[async_trait]
pub trait UserService: Send + Sync + Interface {
    async fn find_reference(&self, event: FindRef) -> AppResult<FindAppResult>;
    async fn find_alert(&self, event: FindAct) -> AppResult<FindResult<Alert>>;
    async fn find_spot(&self, event: FindAct) -> AppResult<FindResult<Spot>>;
}

#[async_trait]
pub trait AdminService: Send + Sync + Interface {
    async fn import_summit_list(&self, event: UploadSOTACSV) -> AppResult<()>;
    async fn import_summit_opt_list(&self, event: UploadSOTAOptCSV) -> AppResult<()>;
    async fn import_pota_park_list(&self, event: UploadPOTACSV) -> AppResult<()>;

    async fn find_sota_reference(&self, event: FindRef) -> AppResult<FindResult<SOTAReference>>;
    async fn update_sota_reference(&self, event: UpdateRef<SOTAReference>) -> AppResult<()>;
    async fn delete_sota_reference(&self, event: DeleteRef<SummitCode>) -> AppResult<()>;

    async fn find_pota_reference(&self, event: FindRef) -> AppResult<FindResult<POTAReference>>;
    async fn update_pota_reference(&self, event: UpdateRef<POTAReference>) -> AppResult<()>;
    async fn delete_pota_reference(&self, event: DeleteRef<ParkCode>) -> AppResult<()>;

    async fn health_check(&self) -> AppResult<bool>;
}

#[async_trait]
pub trait AdminPeriodicService: Send + Sync + Interface {
    async fn update_alert(&self, event: UpdateAct<Alert>) -> AppResult<()>;
    async fn update_spot(&self, event: UpdateAct<Spot>) -> AppResult<()>;
}
