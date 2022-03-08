#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, PetResponse, QueryMsg};
use crate::pet::pet::{Pet, PETS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-pets";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // TODO: Validate `msg.name`

    let pet = Pet::new(info.sender.clone(), msg.name);
    PETS.save(deps.storage, &pet)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("pet_name", pet.name))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::GiveWater {} => try_water(deps, info),
    }
}

pub fn try_water(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    PETS.update(deps.storage, |mut pet| -> Result<_, ContractError> {
        if info.sender != pet.owner {
            return Err(ContractError::Unauthorized {});
        }
        pet.water();
        Ok(pet)
    })?;

    Ok(Response::new().add_attribute("method", "try_water"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPetStatus {} => to_binary(&query_pet_status(deps)?),
    }
}

fn query_pet_status(deps: Deps) -> StdResult<PetResponse> {
    let pet = PETS.load(deps.storage)?;
    Ok(PetResponse {
        name: pet.name,
        pet_type: pet.pet_type,
        stage: pet.stage,
        last_feeding_time: pet.last_feeding_time,
        last_watering_time: pet.last_watering_time,
        birth_date: pet.birth_date,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pet::pet::{PetType, Stage};
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            name: "peepo".to_string(),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPetStatus {}).unwrap();
        let pet: PetResponse = from_binary(&res).unwrap();
        assert_eq!("peepo", pet.name);
        assert_eq!(Stage::Egg, pet.stage);
        assert_eq!(PetType::Fire, pet.pet_type);
    }

    #[test]
    fn test_water() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            name: "peepo".to_string(),
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // non-owner cannot water pet:
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::GiveWater {};
        let res = execute(deps.as_mut(), mock_env(), info, msg).err();
        assert_eq!(Some(ContractError::Unauthorized {}), res);

        // owner can water pet:
        let info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::GiveWater {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // pet.last_watered_time should be recent now:
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPetStatus {}).unwrap();
        let pet: PetResponse = from_binary(&res).unwrap();
        assert_eq!(pet.birth_date, pet.last_feeding_time);
        assert_ne!(pet.birth_date, pet.last_watering_time);
        assert_eq!(true, pet.birth_date < pet.last_watering_time);
    }
}
