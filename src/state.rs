use crate::math::Uint256;
use cosmwasm_std::Uint128;
use cw_storage_plus::Item;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PTrans {
    pub denomination: Uint128,
    pub nullifier_hashes: Vec<Uint256>,
    pub commitments: Vec<Uint256>,
}

pub const PTRANS: Item<PTrans> = Item::new("ptrans");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MerkelTree {
    pub levels: u32,
    pub filled_subtrees: Vec<Uint256>,
    pub roots: Vec<Uint256>,
    pub current_root_index: u32,
    pub next_index: u32,
}

pub const ROOT_HISTORY_SIZE: u32 = 30;

pub const MERKEL_TREE: Item<MerkelTree> = Item::new("merkel_tree");
