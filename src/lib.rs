use scrypto::prelude::*;

#[derive(ScryptoSbor, Debug)]
pub enum Language {
    English,
    Spanish,
    French,
}

#[derive(ScryptoSbor, NonFungibleData, Debug)]
pub struct HelloNftData {
    greeting: String,
    language: Language,

    #[mutable]
    usage_count: u64,
}

impl HelloNftData {
    pub fn new(greeting: String, language: Language) -> Self {
        Self {
            greeting,
            language,
            usage_count: 0,
        }
    }

    pub fn greet(&mut self) {
        info!("{} ({} times)", self.greeting, self.usage_count);
        self.usage_count += 1;
    }
}

#[blueprint]
mod hello {
    struct Hello {
        // Define what resources and data will be managed by Hello components
        token_vault: Vault,
        nft_vault: Vault,
        xrd_vault: Vault,
    }

    impl Hello {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate_hello() -> ComponentAddress {
            // Create a new token called "HelloToken," with a fixed supply of 1000, and put that supply into a bucket
            let token_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "HelloToken")
                .metadata("symbol", "HT")
                .mint_initial_supply(1000);

            // Create a new mintable non-fungible token called "HelloNFT", that holders can use changing the usage_count
            let nft_bucket: Bucket = ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", "HelloNFT")
                // TODO: learn if this gives ability to edit other people's NFTs
                .updateable_non_fungible_data(rule!(allow_all), LOCKED)
                .mintable(rule!(allow_all), LOCKED)
                .mint_initial_supply(vec![
                    HelloNftData::new("Hello!".into(), Language::English),
                    HelloNftData::new("Howdy!".into(), Language::English),
                    HelloNftData::new("Ahoy!".into(), Language::English),
                    HelloNftData::new("Hola!".into(), Language::Spanish),
                    HelloNftData::new("Bonjour!".into(), Language::French),
                ]);

            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            Self {
                token_vault: Vault::with_bucket(token_bucket),
                nft_vault: Vault::with_bucket(nft_bucket),
                xrd_vault: Vault::new(RADIX_TOKEN),
            }
            .instantiate()
            .globalize()
        }

        // This is a method, because it needs a reference to self.  Methods can only be called on components
        pub fn free_token(&mut self) -> Bucket {
            info!(
                "My balance is: {} HelloToken. Now giving away a token!",
                self.token_vault.amount()
            );
            // If the semi-colon is omitted on the last line, the last value seen is automatically returned
            // In this case, a bucket containing 1 HelloToken is returned
            self.token_vault.take(1)
        }

        pub fn request_amount(&mut self, amount: Decimal) {
            info!("Moving {} HelloToken to another account", amount);
            let bucket = self.token_vault.take(amount);
            assert!(
                bucket.amount() > 100.into(),
                "You only requested {}HT. Request at least 100",
                bucket.amount()
            );
        }

        pub fn purchase_nft(&mut self) -> Bucket {
            info!("Purchasing an NFT");
            let bucket = self.nft_vault.take(1);
            assert!(bucket.amount() > 0.into(), "No more NFTs left to purchase");
            bucket
        }

        pub fn greet_nft(&mut self, bucket: Bucket) {
            let nft = bucket.non_fungible::<HelloNftData>();
            nft.data().greet()
        }
    }
}
