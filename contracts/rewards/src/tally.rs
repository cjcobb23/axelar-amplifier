use std::{collections::HashMap, array::TryFromSliceError, vec, thread::current};

use axelar_wasm_std::ContractError;
use cosmwasm_std::{Addr, DepsMut, StdResult, StdError, Fraction, Uint64, Uint256, Env};
use cw_storage_plus::{Map, Item, PrimaryKey, Key, KeyDeserialize, IntKey};

use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::CONFIG;

#[cw_serde]
struct EpochTally {
    total_events: Uint64,
    participation: HashMap<Addr,Uint64>,
    start_block: Uint64
}

#[cw_serde]
struct Event {
    event_id: String,
    epoch: u32,
}

#[cw_serde]
struct Epoch {
    epoch_num: u32,
    start: u64,
}





#[cw_serde]
struct Pool {
    cur_amount: Uint256
}

const CURRENT_EPOCH : Item<Epoch> = Item::new("current_epoch");

const TALLIES : Map<(Addr,u32),EpochTally> = Map::new("tallies");

const EVENTS : Map<(String, Addr),Event> = Map::new("events");

const POOLS: Map<Addr,Pool> = Map::new("pools");


pub fn find_tally(event_id: String, contract: Addr, deps: DepsMut) -> Result<EpochTally, ContractError> {
    let event = EVENTS.may_load(deps.storage, (event_id, contract))?;
    if let Some(event) = event {
        Ok(TALLIES.load(deps.storage, (contract, event.epoch))?)
    }
    
}

pub fn save_tally(epoch_tally: EpochTally) -> Result<(),ContractError> {
    todo!()
}

pub fn record_participation(event_id: String, worker: Addr, contract: Addr, deps: DepsMut) -> Result<(),ContractError> {

    let mut tally = find_tally(event_id, contract, deps)?;
    let event = EVENTS.may_load(deps.storage, event_id)?;
    if event.is_some() {
        TALLIES.load(deps.storage, )
    }
    let mut worker_tally = tally.participation.entry(worker).or_default();
    *worker_tally += 1;
    if E
    save_tally(tally)?;
    Ok(())
}

pub fn get_pool_amount(contract: Addr) -> u32 {
    todo!()
}



pub fn distribute_rewards(contract: Addr, cur_epoch: Epoch, deps: DepsMut) -> Result<Vec<Addr>, ContractError> {
    let epoch = cur_epoch.epoch_num - 2;
    let tally = TALLIES.load(deps.storage,(contract,epoch))?;

    let config = CONFIG.load(deps.storage)?;
    let cutoff = tally.total_events * config.params.participation_threshold.numerator() / config.params.participation_threshold.denominator();
    let mut to_reward = vec![];
    for (worker,participated) in tally.participation {

        if participated >= cutoff {

            to_reward.push(worker);
        }
    }
    Ok(to_reward)
}

pub fn end_blocker(deps: DepsMut, env: Env) -> Result<(), ContractError> {
       let config = CONFIG.load(deps.storage)?; 

       let mut current_epoch = CURRENT_EPOCH.load(deps.storage)?;
       if current_epoch.start + config.params.epoch_duration.into() <= env.block.height {
            current_epoch.epoch_num += 1;
            current_epoch.start = env.block.height;
            CURRENT_EPOCH.save(deps.storage, &current_epoch);
       }
       Ok(())
}
