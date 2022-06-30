use std::ops::Add;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult,
};
use cw2::set_contract_version;
use cw_storage_plus::Bound;

use crate::error::ContractError;
use crate::msg::{EntryResponse, ExecuteMsg, InstantiateMsg, ListResponse, QueryMsg};
use crate::state::{
    Config, Entry, Priority, Status, DEFAULT_LIMIT, ENTRY_SEQ, INIT_CONFIG, LIST, MAX_LIMIT,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:my-first-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    //Store the contract name and version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Get the owner of the contract
    let owner = msg
        .owner
        .and_then(|addr_str| deps.api.addr_validate(addr_str.as_str()).ok())
        .unwrap_or(info.sender);

    let config = Config {
        owner: owner.clone(),
    };

    // save the owner to the INIT_CONFIG state
    INIT_CONFIG.save(deps.storage, &config)?;

    // save the entry sequence to storage starting from 0
    ENTRY_SEQ.save(deps.storage, &0u64)?;

    // return response
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::NewEntry {
            description,
            priority,
        } => execute_create_new_entry(deps, info, description, priority),
        ExecuteMsg::UpdateEntry {
            id,
            description,
            status,
            priority,
        } => execute_update_entry(deps, info, id, description, status, priority),
        ExecuteMsg::DeleteEntry { id } => execute_delete_entry(deps, info, id),
        ExecuteMsg::BurnContractBalance {} => execute_burn_balance(deps, info, _env),
        ExecuteMsg::TransferContractOwnership { new_owner } => {
            execute_transfer_owner(deps, info, new_owner)
        }
    }
}

fn execute_create_new_entry(
    deps: DepsMut,
    info: MessageInfo,
    description: String,
    priority: Option<Priority>,
) -> Result<Response, ContractError> {
    // Before we create a new entry, we check to see if the message sender is the owner of the contract
    let owner = INIT_CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    // in order to generate a new id, we get the ENTRY_SEQ and increment it by 1
    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    // create the new entry info
    let entry = Entry {
        id,
        description,
        status: Status::ToDo,
        priority: priority.unwrap_or(Priority::High),
    };

    // save the new entry to the list
    LIST.save(deps.storage, id, &entry)?;

    Ok(Response::new()
        .add_attribute("method", "execute_create_new_entry")
        .add_attribute("id", id.to_string()))
}

fn execute_update_entry(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    description: Option<String>,
    status: Option<Status>,
    priority: Option<Priority>,
) -> Result<Response, ContractError> {
    // Before we create a new entry, we check to see if the message sender is the owner of the contract
    let owner = INIT_CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    // get the entry with the id
    let entry = LIST.load(deps.storage, id)?;

    // update the entry
    let updated_entry = Entry {
        id,
        description: description.unwrap_or(entry.description),
        status: status.unwrap_or(entry.status),
        priority: priority.unwrap_or(entry.priority),
    };

    // save the updated entry
    LIST.save(deps.storage, id, &updated_entry)?;

    Ok(Response::new()
        .add_attribute("method", "execute_update_entry")
        .add_attribute("id", id.to_string()))
}

fn execute_delete_entry(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    // Before we create a new entry, we check to see if the message sender is the owner of the contract
    let owner = INIT_CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    // remove the entry with id from the list
    LIST.remove(deps.storage, id);

    Ok(Response::new()
        .add_attribute("method", "execute_delete_entry")
        .add_attribute("id", id.to_string()))
}

fn execute_transfer_owner(
    deps: DepsMut,
    info: MessageInfo,
    new_owner: String,
) -> Result<Response, ContractError> {
    // Before we create a new entry, we check to see if the message sender is the owner of the contract
    let owner = INIT_CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    // validate new owner
    let new_owner = deps.api.addr_validate(&new_owner)?;

    // Here we update the owner in the config
    let updated_config = INIT_CONFIG.update(deps.storage, |mut data| -> StdResult<_> {
        data.owner = new_owner;

        Ok(data)
    })?;

    Ok(Response::new()
        .add_attribute("method", "execute_transfer_owner")
        .add_attribute("new_owner", updated_config.owner))
}

fn execute_burn_balance(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
) -> Result<Response, ContractError> {
    // Before we create a new entry, we check to see if the message sender is the owner of the contract
    let owner = INIT_CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    // Get the contract balances
    let amount = deps.querier.query_all_balances(&env.contract.address)?;

    // create a burn message
    let burn_msg = BankMsg::Burn { amount };

    // Then we add the message to the response
    let msgs: Vec<CosmosMsg> = vec![burn_msg.into()];

    // Build response
    let res = Response::new()
        .add_attribute("method", "try_burn_balance")
        .add_messages(msgs);

    // return response
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // Match and route the query message to the appropriate handler
    match msg {
        QueryMsg::QueryEntry { id } => to_binary(&query_entry(deps, id)?),
        QueryMsg::QueryList { start_after, limit } => {
            to_binary(&query_list(deps, start_after, limit)?)
        }
    }
}

fn query_entry(deps: Deps, id: u64) -> StdResult<EntryResponse> {
    // get the entry with the id
    let entry = LIST.load(deps.storage, id)?;

    Ok(EntryResponse {
        id: entry.id,
        description: entry.description,
        priority: entry.priority,
        status: entry.status,
    })
}

fn query_list(deps: Deps, start_after: Option<u64>, limit: Option<u32>) -> StdResult<ListResponse> {
    let start = start_after.map(Bound::exclusive);
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    // get the entries that matches the range
    let entries: StdResult<Vec<_>> = LIST
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .collect();

    let results = ListResponse {
        entries: entries?.into_iter().map(|l| l.1).collect(),
    };

    Ok(results)
}

#[cfg(test)]
mod tests;
