use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{build_ext, HashPowerContract, Origin};
use sp_core::H256;

#[test]
fn it_works() {
	build_ext().execute_with(|| {
		assert!(true);
	})
}

#[test]
fn test_create_single_contract_ok() {
	build_ext().execute_with(|| {
		//no contract before we created one
		assert_eq!(HashPowerContract::contracts_count(), 0);
		assert_ok!(HashPowerContract::create_contract(Origin::signed(5), 1234, 123, 456, 1));
		//count should be 1
		assert_eq!(HashPowerContract::contracts_count(), 1);
	})
}

#[test]
fn test_create_contracts_storage_ok() {
	build_ext().execute_with(|| {
		assert_ok!(HashPowerContract::create_contract(Origin::signed(5), 1234, 123, 456, 1));
		//count should be 1
		assert_eq!(HashPowerContract::contracts_count(), 1);
		assert_ok!(HashPowerContract::create_contract(
			Origin::signed(10),
			4321,
			123,
			999,
			2
		));
		//count should be 2
		assert_eq!(HashPowerContract::contracts_count(), 2);
		assert_ok!(HashPowerContract::create_contract(Origin::signed(5), 1234, 000, 456, 3));
		//count should be 3
		assert_eq!(HashPowerContract::contracts_count(), 3);

		let contract_hash_one = HashPowerContract::contract_at_index(0);
		let contract_hash_two = HashPowerContract::contract_at_index(1);
		let contract_hash_three = HashPowerContract::contract_at_index(2);
		//owner accountid
		assert_eq!(HashPowerContract::owner_of(contract_hash_one), Some(5));
		assert_eq!(HashPowerContract::owner_of(contract_hash_two), Some(10));
		assert_eq!(HashPowerContract::owner_of(contract_hash_three), Some(5));

		//index in all contracts array
		assert_eq!(HashPowerContract::index_of(contract_hash_one), 0);
		assert_eq!(HashPowerContract::index_of(contract_hash_two), 1);
		assert_eq!(HashPowerContract::index_of(contract_hash_three), 2);

		//owned contracs
		assert_eq!(HashPowerContract::contracts_of((5, 0)), contract_hash_one);
		assert_eq!(HashPowerContract::contracts_of((10, 0)), contract_hash_two);
		assert_eq!(HashPowerContract::contracts_of((5, 1)), contract_hash_three);

		//owned contracts count
		assert_eq!(HashPowerContract::owned_contract_count(5), 2);
		assert_eq!(HashPowerContract::owned_contract_count(10), 1);
		assert_eq!(HashPowerContract::owned_contract_count(2), 0); //random account id

		//owned contracts index
		assert_eq!(HashPowerContract::owned_index_of(contract_hash_one), 0);
		assert_eq!(HashPowerContract::owned_index_of(contract_hash_two), 0);
		assert_eq!(HashPowerContract::owned_index_of(contract_hash_three), 1);
	})
}

#[test]
fn test_transfer_single_contract_ok() {
	build_ext().execute_with(|| {
		assert_ok!(HashPowerContract::create_contract(Origin::signed(5), 1234, 123, 456, 1));

		let contract_hash = HashPowerContract::contract_at_index(0);

		//count should be 1
		assert_eq!(HashPowerContract::contracts_count(), 1);
		//owner is still original owner for now
		assert_eq!(HashPowerContract::owner_of(contract_hash), Some(5));

		assert_ok!(HashPowerContract::transfer_contract(
			Origin::signed(5),
			4,
			contract_hash
		));

		//count should be still 1
		assert_eq!(HashPowerContract::contracts_count(), 1);
		//owner should be new owner
		assert_eq!(HashPowerContract::owner_of(contract_hash), Some(4));
	})
}

