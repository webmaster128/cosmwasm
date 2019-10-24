use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Params<'a> {
    #[serde(borrow)]
    pub block: BlockInfo<'a>,
    #[serde(borrow)]
    pub message: MessageInfo<'a>,
    #[serde(borrow)]
    pub contract: ContractInfo<'a>,
}

#[derive(Serialize, Deserialize)]
pub struct BlockInfo<'a> {
    pub height: i64,
    // time is seconds since epoch begin (Jan. 1, 1970)
    pub time: i64,
    pub chain_id: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct MessageInfo<'a> {
    pub signer: &'a str,
    #[serde(borrow)]
    pub sent_funds: &'a [Coin<'a>],
}

#[derive(Serialize, Deserialize)]
pub struct ContractInfo<'a> {
    pub address: &'a str,
    #[serde(borrow)]
    pub balance: &'a [Coin<'a>],
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Coin<'a> {
    pub denom: &'a str,
    pub amount: &'a str,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CosmosMsg<'a> {
    // this moves tokens in the underlying sdk
    Send {
        from_address: &'a str,
        to_address: &'a str,
        amount: &'a [Coin<'a>],
    },
    // this dispatches a call to another contract at a known address (with known ABI)
    // msg is the json-encoded HandleMsg struct
    Contract {
        contract_addr: &'a str,
        msg: &'a str,
    },
    // this should never be created here, just passed in from the user and later dispatched
    Opaque {
        data: &'a str,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContractResult<'a> {
    Ok(Response<'a>),
    Err(&'a str),
}

impl<'a> ContractResult<'a> {
    // unwrap will panic on err, or give us the real data useful for tests
    pub fn unwrap(self) -> Response<'a> {
        match self {
            ContractResult::Err(msg) => panic!("Unexpected error: {}", msg),
            ContractResult::Ok(res) => res,
        }
    }

    pub fn is_err(&self) -> bool {
        match self {
            ContractResult::Err(_) => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Response<'a> {
    // let's make the positive case a struct, it contrains Msg: {...}, but also Data, Log, maybe later Events, etc.
    pub messages: &'a [CosmosMsg<'a>],
    pub log: Option<&'a str>,
    pub data: Option<&'a str>,
}

// just set signer, sent funds, and balance - rest given defaults
// this is intended for use in testcode only
pub fn mock_params<'a>(signer: &'a str, sent: &'a [Coin<'a>], balance: &'a [Coin<'a>]) -> Params<'a> {
    Params {
        block: BlockInfo {
            height: 12345,
            time: 1571797419,
            chain_id: "cosmos-testnet-14002",
        },
        message: MessageInfo {
            signer: signer,
            sent_funds: sent,
        },
        contract: ContractInfo {
            address: "cosmos2contract",
            balance: balance,
        },
    }
}

// coin is a shortcut constructor for a set of one denomination of coins
pub fn coin<'a>(amount: &'a str, denom: &'a str) -> Vec<Coin<'a>> {
    vec![Coin {
        amount: amount,
        denom: denom,
    }]
}
