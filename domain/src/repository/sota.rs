use async_trait::async_trait;
use common::error::AppResult;
use shaku::Interface;

use crate::model::common::event::{
    CreateRef, DeleteAct, DeleteRef, FindAct, FindRef, FindResult, UpdateAct, UpdateRef,
};
use crate::model::sota::{SOTAAlert, SOTARefOptInfo, SOTAReference, SOTASpot, SummitCode};

#[async_trait]
pub trait SOTAReferenceReposity: Send + Sync + Interface {
    async fn import_reference(&self, event: CreateRef<SOTAReference>) -> AppResult<()>;
    async fn find_reference(&self, event: &FindRef) -> AppResult<FindResult<SOTAReference>>;
    async fn update_reference_opt(&self, event: UpdateRef<SOTARefOptInfo>) -> AppResult<()>;
    async fn delete_reference_opt(&self, event: DeleteRef<SummitCode>) -> AppResult<()>;
}

#[async_trait]
pub trait SOTAActivationRepositry: Send + Sync + Interface {
    async fn update_alert(&self, event: UpdateAct<SOTAAlert>) -> AppResult<()>;
    async fn find_alert(&self, event: &FindAct) -> AppResult<FindResult<SOTAAlert>>;
    async fn delete_alert(&self, event: DeleteAct) -> AppResult<()>;
    async fn update_spot(&self, event: UpdateAct<SOTASpot>) -> AppResult<()>;
    async fn find_spot(&self, event: &FindAct) -> AppResult<FindResult<SOTASpot>>;
    async fn delete_spot(&self, event: DeleteAct) -> AppResult<()>;
}