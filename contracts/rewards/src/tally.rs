use std::{array::TryFromSliceError, collections::HashMap, hash::Hash, vec};

use axelar_wasm_std::voting::Poll;
use cosmwasm_std::{Addr, DepsMut, Env, Fraction, StdError, StdResult, Storage, Uint256, Uint64};
use cw_storage_plus::{IntKey, Item, Key, KeyDeserialize, Map, PrimaryKey};

use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::{
    error::ContractError,
    msg::RewardsParams,
    state::{Config, CONFIG},
};



/*
pub fn record_participation(
    event_id: String,
    worker: Addr,
    contract: Addr,
    mut deps: DepsMut,
    env: Env,
) -> Result<(), ContractError> {
    let cur_epoch = update_epoch(&mut deps, &env)?;

    let event = EVENTS.may_load(deps.storage, (event_id.clone(), contract.clone()))?;
    let mut tally = match event {
        Some(event) => TALLIES
            .may_load(deps.storage, (contract.clone(), event.epoch_num))?
            .expect("couldn't find epoch tally for existing event"),
        None => {
            EVENTS.save(
                deps.storage,
                (event_id.clone(), contract.clone()),
                &Event {
                    event_id,
                    epoch_num: cur_epoch.epoch_num,
                },
            )?;
            TALLIES
                .may_load(deps.storage, (contract.clone(), cur_epoch.epoch_num))?
                .map_or(
                    EpochTally {
                        total_events: 1,
                        participation: HashMap::new(),
                        epoch_num: cur_epoch.epoch_num,
                        distributed_rewards: false,
                    },
                    |tally| EpochTally {
                        total_events: tally.total_events + 1,
                        ..tally
                    },
                )
        }
    };

    tally
        .participation
        .entry(worker)
        .and_modify(|count| *count = *count + 1)
        .or_insert(1);
    TALLIES.save(deps.storage, (contract, tally.epoch_num), &tally)?;
    Ok(())
}

pub fn process_rewards(
    contract: Addr,
    mut deps: DepsMut,
    env: Env,
) -> Result<HashMap<Addr, Uint256>, ContractError> {
    let cur_epoch = update_epoch(&mut deps, &env)?;
    let epoch = cur_epoch.epoch_num - 2;
    let mut to_reward = HashMap::new();
    loop {
        let res = process_rewards_for_epoch(contract.clone(), epoch, &mut deps, &env);
        match res {
            Err(ContractError::AlreadyDistributedRewards) => {
                break;
            }
            Err(_) => {
                return res;
            }
            Ok(rewards) => {
                to_reward.extend(rewards);
            }
        }
    }
    Ok(to_reward)
}

pub fn process_rewards_for_epoch(
    contract: Addr,
    epoch: u64,
    deps: &mut DepsMut,
    env: &Env,
) -> Result<HashMap<Addr, Uint256>, ContractError> {
    let tally = TALLIES.load(deps.storage, (contract.clone(), epoch))?;
    if tally.distributed_rewards {
        return Err(ContractError::AlreadyDistributedRewards);
    }

    let config = CONFIG.load(deps.storage)?;
    let cutoff = tally.total_events * u64::from(config.params.participation_threshold.numerator())
        / u64::from(config.params.participation_threshold.denominator());
    let mut to_reward = vec![];
    for (worker, participated) in &tally.participation {
        if *participated >= cutoff {
            to_reward.push(worker.clone());
        }
    }

    let pool = POOLS.load(deps.storage, contract.clone())?;
    let rate: cosmwasm_std::Uint256 = config.params.rewards_rate.into();
    if pool.cur_amount < rate {
        return Err(ContractError::PoolBalanceInsufficient);
    }
    if rate < Uint256::from_u128(to_reward.len() as u128) {
        return Err(ContractError::RateTooLow);
    }
    POOLS.save(
        deps.storage,
        contract.clone(),
        &Pool {
            cur_amount: pool.cur_amount - rate,
        },
    )?;
    TALLIES.save(
        deps.storage,
        (contract, epoch),
        &EpochTally {
            distributed_rewards: true,
            ..tally
        },
    )?;
    let rewards_per_worker = rate.multiply_ratio(1u32, to_reward.len() as u32);
    Ok(to_reward
        .into_iter()
        .map(|worker| (worker, rewards_per_worker))
        .collect())
}

fn update_epoch(deps: &mut DepsMut, env: &Env) -> Result<Epoch, ContractError> {
    let epoch_duration: u64 = CONFIG.load(deps.storage)?.params.epoch_duration.into();
    let cur_height = env.block.height;
    let epoch = CURRENT_EPOCH.load(deps.storage)?;
    if cur_height >= epoch.block_height_started + epoch_duration {
        let new_epoch_num = ((cur_height - epoch.block_height_started) / epoch_duration) + epoch.epoch_num;
        let new_epoch = Epoch {
            epoch_num: new_epoch_num,
            block_height_started: cur_height,
        };
        CURRENT_EPOCH.save(deps.storage, &new_epoch)?;
        return Ok(new_epoch);
    }
    Ok(epoch)
}

pub fn update_params(
    mut deps: DepsMut,
    env: Env,
    new_params: RewardsParams,
) -> Result<(), ContractError> {
    let _ = update_epoch(&mut deps, &env)?;
    CONFIG.update(deps.storage, |config| -> Result<Config, ContractError> {
        Ok(Config {
            params: new_params,
            ..config
        })
    })?;
    Ok(())
}

pub fn add_rewards(
    contract: Addr,
    amount: Uint256,
    deps: &mut DepsMut,
    env: &Env,
) -> Result<(), ContractError> {
    let _ = update_epoch(deps, env)?;
    POOLS.update(
        deps.storage,
        contract,
        |pool| -> Result<Pool, ContractError> {
            match pool {
                Some(pool) => Ok(Pool {
                    cur_amount: pool.cur_amount + amount,
                    ..pool
                }),
                None => Ok(Pool { cur_amount: amount }),
            }
        },
    )?;
    Ok(())
}

*/
