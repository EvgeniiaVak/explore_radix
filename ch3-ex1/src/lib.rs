use scrypto::prelude::*;

#[blueprint]
mod exercise_module {

    struct Exercise1 {
        // Step 3: Add two variables to the Struct of the blueprint.
        //   - a Vault named `bananas`
        //   - a ResourceAddress named `apples_resource_address`
        bananas: Vault,
        apples_resource_address: ResourceAddress,
    }

    impl Exercise1 {
        pub fn instantiate_exercise() -> ComponentAddress {
            // Step 1: Create a new fungible resource with a name "Banana" and symbol "BNN".
            // Make the resource burnable with an `allow_all` rule. Don't forget to make this flag unchangeable
            // by specifying `LOCKED` in the second parameter.
            // Make the initial supply 1000 and store it inside a `bananas` Bucket variable.
            let bananas = ResourceBuilder::new_fungible()
                .metadata("name", "Banana")
                .metadata("symbol", "BNN")
                .burnable(rule!(allow_all), LOCKED)
                .mint_initial_supply(1000);

            // Step 2: Create a new fungible resource with a name "Apple" and symbol "APP".
            // Make the resource mintable with an `allow_all` rule. Don't forget to make this flag unchangeable
            // by specifying `LOCKED` in the second parameter.
            // Initialize the resource with no initial supply and store the ResourceAddress in a `apples` variable.
            let apples = ResourceBuilder::new_fungible()
                .metadata("name", "Apple")
                .metadata("symbol", "APP")
                .mintable(rule!(allow_all), LOCKED)
                .create_with_no_initial_supply();

            Self {
                // Step 4: Initialize the two variables.
                //  - bananas will be a Vault containing the `bananas` bucket we just created.
                //  - apples_resource_address will simply be the `apples` variables we got in step 2.
                bananas: Vault::with_bucket(bananas),
                apples_resource_address: apples,
            }
            .instantiate()
            .globalize()
        }

        // Step 5: Write a `mint_apple` method that returns a Bucket.
        // Inside, borrow the apple resource manager, mint a token and return it.
        pub fn mint_apple(&self) -> Bucket {
            let resource_manager = borrow_resource_manager!(self.apples_resource_address);
            resource_manager.mint(1)
        }

        // Step 6: Write a `get_banana` method that returns a Bucket.
        // Inside, take a single token from the `bananas` vault and return it.
        pub fn get_banana(&mut self) -> Bucket {
            self.bananas.take(1)
        }

        // Step 7: Create a `burn_banana` method that accepts a `banana` Bucket.
        // Inside, burn the bucket passed as argument.
        pub fn burn_banana(&mut self, banana: Bucket) {
            banana.burn();
        }
    }
}
