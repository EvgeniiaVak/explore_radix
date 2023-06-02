use scrypto::prelude::*;

// Step 1: Define the Struct that will be used to store the
// members non fungible metadata. Name the struct `MemberData` and add a Decimal field
// named `amount_staked` that is **mutable**.
#[derive(ScryptoSbor, NonFungibleData, Debug)]
struct MemberData {
    #[mutable]
    amount_staked: Decimal,
}

#[blueprint]
mod exercise_module {
    struct Exercise1 {
        // Step 4: Define the variables that the instantiated component
        // will have access to. We need a vault to store the staked XRD,
        // a Vault to store the member_manager badge and a ResourceAddress to store the address
        // of the member badges
        xrd_vault: Vault,
        member_manager_badge: Vault,
        member_badge: ResourceAddress,
    }

    impl Exercise1 {
        pub fn instantiate_exercise() -> ComponentAddress {
            // Step 2: Create a new fungible resource with a divisibility of 0 and an initial supply of 1.
            // This will be the badge that is allowed to mint member badges. Store the token in a `member_manager_badge` Bucket variable.
            let member_manager_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "MemberManagerBadge")
                .mint_initial_supply(1);

            // Step 3: Create a non-fungible resource with IDs of the UUID type.
            // This resource is `mintable` and `updateable_non_fungible_data` by someone showing ownership of
            // the `member_manager_badge`. Initialize the resource with no initial supply and store the returned
            // ResourceAddress in a `member_badge` variable.
            let member_badge: ResourceAddress =
                ResourceBuilder::new_uuid_non_fungible::<MemberData>()
                    .metadata("name", "MemberBadge")
                    .mintable(
                        rule!(require(member_manager_badge.resource_address())),
                        LOCKED,
                    )
                    .updateable_non_fungible_data(
                        rule!(require(member_manager_badge.resource_address())),
                        LOCKED,
                    )
                    .create_with_no_initial_supply();

            Self {
                // Step 5: Assign a value for the three variables we defined in the struct.
                xrd_vault: Vault::new(RADIX_TOKEN),
                member_manager_badge: Vault::with_bucket(member_manager_badge),
                member_badge,
            }
            .instantiate()
            .globalize()
        }

        // Step 6: Create a `become_member` method that returns a Bucket.
        pub fn become_member(&mut self) -> Bucket {
            // Inside, instantiate a new MemberData struct with a value of 0 for the amount_staked.
            let member_data = MemberData {
                amount_staked: Decimal::from(0),
            };

            // Then put a proof of the `manager_badge` on the AuthZone (using the .authorize() method),
            let member_resource_manager = borrow_resource_manager!(self.member_badge);
            self.member_manager_badge.authorize(|| {
                // mint a new member non fungible resource with a random local ID and return it.
                // Remember that because the ID type of the resource is UUID, you will have to call `mint_uuid_non_fungible(data)`
                // instead of `mint_non_fungible(id, data)`
                member_resource_manager.mint_uuid_non_fungible(member_data)
            })
        }

        // Step 7: Complete the `stake_xrd` method. Start by adding a Bucket parameter named `xrd`
        // and a Proof parameter named `member_proof`.
        pub fn stake_xrd(&mut self, xrd: Bucket, member_proof: Proof) {
            // Step 8: Validate the passed proof to make sure its resource address is the same as the one stored on the component's state.
            // Store the ValidatedProof in a variable.
            let validated_proof = member_proof
                .validate_proof(self.member_badge)
                .expect("Invalid proof");

            // Step 9: Store the amount of passed XRD in an `amount` variable
            // and put the xrd bucket inside the component's `xrd_vault`.
            let amount = xrd.amount();
            self.xrd_vault.put(xrd);

            // Step 10: Get the local ID and data of the passed-in proof
            let nft = validated_proof.non_fungible::<MemberData>();
            let local_id = nft.local_id();
            let data = nft.data();

            // Step 11: Add the amount of new XRD token staked to the `amount_staked`
            // field of the NFT metadata and update the data on the ledger (by calling the ResourceManager::update_non_fungible_data method).
            // Do not forget to put the member_manager_badge on the authzone with `.authorize()`.
            // to be able to update the data on the ledger.
            // TODO: find out why we need member_manager_badge and not member_badge here
            // TODO: find out how `update_non_fungible_data` knows which nft exactly to update
            let amount_staked = data.amount_staked + amount;
            self.member_manager_badge.authorize(|| {
                borrow_resource_manager!(self.member_badge).update_non_fungible_data(
                    local_id,
                    "amount_staked",
                    amount_staked,
                );
            });
        }

        // Step 12: Complete the `withdraw` method. Start by adding a Proof parameter named `member_proof`
        // just like with the `stake_xrd` method
        pub fn withdraw(&mut self, member_proof: Proof) -> Bucket {
            // Step 13: validate the proof and store the ValidatedProof in a variable
            let validated_proof = member_proof
                .validate_proof(self.member_badge)
                .expect("Invalid proof");

            // Step 14: Use the ValidatedProof to get the local ID of the member and the data attached to
            // its NFT
            let nft = validated_proof.non_fungible::<MemberData>();
            let local_id = nft.local_id();
            let data = nft.data();

            // Step 15: Store the value of the `amount_staked` field of the NFT's data in a variable
            let amount_staked = data.amount_staked;

            // Step 16: Set the NFT's data `amount_staked` value to 0 and save the changes to the ledger.
            // TODO: find out how does this determine which who's nft to use
            // what if there are multiple nfts on the same account?
            // will a person with no member_manager_badge be able to withdraw?
            self.member_manager_badge.authorize(|| {
                borrow_resource_manager!(self.member_badge).update_non_fungible_data(
                    local_id,
                    "amount_staked",
                    dec!(0),
                );
            });

            // Step 17: Take the amount of tokens that was staked from the `xrd_vault` and return it.
            self.xrd_vault.take(amount_staked)
        }
    }
}
