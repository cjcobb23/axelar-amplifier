use axelar_wasm_std_derive::IntoContractError;
use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, IntoContractError)]
pub enum ContractError {
    #[error("not enough funds in rewards pool")]
    PoolBalanceInsufficient,

    #[error("rewards rate too low for number of workers")]
    RateTooLow,

    #[error("already distributed rewards for epoch")]
    AlreadyDistributedRewards,

    #[error(transparent)]
    Std(#[from] StdError),
}
