use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

use crate::msg::RewardsParams;


#[cw_serde]
struct Config {
    pub params: RewardsParams
}

pub const CONFIG : Item<Config> = Item::new("config");
