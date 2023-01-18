use crate::client_features::Payload;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FrontendResult {
    pub toggles: Vec<EvaluatedToggle>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluatedToggle {
    pub name: String,
    pub enabled: bool,
    pub variant: EvaluatedVariant,
    pub impression_data: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluatedVariant {
    pub name: String,
    pub enabled: bool,
    pub payload: Option<Payload>,
}
