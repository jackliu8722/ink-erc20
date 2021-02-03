#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {

    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: StorageHashMap<AccountId, Balance>,
        allowances: StorageHashMap<(AccountId, AccountId), Balance>,
    }

    #[derive(Debug,PartialEq,Eq,scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
    }

    type Result<T> = core::result::Result<T,Error>;

    #[ink(event)]
    pub struct Transfer{
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Burn {
        #[ink(topic)]
        from: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Mint {
        #[ink(topic)]
        from: AccountId,
        value: Balance,
    }

    impl Erc20 {

        #[ink(constructor)]
        pub fn new(init_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = StorageHashMap::new();
            balances.insert(caller,init_supply);
            Self {
                total_supply: init_supply,
                balances,
                allowances: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            *self.balances.get(&owner).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn allowance_of(&self, owner: AccountId, spender: AccountId) -> Balance {
            *self.allowances.get(&(owner,spender)).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(from,to,value)
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let spender = self.env().caller();
            let allowance = self.allowance_of(from,spender);
            if allowance < value {
                return Err(Error::InsufficientBalance);
            }

            self.allowances.insert((from,spender),allowance - value);
            self.transfer_from_to(from,to,value)

        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()>{
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(from,from_balance - value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to,to_balance + value);
            self.env().emit_event(Transfer{from,to,value});
            Ok(())
        }

        #[ink(message)]
        pub fn burn(&mut self, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            let balance = self.balance_of(caller);
            if balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.total_supply = self.total_supply - value;
            self.balances.insert(caller,balance - value);
            self.env().emit_event(Burn{from: caller, value});
            Ok(())
        }

        #[ink(message)]
        pub fn mint(&mut self, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            let balance = self.balance_of(caller);

            self.total_supply = self.total_supply + value;
            self.balances.insert(caller,balance + value);
            self.env().emit_event(Mint{from: caller, value});
            Ok(())
        }


    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        // /// We test if the default constructor does its job.
        // #[test]
        // fn default_works() {
        //     let erc20 = Erc20::default();
        //     assert_eq!(erc20.get(), false);
        // }
        //
        // /// We test a simple use case of our contract.
        // #[test]
        // fn it_works() {
        //     let mut erc20 = Erc20::new(false);
        //     assert_eq!(erc20.get(), false);
        //     erc20.flip();
        //     assert_eq!(erc20.get(), true);
        // }
    }
}
