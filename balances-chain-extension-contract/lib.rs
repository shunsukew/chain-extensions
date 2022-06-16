#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;

#[ink::chain_extension]
pub trait BalancesExt {
    type ErrorCode = BalancesErrorCode;

    #[ink(extension = 3101, returns_result = false, handle_status = false)]
    fn free_balance(
        account_id: <ink_env::DefaultEnvironment as Environment>::AccountId
    ) -> <ink_env::DefaultEnvironment as Environment>::Balance;

    #[ink(extension = 3102, returns_result = false, handle_status = false)]
    fn usable_balance(
        account_id: <ink_env::DefaultEnvironment as Environment>::AccountId
    ) -> <ink_env::DefaultEnvironment as Environment>::Balance;

    #[ink(extension = 3103)]
    fn transfer(
        account_id: <ink_env::DefaultEnvironment as Environment>::AccountId,
        value: <ink_env::DefaultEnvironment as Environment>::Balance,
    ) -> Result<(), BalancesError>;
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum BalancesErrorCode {
    Failed,
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum BalancesError {
    ErrorCode(BalancesErrorCode),
}

impl From<BalancesErrorCode> for BalancesError {
    fn from(error_code: BalancesErrorCode) -> Self {
        Self::ErrorCode(error_code)
    }
}

impl From<scale::Error> for BalancesError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

impl ink_env::chain_extension::FromStatusCode for BalancesErrorCode {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::Failed),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize = <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = BalancesExt;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod dapp_staking_extension {
    use super::{BalancesError};

    #[ink(storage)]
    pub struct BalancesExtension {}

    impl BalancesExtension {
        #[ink(constructor)]
        pub fn new() -> Self {
            BalancesExtension {}
        }

        /// Calls current_era() in the pallet-dapps-staking
        #[ink(message)]
        pub fn free_balance(&self, account_id: AccountId) -> Balance {
             self.env().extension().free_balance(account_id)
        }

        /// Calls general_era_info() in the pallet-dapps-staking
        #[ink(message)]
        pub fn usable_balance(&self, account_id: AccountId) -> Balance {
            self.env().extension().usable_balance(account_id)
        }

        #[ink(message)]
        pub fn transfer(
            &mut self,
            account_id: AccountId,
            value: Balance,
        ) -> Result<(), BalancesError> {
            self.env().extension().transfer(account_id, value)
        }
    }
}
