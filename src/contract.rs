use coreum_wasm_sdk::assetnft::{self, DISABLE_SENDING};
use coreum_wasm_sdk::core::{CoreumMsg, CoreumQueries};
use cosmwasm_std::{entry_point, to_binary, Binary, Deps, QueryRequest, StdResult};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError};
use cw2::set_contract_version;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// version info for migration info
const CONTRACT_NAME: &str = "creates.io:nft-test";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: String,
}

pub const STATE: Item<State> = Item::new("state");

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid input")]
    InvalidInput(String),

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    MintNFT {
        class_id: String,
        id: String,
        data: Binary,
    },
    // ReceiveAirdrop {},
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<CoreumMsg>, ContractError> {
    match msg {
        ExecuteMsg::MintNFT { class_id, id, data } => mint_nft(deps, info, class_id, id, data), // ExecuteMsg::ReceiveAirdrop {} => receive_airdrop(deps, info),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Token { id: String },
    // MintedForAirdrop {},
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<CoreumQueries>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Token { id } => token(deps, id),
        // QueryMsg::MintedForAirdrop {} => minted_for_airdrop(deps),
    }
}

// ********** Instantiate **********

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<CoreumMsg>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let issue_class_msg = CoreumMsg::AssetNFT(assetnft::Msg::IssueClass {
        name: msg.name,
        symbol: msg.symbol,
        description: Some("Test description".to_string()),
        uri: None,
        uri_hash: None,
        data: None,
        features: Some(vec![DISABLE_SENDING]),
        royalty_rate: Some("0".to_string()),
    });

    let state = State {
        owner: info.sender.into(),
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("owner", state.owner)
        .add_message(issue_class_msg))
}

// ********** Transactions **********

fn mint_nft(
    deps: DepsMut,
    info: MessageInfo,
    class_id: String,
    id: String,
    data: Binary,
) -> Result<Response<CoreumMsg>, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    let msg = CoreumMsg::AssetNFT(assetnft::Msg::Mint {
        class_id: class_id.clone(),
        id: id.clone(),
        uri: None,
        uri_hash: None,
        data: Some(data.clone()),
    });

    // state.minted_for_airdrop = state.minted_for_airdrop.add(Uint128::new(amount));
    // STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "mint_nft")
        .add_attribute("class_id", class_id)
        .add_attribute("id", id)
        .add_attribute("data", data.to_string())
        .add_message(msg))
}

// ********** Queries **********

fn token(deps: Deps<CoreumQueries>, id: String) -> StdResult<Binary> {
    // let state = STATE.load(deps.storage)?;
    let request: QueryRequest<CoreumQueries> =
        CoreumQueries::AssetNFT(assetnft::Query::Class { id }).into();
    let res: assetnft::ClassResponse = deps.querier.query(&request)?;
    to_binary(&res)
}
