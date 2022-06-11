use cosmwasm_std::{Uint128, HumanAddr};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub denomination: Uint128,
    pub levels: u32,

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    DepositMsg {
        commitment: String,
    },
    WithDrawMsg {
        proof: [String;8],
        root: String,
        nullifier_hash: String,
        recipient: HumanAddr,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    CheckSpentMsg {
        nullifier_hash: String
    },

    GetLastRoot {
    },
}
