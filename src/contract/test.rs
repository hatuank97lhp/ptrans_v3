use super::*;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{attr, coins, CosmosMsg};


#[test]
pub fn test() {
let mut deps = mock_dependencies(&[]);

let one = Uint128(1);

let msg_init = InitMsg {
    denomination: one,
    levels: 20,
};

let info = mock_info("creator", &coins(1, "orai"));

let msg = init(deps.as_mut(), mock_env(), info, msg_init).unwrap();

println!("--init:---\n{}\n\n", msg.messages.len());


println!("---deposit---");
let info = mock_info("creator", &coins(1, "orai"));
let commitment: String = String::from("12345");
let msg = deposit(deps.as_mut(), mock_env(), info, commitment).unwrap();
println!("{}\n\n", msg.messages.len());

println!("---with_draw---\n");
let info = mock_info("anyone", &coins(0, "orai"));
let last_root = get_last_root(deps.as_ref(), mock_env()).unwrap().last_root;
let string1 = String::from("11");
let string2 = String::from("11");
let string3 = String::from("11");
let string4 = String::from("11");
let string5 = String::from("11");
let string6 = String::from("11");
let string7 = String::from("11");
let string8 = String::from("11");
let proof: [String; 8] = [string1.clone(), string2.clone(),string3.clone(),string4.clone(),string5.clone(),string6.clone(),string7.clone(),string8.clone()];

let root = last_root;
let nullifier_hash = String::from("111");
let recipient = String::from("someone");
let msg = with_draw(deps.as_mut(), mock_env(), info, proof, root, nullifier_hash, recipient).unwrap();

println!("{}", msg.messages.len());

}
