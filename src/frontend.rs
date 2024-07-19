use crate::client_features::Payload;
use serde::{Deserialize, Serialize};

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct FrontendResult {
    pub toggles: Vec<EvaluatedToggle>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct EvaluatedToggle {
    pub name: String,
    pub enabled: bool,
    pub variant: EvaluatedVariant,
    #[serde(rename = "impression_data")]
    pub impression_data: bool,
    pub impressionData: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct EvaluatedVariant {
    pub name: String,
    pub enabled: bool,
    pub payload: Option<Payload>,
}
