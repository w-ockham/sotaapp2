use axum::extract::FromRef;
use common::config::AppConfig;
use shaku::module;
use std::sync::{Arc, Mutex};

use adapter::{
    database::connect_database_with,
    implement::{
        activation::ActivationRepositryImplParameters, geomag::GeoMagRepositryImplParameters,
        healthcheck::HealthCheckRepositryImplParameters, locator::LocatorRepositryImplParameters,
        pota_reference::POTAReferenceRepositryImplParameters,
        sota_reference::SOTAReferenceReposityImplParameters,
    },
};

use service::implement::{
    admin_periodic::{AdminPeriodicServiceImpl, AdminPeriodicServiceImplParameters},
    admin_service::{AdminServiceImpl, AdminServiceImplParameters},
    user_service::{UserServiceImpl, UserServiceImplParameters},
};

use adapter::implement::{
    activation::ActivationRepositryImpl, geomag::GeoMagRepositryImpl,
    healthcheck::HealthCheckRepositryImpl, locator::LocatorRepositryImpl,
    pota_reference::POTAReferenceRepositryImpl, sota_reference::SOTAReferenceReposityImpl,
};

module! {
    pub AppRegistry {
        components = [UserServiceImpl, AdminServiceImpl, AdminPeriodicServiceImpl,
        SOTAReferenceReposityImpl,ActivationRepositryImpl,POTAReferenceRepositryImpl,
        LocatorRepositryImpl,GeoMagRepositryImpl,
        HealthCheckRepositryImpl],
        providers = [],
    }
}

impl AppRegistry {
    pub fn new(config: &AppConfig) -> Self {
        let pool = connect_database_with(config).unwrap();
        AppRegistry::builder()
            .with_component_parameters::<SOTAReferenceReposityImpl>(
                SOTAReferenceReposityImplParameters { pool: pool.clone() },
            )
            .with_component_parameters::<POTAReferenceRepositryImpl>(
                POTAReferenceRepositryImplParameters { pool: pool.clone() },
            )
            .with_component_parameters::<ActivationRepositryImpl>(
                ActivationRepositryImplParameters { pool: pool.clone() },
            )
            .with_component_parameters::<LocatorRepositryImpl>(LocatorRepositryImplParameters {
                config: config.clone(),
                pool: pool.clone(),
            })
            .with_component_parameters::<UserServiceImpl>(UserServiceImplParameters {
                config: config.clone(),
            })
            .with_component_parameters::<AdminServiceImpl>(AdminServiceImplParameters {
                config: config.clone(),
            })
            .with_component_parameters::<AdminPeriodicServiceImpl>(
                AdminPeriodicServiceImplParameters {
                    config: config.clone(),
                },
            )
            .with_component_parameters::<GeoMagRepositryImpl>(GeoMagRepositryImplParameters {
                latest_data: Arc::new(Mutex::new(None)),
            })
            .with_component_parameters::<HealthCheckRepositryImpl>(
                HealthCheckRepositryImplParameters { pool: pool.clone() },
            )
            .build()
    }
}

#[derive(Clone)]
pub struct AppState {
    module: Arc<AppRegistry>,
}

impl AppState {
    pub fn new(module: AppRegistry) -> Self {
        Self {
            module: Arc::new(module),
        }
    }
}

impl FromRef<AppState> for Arc<AppRegistry> {
    fn from_ref(app_state: &AppState) -> Arc<AppRegistry> {
        app_state.module.clone()
    }
}

impl From<&AppState> for Arc<AppRegistry> {
    fn from(app_state: &AppState) -> Arc<AppRegistry> {
        app_state.module.clone()
    }
}
