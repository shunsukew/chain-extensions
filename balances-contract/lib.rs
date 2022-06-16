#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod balances {
    use balances_crate::*;

    #[derive(scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum CustomError {
        BalancesError(BalancesError),
    }

    #[ink(storage)]
    pub struct BalancesContract {}

    impl BalancesContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn free_balance(&self, account_id: AccountId) -> Balance {
            Balances::free_balance(account_id)
        }

        #[ink(message)]
        pub fn usable_balance(&self, account_id: AccountId) -> Balance {
            Balances::usable_balance(account_id)
        }

        #[ink(message)]
        pub fn transfer(&mut self, account_id: AccountId, value: Balance) -> Result<(), CustomError> {
            Balances::transfer(account_id, value)
                .map_err(|e| return CustomError::BalancesError(e))
        }
    }
}