use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use cosmwasm_std::Addr;
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
    pub last_watering_time: SystemTime,
    pub last_feeding_time: SystemTime,
    pub birth_date: SystemTime,
}

impl Pet {
    pub fn new(owner: Addr, name: String) -> Pet {
        let now = SystemTime::now();
        let options = [
            PetType::Water,
            PetType::Fire,
            PetType::Grass,
            PetType::Air,
            PetType::Ground,
            PetType::Space,
        ];
        let idx = name.chars().take(1).last().unwrap().to_digit(32).unwrap() as usize;

        Pet {
            owner,
            name,
            pet_type: options[idx % options.len()],
            stage: Stage::Egg,
            last_watering_time: now,
            last_feeding_time: now,
            birth_date: now,
        }
    }

    pub fn water(&mut self) {
        self.last_watering_time = SystemTime::now();
    }

    pub fn feed(&mut self) {
        self.last_feeding_time = SystemTime::now();
    }
}

pub const PETS: Item<Pet> = Item::new("pet");
