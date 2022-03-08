use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::pet::state::{PetType, Stage};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    GiveWater {},
    // TODO: Finish
    //Feed {},
    //Pet {},
    // TODO: Breed
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetPetStatus {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PetResponse {
    pub name: String,
    pub pet_type: PetType,
    pub stage: Stage,
    pub last_watering_time: SystemTime,
    pub last_feeding_time: SystemTime,
    pub birth_date: SystemTime,
}
