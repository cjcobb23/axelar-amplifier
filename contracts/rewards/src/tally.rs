use std::{array::TryFromSliceError, collections::HashMap, hash::Hash, vec};

use axelar_wasm_std::ContractError;
use cosmwasm_std::{Addr, DepsMut, Env, Fraction, StdError, StdResult, Uint256, Uint64};
use cw_storage_plus::{IntKey, Item, Key, KeyDeserialize, Map, PrimaryKey};

use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::{msg::RewardsParams, state::CONFIG};

#[cw_serde]
struct EpochTally {
    total_events: u64,
    participation: HashMap<Addr, u64>,
    epoch_num: u64,
}

#[cw_serde]
struct Event {
    event_id: String,
    epoch: u64,
}

#[cw_serde]
struct Epoch {
    epoch_num: u64,
    start: u64,
}

#[cw_serde]
struct Pool {
    cur_amount: Uint256,
}

const CURRENT_EPOCH: Item<Epoch> = Item::new("current_epoch");

const TALLIES: Map<(Addr, u64), EpochTally> = Map::new("tallies");

const EVENTS: Map<(String, Addr), Event> = Map::new("events");

const POOLS: Map<Addr, Pool> = Map::new("pools");

pub fn record_participation(
    event_id: String,
    worker: Addr,
    contract: Addr,
    deps: DepsMut,
    env: Env,
) -> Result<(), ContractError> {
    let cur_epoch = update_epoch(deps, env)?;

    let event = EVENTS.may_load(deps.storage, (event_id, contract))?;
    let mut tally = match event {
        Some(event) => TALLIES
            .may_load(deps.storage, (contract, event.epoch))?
            .expect("couldn't find epoch tally for existing event"),
        None => {
            EVENTS.save(
                deps.storage,
                (event_id, contract),
                &Event {
                    event_id,
                    epoch: cur_epoch.epoch_num,
                },
            )?;
            TALLIES
                .may_load(deps.storage, (contract, cur_epoch.epoch_num))?
                .map_or(
                    EpochTally {
                        total_events: 1,
                        participation: HashMap::new(),
                        epoch_num: cur_epoch.epoch_num,
                    },
                    |tally| EpochTally {
                        total_events: tally.total_events + 1,
                        ..tally
                    },
                )
        }
    };
    
    tally.participation.entry(worker).and_modify(|count| *count = *count+1).or_insert(1);
    TALLIES.save(deps.storage, (contract, tally.epoch_num), &tally)?;
    Ok(())
}

pub fn get_pool_amount(contract: Addr) -> u32 {
    todo!()
}

pub fn distribute_rewards(
    contract: Addr,
    cur_epoch: Epoch,
    deps: DepsMut,
    env: Env,
) -> Result<Vec<Addr>, ContractError> {
    let cur_epoch = update_epoch(deps, env)?;
    let epoch = cur_epoch.epoch_num - 2;
    let tally = TALLIES.load(deps.storage, (contract, epoch))?;

    let config = CONFIG.load(deps.storage)?;
    let cutoff = tally.total_events * config.params.participation_threshold.numerator()
        / config.params.participation_threshold.denominator();
    let mut to_reward = vec![];
    for (worker, participated) in tally.participation {
        if participated >= cutoff {
            to_reward.push(worker);
        }
    }
    Ok(to_reward)
}

fn update_epoch(deps: DepsMut, env: Env) -> Result<Epoch, ContractError> {
    let epoch_duration: u64 = CONFIG.load(deps.storage)?.params.epoch_duration.into();
    let cur_height = env.block.height;
    let epoch = CURRENT_EPOCH.load(deps.storage)?;
    if cur_height >= epoch.start + epoch_duration {
        let new_epoch_num = ((cur_height - epoch.start) / epoch_duration) + epoch.epoch_num;
        let new_epoch = Epoch {
            epoch_num: new_epoch_num,
            start: cur_height,
        };
        CURRENT_EPOCH.save(deps.storage, &new_epoch)?;
        return Ok(new_epoch);
    }
    Ok(epoch)
}

pub fn update_params(
    deps: DepsMut,
    env: Env,
    new_params: RewardsParams,
) -> Result<Vec<Addr>, ContractError> {
    let cur_epoch = update_epoch(deps, env)?;
    todo!()
}

/*
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
*/
