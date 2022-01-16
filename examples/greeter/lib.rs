#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod greeter {

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if caller is not owner.
        CallerIsNotOwner,
    }

    #[ink(storage)]
    pub struct Greeter {
        greeting: str,
        owner: AccountId,
    }

    impl Greeter {
        #[ink(constructor)]
        pub fn new(init_greeting: Box<str>) -> Box<Self> {
            Self {
                greeting: init_greeting,
                owner: Self::env().caller(),
            }
        }

        #[ink(message)]
        pub fn change_greeting(&mut self, new_greeting: Box<str>) {
            let caller = self.env().caller();
            if caller == owner {
                self.greeting = new_greeting;
            } else {
                return Err(Error::CallerIsNotOwner);
            }
            Ok(());
        }

        #[ink(message)]
        pub fn change_owner(&mut self, new_owner: AccountId) -> Result {
            let caller = self.env().caller();
            if caller == owner {
                self.owner = new_owner;
            } else {
                return Err(Error::CallerIsNotOwner);
            }
            Ok(());
        }

        #[ink(message)]
        pub fn greet(&self) -> Box<str> {
            self.greeting
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn it_can_set_greeting() {
            let mut contract = Greeter::new("Welcome!");
            assert_eq!(contract.greet(), "Welcome!");
            contract.change_greeting("Hello mate!");
            assert_eq!(contract.greet(), "Hello mate!");
        }

        fn it_can_change_owner() {
            let accounts = default_accounts();
            let alice = accounts.alice;
            let mut contract = Greeter::new("Welcome!");
            contract.change_owner(alice);
            assert_eq!(contract.owner(), alice);
        }
    }
}
