use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn test_hello() {
    // Setup the environment
    let mut test_runner = TestRunner::builder().build();

    // Create an account
    let (public_key, _private_key, account_component) = test_runner.new_allocated_account();

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());

    // Instantiate
    let manifest = ManifestBuilder::new()
        .call_function(
            package_address,
            "Exercise1",
            "instantiate_exercise",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}\n", receipt);
    let resource_addresses = receipt.expect_commit_success().new_resource_addresses();
    let component = receipt.expect_commit(true).new_component_addresses()[0];

    // Test methods with no args
    for method in ["mint_apple", "get_banana"].iter() {
        let manifest = ManifestBuilder::new()
            .call_method(component, method, manifest_args!())
            .call_method(
                account_component,
                "deposit_batch",
                manifest_args!(ManifestExpression::EntireWorktop),
            )
            .build();
        let receipt = test_runner.execute_manifest_ignoring_fee(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&public_key)],
        );
        println!("{:?}\n", receipt);
        receipt.expect_commit_success();
    }

    // test burn_banana
    // FIXME: Transaction Status: COMMITTED FAILURE:
    // InterpreterError(ScryptoInputSchemaNotMatch("burn_banana",
    // "TraversalError(ValueMismatchWithType(MismatchingType { expected: WellKnown(145), actual: Custom(Address) }))
    // occurred at byte offset 3-31 and value path Exercise1_burn_banana_Input.arg0[0]->Address"))
    let manifest = ManifestBuilder::new()
        .call_method(
            component,
            "burn_banana",
            manifest_args!(resource_addresses[0]),
        )
        .call_method(
            account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}\n", receipt);
    receipt.expect_commit_success();
}
