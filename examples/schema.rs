use std::{env::current_dir, fs::create_dir_all};

use cosmwasm_schema::{remove_schemas, export_schema, schema_for};
use cosmwasm_std::MessageInfo;
use ptrans::msg::{InitMsg, ExecuteMsg, QueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");

    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InitMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(MessageInfo), &out_dir);
}