use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
struct MemberData {
    #[mutable]
    amount_staked: Decimal,
}

#[blueprint]
mod exercise_module {
    struct Exercise1 {
        xrd_vault: Vault,
        manager_badge: Vault,
        member_resource_address: ResourceAddress,
    }

    impl Exercise1 {
        pub fn instantiate_exercise() -> ComponentAddress {
            // Create the badge that will be held within the instantiated component
            // and allow it to mint member badges
            let member_manager_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);

            // Define the member badge non fungible resource.
            // The IDs of this resource will be of type `NonFungibleIdType::UUID`
            // It is mintable and the individual metadata can be updated if a proof
            // of the member_manager badge is presented.
            let member_badge = ResourceBuilder::new_uuid_non_fungible::<MemberData>()
                .metadata("name", "Member Badge")
                .mintable(
                    rule!(require(member_manager_badge.resource_address())),
                    rule!(deny_all),
                )
                .updateable_non_fungible_data(
                    rule!(require(member_manager_badge.resource_address())),
                    rule!(deny_all),
                )
                .create_with_no_initial_supply();

            Self {
                xrd_vault: Vault::new(RADIX_TOKEN),
                manager_badge: Vault::with_bucket(member_manager_badge),
                member_resource_address: member_badge,
            }
            .instantiate()
            .globalize()
        }

        // Allow people to get access to this component by
        // getting a member badge
        pub fn become_member(&self) -> Bucket {
            let data = MemberData {
                amount_staked: Decimal::zero(),
            };

            self.manager_badge.authorize(|| {
                borrow_resource_manager!(self.member_resource_address).mint_uuid_non_fungible(data)
            })
        }

        // Allow people presenting a member badge to stake XRD into
        // a vault on this component. It updates the amount that is staked on the
        // member NFT metadata.
        pub fn stake_xrd(&mut self, xrd: Bucket, member_proof: Proof) {
            let member_proof = member_proof
                .validate_proof(self.member_resource_address)
                .expect("Wrong proof provided!");
            let amount = xrd.amount();

            self.xrd_vault.put(xrd);

            // Update the amount staked on the member NFT
            let non_fungible: NonFungible<MemberData> = member_proof.non_fungible();
            let member_data = non_fungible.data();

            let current_stake = member_data.amount_staked;

            let new_stake = current_stake + amount;

            self.manager_badge.authorize(|| {
                borrow_resource_manager!(self.member_resource_address).update_non_fungible_data(
                    &non_fungible.local_id(),
                    "amount_staked",
                    new_stake,
                );
            });
        }

        // Allow people presenting their member badge to withdraw the XRD
        // they previously staked. The amount staked stored on their member NFT set of 0.
        pub fn withdraw(&mut self, member_proof: Proof) -> Bucket {
            let member_proof = member_proof
                .validate_proof(self.member_resource_address)
                .expect("Wrong proof provided!");

            let non_fungible: NonFungible<MemberData> = member_proof.non_fungible();
            let member_data = non_fungible.data();

            let amount_to_withdraw = member_data.amount_staked;

            // Update the amount staked to 0
            let new_stake = Decimal::zero();

            self.manager_badge.authorize(|| {
                borrow_resource_manager!(self.member_resource_address).update_non_fungible_data(
                    &non_fungible.local_id(),
                    "amount_staked",
                    new_stake,
                );
            });

            // Return their staked XRD
            self.xrd_vault.take(amount_to_withdraw)
        }
    }
}