#[test]
fn test_transfer_contract_storage_ok() {
	build_ext().execute_with(|| {
		assert_ok!(HashPowerContract::create_contract(Origin::signed(5), 1234, 123, 456, 1));
		assert_ok!(HashPowerContract::create_contract(
			Origin::signed(10),
			4321,
			123,
			999,
			2
		));
		assert_ok!(HashPowerContract::create_contract(Origin::signed(5), 1234, 000, 456, 3));
		assert_ok!(HashPowerContract::create_contract(Origin::signed(5), 1234, 000, 456, 4));

		let contract_hash_one = HashPowerContract::contract_at_index(0);
		let contract_hash_two = HashPowerContract::contract_at_index(1);
		let contract_hash_three = HashPowerContract::contract_at_index(2);
		let contract_hash_four = HashPowerContract::contract_at_index(3);

		assert_ok!(HashPowerContract::transfer_contract(
			Origin::signed(5),
			10,
			contract_hash_three
		));

		//owner accountid
		assert_eq!(HashPowerContract::owner_of(contract_hash_one), Some(5));
		assert_eq!(HashPowerContract::owner_of(contract_hash_two), Some(10));
		assert_eq!(HashPowerContract::owner_of(contract_hash_three), Some(10));
		assert_eq!(HashPowerContract::owner_of(contract_hash_four), Some(5));

		//index in all contracts array
		assert_eq!(HashPowerContract::index_of(contract_hash_one), 0);
		assert_eq!(HashPowerContract::index_of(contract_hash_two), 1);
		assert_eq!(HashPowerContract::index_of(contract_hash_three), 2);
		assert_eq!(HashPowerContract::index_of(contract_hash_four), 3);

		//owned contracs
		assert_eq!(HashPowerContract::contracts_of((5, 0)), contract_hash_one);
		//assert_eq!(HashPowerContract::contracts_of((5, 2)), contract_hash_four);
		assert_eq!(HashPowerContract::contracts_of((5, 1)), contract_hash_four);
		assert_eq!(HashPowerContract::contracts_of((10, 0)), contract_hash_two);
		assert_eq!(HashPowerContract::contracts_of((10, 1)), contract_hash_three);

		//owned contracts count
		assert_eq!(HashPowerContract::owned_contract_count(5), 2);
		assert_eq!(HashPowerContract::owned_contract_count(10), 2);
		assert_eq!(HashPowerContract::owned_contract_count(2), 0); //random account id

		//owned contracts index
		assert_eq!(HashPowerContract::owned_index_of(contract_hash_one), 0);
		assert_eq!(HashPowerContract::owned_index_of(contract_hash_two), 0);
		assert_eq!(HashPowerContract::owned_index_of(contract_hash_three), 1);
		assert_eq!(HashPowerContract::owned_index_of(contract_hash_four), 1);
	})
}

#[test]
fn test_create_contract_exist_fail() {
	build_ext().execute_with(|| {
		assert_ok!(HashPowerContract::create_contract(Origin::signed(5), 555, 123, 456, 1));

		let contract_id = HashPowerContract::contract_at_index(0);
		let contract = HashPowerContract::contract(contract_id);

		assert_noop!(
			HashPowerContract::mint_contract(&5, &contract_id, &contract),
			"contract already exits"
		);
	})
}

#[test]
fn test_transfer_contract_from_wrong_owner_fail() {
	build_ext().execute_with(|| {
		assert_ok!(HashPowerContract::create_contract(Origin::signed(5), 555, 123, 456, 1));
		let contract_id = HashPowerContract::contract_at_index(0);
		assert_noop!(
			HashPowerContract::transfer_contract(Origin::signed(4), 6, contract_id),
			"can only transfer your own contract"
		);
	})
}

#[test]
fn test_transfer_to_owner_fail() {
	build_ext().execute_with(|| {
		assert_ok!(HashPowerContract::create_contract(Origin::signed(5), 555, 123, 456, 1));
		let contract_id = HashPowerContract::contract_at_index(0);
		assert_noop!(
			HashPowerContract::transfer_contract(Origin::signed(5), 5, contract_id),
			"can't transfer contract to yourself"
		);
	})
}

#[test]
fn test_tranfer_contract_nonexist_fail() {
	build_ext().execute_with(|| {
		assert_ok!(HashPowerContract::create_contract(Origin::signed(5), 555, 123, 456, 1));
		let contract_id = HashPowerContract::contract_at_index(0);
		assert_noop!(
			HashPowerContract::transfer_from(&5, &6, &H256::from_low_u64_be(100)),
			"this contract doesn't exist"
		);
	})
}
