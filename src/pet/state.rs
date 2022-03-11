use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Copy)]
pub enum PetType {
    Water,
    Fire,
    Grass,
    Air,
    Ground,
    Space,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Stage {
    Egg,
    Baby,
    Adult,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pet {
    pub owner: Addr,
    pub name: String,
    pub pet_type: PetType,
    pub stage: Stage,
    pub last_watering_time: Timestamp,
    pub last_feeding_time: Timestamp,
    pub birth_date: Timestamp,
}

impl Pet {
    pub fn new(owner: Addr, name: String, height: u64, now: Timestamp) -> Pet {
        let options = [
            PetType::Water,
            PetType::Fire,
            PetType::Grass,
            PetType::Air,
            PetType::Ground,
            PetType::Space,
        ];

        Pet {
            owner,
            name,
            pet_type: options[height as usize % options.len()],
            stage: Stage::Egg,
            last_watering_time: now,
            last_feeding_time: now,
            birth_date: now,
        }
    }

    pub fn water(&mut self, now: Timestamp) {
        self.last_watering_time = now;
    }

    pub fn feed(&mut self, now: Timestamp) {
        self.last_feeding_time = now;
    }
}

pub const PETS: Item<Pet> = Item::new("pet");
