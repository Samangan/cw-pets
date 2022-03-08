// use rand::SeedableRng;
// use rand::{
//     distributions::{Distribution, Standard},
//     rngs::StdRng,
//     Rng,
// };
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum PetType {
    Water,
    Fire,
    Grass,
    Air,
    Ground,
    Space,
}

// impl Distribution<PetType> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PetType {
//         match rng.gen_range(0..=5) {
//             0 => PetType::Water,
//             1 => PetType::Fire,
//             2 => PetType::Grass,
//             3 => PetType::Air,
//             4 => PetType::Ground,
//             _ => PetType::Space,
//         }
//     }
// }

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
    seed: u64,
}

impl Pet {
    pub fn new(owner: Addr, name: String) -> Pet {
        let now = SystemTime::now();
        let seed = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        //let mut r = StdRng::seed_from_u64(seed);

        Pet {
            owner,
            name,
            seed,
            // TODO: How do I do RNG inside of the WASM runtime?
            // pet_type: r.gen(),
            pet_type: PetType::Air,
            stage: Stage::Egg,
            last_watering_time: now,
            last_feeding_time: now,
            birth_date: now,
        }
    }

    pub fn water(&mut self) {
        self.last_watering_time = SystemTime::now();
    }
}

pub const PETS: Item<Pet> = Item::new("pet");
