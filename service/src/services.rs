use async_trait::async_trait;
use shaku::Interface;

use common::error::AppResult;
use domain::model::common::activation::{Alert, Spot};
use domain::model::common::event::{DeleteRef, FindAct, FindAppResult, FindRef, FindResult};
use domain::model::common::id::UserId;
use domain::model::locator::MunicipalityCenturyCode;
use domain::model::pota::{POTAReference, ParkCode};
use domain::model::sota::{SOTAReference, SummitCode};

use crate::model::locator::UploadMuniCSV;
use crate::model::pota::{UploadActivatorCSV, UploadHunterCSV, UploadPOTACSV};
use crate::model::sota::{UploadSOTACSV, UploadSOTAOptCSV};

#[async_trait]
pub trait UserService: Send + Sync + Interface {
    async fn find_references(&self, event: FindRef) -> AppResult<FindAppResult>;
    async fn find_alerts(&self, event: FindAct) -> AppResult<FindResult<Alert>>;
    async fn find_spots(&self, event: FindAct) -> AppResult<FindResult<Spot>>;
    async fn upload_activator_csv(
        &self,
        user_id: UserId,
        event: UploadActivatorCSV,
    ) -> AppResult<()>;
    async fn find_century_code(
        &self,
        muni_code: i32,
    ) -> AppResult<FindResult<MunicipalityCenturyCode>>;
    async fn find_mapcode(&self, lon: f64, lat: f64) -> AppResult<String>;
    async fn upload_hunter_csv(&self, user_id: UserId, event: UploadHunterCSV) -> AppResult<()>;
}

#[async_trait]
pub trait AdminService: Send + Sync + Interface {
    async fn import_summit_list(&self, event: UploadSOTACSV) -> AppResult<()>;
    async fn import_summit_opt_list(&self, event: UploadSOTAOptCSV) -> AppResult<()>;
    async fn import_pota_park_list(&self, event: UploadPOTACSV) -> AppResult<()>;
    async fn import_muni_century_list(&self, event: UploadMuniCSV) -> AppResult<()>;

    async fn find_sota_reference(&self, query: FindRef) -> AppResult<FindResult<SOTAReference>>;
    async fn update_sota_reference(&self, references: Vec<SOTAReference>) -> AppResult<()>;
    async fn delete_sota_reference(&self, query: DeleteRef<SummitCode>) -> AppResult<()>;

    async fn find_pota_reference(&self, query: FindRef) -> AppResult<FindResult<POTAReference>>;
    async fn update_pota_reference(&self, references: Vec<POTAReference>) -> AppResult<()>;
    async fn delete_pota_reference(&self, query: DeleteRef<ParkCode>) -> AppResult<()>;

    async fn health_check(&self) -> AppResult<bool>;
}

#[async_trait]
pub trait AdminPeriodicService: Send + Sync + Interface {
    async fn update_alerts(&self, alerts: Vec<Alert>) -> AppResult<()>;
    async fn update_spots(&self, spots: Vec<Spot>) -> AppResult<()>;
}
