use cosmwasm_std::{Deps, StdResult};

use connection_router::state::Message;

use crate::error::ContractError;
use crate::state::VERIFIED_MESSAGES;

pub fn verification_statuses(deps: Deps, messages: Vec<Message>) -> StdResult<Vec<(String, bool)>> {
    messages
        .into_iter()
        .map(|message| {
            is_message_verified(deps, &message).map(|verified| (message.id.to_string(), verified))
        })
        .collect::<Result<Vec<(_, _)>, _>>()
        .map_err(Into::into)
}

pub fn is_message_verified(deps: Deps, message: &Message) -> Result<bool, ContractError> {
    match VERIFIED_MESSAGES.may_load(deps.storage, &message.id)? {
        Some(stored) if stored != *message => {
            Err(ContractError::MessageMismatch(message.id.to_string()))
        }
        Some(_) => Ok(true),
        None => Ok(false),
    }
}