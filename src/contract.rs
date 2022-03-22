#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::{get_contract_version, set_contract_version};
use semver::Version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, PetResponse, QueryMsg};
use crate::pet::state::{Pet, PETS};

// TODO: Refactor to use the production ready cw-20 base contract

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-pets";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // TODO: Validate `msg.name`

    let pet = Pet::new(
        info.sender.clone(),
        msg.name,
        env.block.height,
        env.block.time,
    );
    PETS.save(deps.storage, &pet)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("pet_name", pet.name))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::GiveWater {} => try_water(deps, env, info),
        ExecuteMsg::Feed {} => try_feed(deps, env, info),
    }
}

// TODO: Remove watering for now (Feeding can be made generic to feed items later if I want)
pub fn try_water(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PETS.update(deps.storage, |mut pet| -> Result<_, ContractError> {
        if info.sender != pet.owner {
            return Err(ContractError::Unauthorized {});
        }
        pet.water(env.block.time);
        Ok(pet)
    })?;

    Ok(Response::new().add_attribute("method", "try_water"))
}

pub fn try_feed(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PETS.update(deps.storage, |mut pet| -> Result<_, ContractError> {
        if info.sender != pet.owner {
            return Err(ContractError::Unauthorized {});
        }

        // TODO: After or before feeding check if it's time to evolve to the next pet.Stage:
        // * If the pet is happy enough (this can be based on pet.last_feeding_time and pet.stats.happiness_multiplier maybe)
        //   then maybe theres a percent chance of evolving (but I need to figure out the 'randomness' in a similar way that I did for stat selection and pet.pet_type selection but I should move that over into a separate module)

        pet.feed(env.block.time);
        Ok(pet)
    })?;

    Ok(Response::new().add_attribute("method", "try_feed"))
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

// TODO: Add unit tests to migrate()

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    let storage_ver = get_contract_version(deps.storage)?;

    // ensure we are migrating from an allowed contract
    if storage_ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }

    let cur_ver: Version = storage_ver.version.parse()?;
    let ver: Version = CONTRACT_VERSION.parse()?;
    if cur_ver < ver {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        // If state structure changed in any contract version in the way migration is needed, it
        // should occur here
    }

    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pet::state::{PetType, Stage};
    use cosmwasm_std::testing::{
        mock_dependencies_with_balance, mock_env, mock_info, MOCK_CONTRACT_ADDR,
    };
    use cosmwasm_std::{coins, from_binary};
    use cosmwasm_std::{Addr, BlockInfo, ContractInfo, Timestamp, TransactionInfo};

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
        assert_eq!(PetType::Air, pet.pet_type);
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
        let env = Env {
            block: BlockInfo {
                height: 12_345,
                time: Timestamp::from_nanos(1_572_797_419_879_305_533),
                chain_id: "cosmos-testnet-14002".to_string(),
            },
            transaction: Some(TransactionInfo { index: 3 }),
            contract: ContractInfo {
                address: Addr::unchecked(MOCK_CONTRACT_ADDR),
            },
        };
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();

        // pet.last_watered_time should be recent now:
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPetStatus {}).unwrap();
        let pet: PetResponse = from_binary(&res).unwrap();
        assert_eq!(pet.birth_date, pet.last_feeding_time);
        assert_eq!(true, pet.birth_date < pet.last_watering_time);
    }

    #[test]
    fn test_feed() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            name: "peepo".to_string(),
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // non-owner cannot feed pet:
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Feed {};
        let res = execute(deps.as_mut(), mock_env(), info, msg).err();
        assert_eq!(Some(ContractError::Unauthorized {}), res);

        // owner can feed pet:
        let info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Feed {};
        let env = Env {
            block: BlockInfo {
                height: 12_345,
                time: Timestamp::from_nanos(1_572_797_419_879_305_533),
                chain_id: "cosmos-testnet-14002".to_string(),
            },
            transaction: Some(TransactionInfo { index: 3 }),
            contract: ContractInfo {
                address: Addr::unchecked(MOCK_CONTRACT_ADDR),
            },
        };

        let _res = execute(deps.as_mut(), env, info, msg).unwrap();

        // pet.last_feed_time should be recent now:
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPetStatus {}).unwrap();
        let pet: PetResponse = from_binary(&res).unwrap();
        assert_eq!(pet.birth_date, pet.last_watering_time);
        assert_eq!(true, pet.birth_date < pet.last_feeding_time);
    }
}
