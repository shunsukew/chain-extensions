#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::{AccountId, DefaultEnvironment, Environment};
use scale::{Decode, Encode};

type Balance = <DefaultEnvironment as Environment>::Balance;

pub struct Balances;

impl Balances {
    pub fn free_balance(account_id: AccountId) -> Balance {
        ::ink_env::chain_extension::ChainExtensionMethod::build(3101u32)
            .input::<AccountId>()
            .output::<Balance>()
            .ignore_error_code()
            .call(&account_id)
    }

    pub fn usable_balance(account_id: AccountId) -> Balance {
        ::ink_env::chain_extension::ChainExtensionMethod::build(3102u32)
            .input::<AccountId>()
            .output::<Balance>()
            .ignore_error_code()
            .call(&account_id)
    }

    pub fn transfer(account_id: AccountId, value: Balance) -> Result<(), BalancesError> {
        let input = TransferInput { account_id, value };
        ::ink_env::chain_extension::ChainExtensionMethod::build(3103u32)
            .input::<TransferInput>()
            .output::<Result<(), BalancesError>>()
            .handle_error_code::<BalancesError>()
            .call(&input)?
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode)]
pub struct TransferInput {
    account_id: AccountId,
    value: Balance,
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum BalancesErrorCode {
    Failed,
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum BalancesError {
    Failed
}

impl From<BalancesErrorCode> for BalancesError {
    fn from(error_code: BalancesErrorCode) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

impl From<scale::Error> for BalancesError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

impl ink_env::chain_extension::FromStatusCode for BalancesError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::Failed),
            _ => panic!("encountered unknown status code"),
        }
    }
}
