use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn test_magic_card() {
    // Set up environment.
    let mut test_runner = TestRunner::builder().build();

    // Create an account
    let (public_key, _private_key, account_component) = test_runner.new_allocated_account();

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());

    // Test the `instantiate_component` function.
    let transaction1 = ManifestBuilder::new()
        .call_function(
            package_address,
            "HelloNft",
            "instantiate_component",
            manifest_args!(),
        )
        .build();
    let receipt1 = test_runner.execute_manifest_ignoring_fee(
        transaction1,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}\n", receipt1);
    receipt1.expect_commit_success();

    // Test the `buy_special_card` method.
    let component = receipt1
        .expect_commit(true).new_component_addresses()[0];

    let transaction2 = ManifestBuilder::new()
        .withdraw_from_account(account_component, RADIX_TOKEN,  dec!("666"))
        .take_from_worktop(RADIX_TOKEN, dec!("666"), |builder, bucket| {
            builder.call_method(
                component,
                "buy_special_card",
                manifest_args!(NonFungibleLocalId::integer(2u64), bucket),
            )
        })
        .call_method(
            account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt2 = test_runner.execute_manifest_ignoring_fee(
        transaction2,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}\n", receipt2);
    receipt2.expect_commit_success();

    // Test the `buy_special_card` method.
    let component = receipt1
        .expect_commit(true).new_component_addresses()[0];

    let transaction3 = ManifestBuilder::new()
        .withdraw_from_account(account_component, RADIX_TOKEN, dec!("500"))
        .take_from_worktop(RADIX_TOKEN, dec!("500"), |builder, bucket| {
            builder.call_method(component, "buy_random_card", manifest_args!(bucket))
        })
        .call_method(
            account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt3 = test_runner.execute_manifest_ignoring_fee(
        transaction3,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}\n", receipt3);
    receipt3.expect_commit_success();
}
