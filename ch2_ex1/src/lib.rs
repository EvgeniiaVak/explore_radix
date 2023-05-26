use scrypto::prelude::*;

#[derive(ScryptoSbor, Debug)]
enum Vegetable {
    Tomato,
    Carrot,
    Broccoli,
}

#[blueprint]
mod exercise_module {
    struct Exercise1 {
        instantiated_at: u64,
        instantiator_name: String,
        favorite_vegetable: Vegetable,
    }

    impl Exercise1 {
        pub fn instantiate_exercise(instantiator_name: String) -> ComponentAddress {
            Self {
                instantiated_at: Runtime::current_epoch(),
                instantiator_name,
                favorite_vegetable: Vegetable::Tomato,
            }
            .instantiate()
            .globalize()
        }

        pub fn log_data(&self) {
            error!(
                "Instantiated by {} at epoch {}",
                self.instantiator_name, self.instantiated_at
            );
            debug!("Favorite vegetable: {:?}", self.favorite_vegetable);
        }
    }
}
