use crate::client_features::Payload;
use serde::{Deserialize, Serialize};

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct FrontendResult {
    pub toggles: Vec<EvaluatedToggle>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct EvaluatedToggle {
    pub name: String,
    pub enabled: bool,
    pub variant: EvaluatedVariant,
    pub impression_data: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct EvaluatedVariant {
    pub name: String,
    pub enabled: bool,
    pub payload: Option<Payload>,
}
