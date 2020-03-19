use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{build_ext, PaymentToken, Origin, TestRuntime};
use sp_core::H256;

#[test]
fn issue_token_owner_check_ok() {
	build_ext().execute_with(|| {
		//wrong owner of token system
		assert_noop!(PaymentToken::issue_token(Origin::signed(5)), Error::<TestRuntime>::OnlyOwnerCanOperate);
		assert_ok!(PaymentToken::issue_token(Origin::signed(1)));
	})
}

#[test]
fn issue_token_amount_ok() {
	build_ext().execute_with(|| {
		assert_ok!(PaymentToken::issue_token(Origin::signed(1)));
		assert_eq!(PaymentToken::free_balance(1), 21000000);
	})
}

#[test]
fn set_balance_owner_ok() {
	build_ext().execute_with(|| {
		assert_noop!(PaymentToken::set_balance(Origin::signed(5), 5, 1000, 2000), Error::<TestRuntime>::OnlyOwnerCanOperate);
		assert_ok!(PaymentToken::set_balance(Origin::signed(1), 5, 1000, 2000));
	})
}

#[test]
fn set_balance_amount_ok() {
	build_ext().execute_with(|| {
		assert_ok!(PaymentToken::set_balance(Origin::signed(1), 5, 1000, 2000));
		assert_eq!(PaymentToken::free_balance(5), 1000);
		assert_eq!(PaymentToken::reserved_balance(5), 2000);
	})
}

#[test]
fn transfer_from_amount_not_enough() {
	build_ext().execute_with(|| {
		assert_ok!(PaymentToken::set_balance(Origin::signed(1), 5, 1000, 2000));
		//not enough balance
		assert_noop!(PaymentToken::transfer_from(Origin::signed(5), 2, 1001), Error::<TestRuntime>::AmountTooLow);
		assert_ok!(PaymentToken::transfer_from(Origin::signed(5), 2, 1000));
	})
}

#[test]
fn transfer_from_ok() {
	build_ext().execute_with(|| {
		assert_ok!(PaymentToken::set_balance(Origin::signed(1), 5, 1000, 2000));
		assert_ok!(PaymentToken::transfer_from(Origin::signed(5), 2, 600));
		assert_eq!(PaymentToken::free_balance(5), 400);
		assert_eq!(PaymentToken::free_balance(2), 600);
	})
}

#[test]
fn reserve_owner() {
	build_ext().execute_with(|| {
		assert_ok!(PaymentToken::set_balance(Origin::signed(1), 5, 1000, 2000));
		//wrong owner
		assert_noop!(PaymentToken::reserve_token(Origin::signed(2), 5, 600), Error::<TestRuntime>::OnlyOwnerCanOperate);
		assert_noop!(PaymentToken::unreserve_token(Origin::signed(3), 5, 600), Error::<TestRuntime>::OnlyOwnerCanOperate);
		assert_ok!(PaymentToken::reserve_token(Origin::signed(1), 5, 600));
		assert_ok!(PaymentToken::unreserve_token(Origin::signed(1), 5, 600));
	})
}

#[test]
fn reserve_amount() {
	build_ext().execute_with(|| {
		assert_ok!(PaymentToken::set_balance(Origin::signed(1), 5, 1000, 2000));
		//too much
		assert_noop!(PaymentToken::reserve_token(Origin::signed(1), 5, 1001), Error::<TestRuntime>::AmountTooLow);
		//check amount correctness of reserve process
		assert_eq!(PaymentToken::reserved_balance(5), 2000);

		assert_ok!(PaymentToken::reserve_token(Origin::signed(1), 5, 401));
		assert_eq!(PaymentToken::reserved_balance(5), 2401);
		assert_eq!(PaymentToken::free_balance(5), 599);

		assert_ok!(PaymentToken::reserve_token(Origin::signed(1), 5, 599));
		assert_eq!(PaymentToken::reserved_balance(5), 3000);
		assert_eq!(PaymentToken::free_balance(5), 0);

		assert_noop!(PaymentToken::reserve_token(Origin::signed(1), 5, 1), Error::<TestRuntime>::AmountTooLow);
		//not enough to unreserve
		assert_noop!(PaymentToken::unreserve_token(Origin::signed(1), 5, 3001), Error::<TestRuntime>::AmountTooLow);
		//check amount in unreserve process
		assert_ok!(PaymentToken::unreserve_token(Origin::signed(1), 5, 2000));
		assert_eq!(PaymentToken::reserved_balance(5), 1000);
		assert_eq!(PaymentToken::free_balance(5), 2000);

		assert_ok!(PaymentToken::unreserve_token(Origin::signed(1), 5, 1000));
		assert_eq!(PaymentToken::reserved_balance(5), 0);
		assert_eq!(PaymentToken::free_balance(5), 3000);

		assert_noop!(PaymentToken::unreserve_token(Origin::signed(1), 5, 1), Error::<TestRuntime>::AmountTooLow);
	})
}

#[test]
fn total_supply_ok() {
	build_ext().execute_with(|| {
		assert_eq!(PaymentToken::total_supply(), 21000000);
		//set a balance to increase total supply
		assert_ok!(PaymentToken::set_balance(Origin::signed(1), 5, 1000000, 2000000));
		assert_eq!(PaymentToken::total_supply(), 24000000);
		//set another account's to increase total supply
		assert_ok!(PaymentToken::set_balance(Origin::signed(1), 2, 100000, 200000));
		assert_eq!(PaymentToken::total_supply(), 24300000);

		//reduce free and increase reserved to increase total supply
		assert_ok!(PaymentToken::set_balance(Origin::signed(1), 5, 0, 4000000));
		assert_eq!(PaymentToken::total_supply(), 25300000);

		//increate free and decrease reserved to decrease total supply
		assert_ok!(PaymentToken::set_balance(Origin::signed(1), 5, 1000000, 2000000));
		assert_eq!(PaymentToken::total_supply(), 24300000);
	})
}