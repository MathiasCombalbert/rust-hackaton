#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod ink_show {
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Role {
        SuperAdmin,
        Admin,
        Moderator,
        User,
    }

    impl Clone for Role {
        fn clone(&self) -> Self {
            match self {
                Role::SuperAdmin => Role::SuperAdmin,
                Role::Admin => Role::Admin,
                Role::Moderator => Role::Moderator,
                Role::User => Role::User,
            }
        }
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]

    pub struct InkShow {
        /// Stores a single `bool` value on the storage.
        value: bool,
        role: Role,
    }

    impl InkShow {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool, init_role: Role) -> Self {
            Self {
                value: init_value,
                role: init_role,
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default(), Role::User)
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn get_role(&self) -> Role {
            self.role.clone()
        }

        #[ink(message)]
        pub fn set(&mut self, role: Role, cli: &InkShow) {
            if self.role == Role::SuperAdmin {
                cli.role = role.clone();
            } else if self.role == Role::Admin {
                if role == Role::Modo || role == Role::User {
                    cli.role = role.clone();
                }
            }
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let ink_show = InkShow::default();
            assert_eq!(ink_show.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut ink_show = InkShow::new(false, Role::Moderator);
            assert_eq!(ink_show.get(), false);
            ink_show.flip();
            assert_eq!(ink_show.get(), true);
        }
    }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = InkShowRef::default();

            // When
            let contract_account_id = client
                .instantiate("ink_show", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<InkShowRef>(contract_account_id.clone())
                .call(|ink_show| ink_show.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = InkShowRef::new(false);
            let contract_account_id = client
                .instantiate("ink_show", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<InkShowRef>(contract_account_id.clone())
                .call(|ink_show| ink_show.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<InkShowRef>(contract_account_id.clone())
                .call(|ink_show| ink_show.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<InkShowRef>(contract_account_id.clone())
                .call(|ink_show| ink_show.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
