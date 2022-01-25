use std::ops::Mul;
use crate::{mock::*, Error};
use crate::mock::Event as TestEvent;
use frame_support::{assert_noop, assert_ok};
use sp_core::H160;
use sp_runtime::{Permill,Percent};


#[test]
fn it_works_for_Permill_value() {
    new_test_ext().execute_with(|| {
		// 这里有个讨论： https://github.com/paritytech/substrate/issues/2441
		let s_2679: u32 =2679;
        let a_parts = Permill::from_parts(s_2679); // a_parts:Permill(2679)
        let b_percent = Permill::from_percent(s_2679); // b_percent:Permill(1000000)

		let c_10000 =Permill::from_parts(10000u32);  // c_10000:Permill(10000)

		let d = a_parts.mul(c_10000);   // d:Permill(26)
		let e =b_percent.mul(c_10000);  // e:Permill(10000)

		println!("a_parts:{:?}, b_percent:{:?}, c_10000:{:?}, d:{:?}, e:{:?}",a_parts,b_percent,c_10000,d,e);

		let f_permill =Permill::from_percent(30000);
		let g_permill =f_permill *2000 as u32;

		eprintln!("f:{:?},g:{}",f_permill, g_permill);

		//System::assert_has_event(TestEvent::TemplateModule(Event::<Test>::SomethingStored(42, 1)));
    });
}

#[test]
fn it_works_for_Percent_value() {
	new_test_ext().execute_with(|| {

		let s_2679: u8 =80;
		let a_parts = Percent::from_parts(s_2679);
		let b_percent = Percent::from_percent(s_2679);

		let c_10000 =Percent::from_parts(100u8);

		let d = a_parts.mul(c_10000);
		let e =b_percent.mul(c_10000);

		println!("a_parts:{:?}, b_percent:{:?}, c_10000:{:?}, d:{:?}, e:{:?}",a_parts,b_percent,c_10000,d,e);

		//System::assert_has_event(TestEvent::TemplateModule(Event::<Test>::SomethingStored(42, 1)));
	});
}


#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));


		//System::assert_has_event(TestEvent::TemplateModule(Event::<Test>::SomethingStored(42, 1)));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

#[test]
fn correct_add_validator_value() {
	new_test_ext().execute_with(|| {

		assert_ok!(TemplateModule::add_validator(Origin::signed(1), 2 as u64));
		let vlist =TemplateModule::validators();
		println!("vlist:{:?}",vlist);
		assert_eq!(vlist, vec![2]);
	});
}


#[test]
fn correct_remove_validator_value() {
	new_test_ext().execute_with(|| {

		assert_ok!(TemplateModule::add_validator(Origin::signed(1), 2 as u64));
		let vlist =TemplateModule::validators();
		println!("vlist:{:?}",vlist);

		assert_ok!(TemplateModule::remove_validator(Origin::signed(1), 2 as u64));

		let vlist =TemplateModule::validators();
		let zero:Vec<u64> =vec![];
		assert_eq!(vlist, zero);

	});
}

/*#[test]
fn correct_remove_dup_validator_value() {
	new_test_ext().execute_with(|| {

		let v20= "hello".as_bytes();
		let a1 =H160([1;20]);
		let b1 =H160([2u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]);
		let b2 =H160([1u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]);
		let b3 =H160([2u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]);
		assert_ok!(TemplateModule::add_validator(Origin::signed(a1), b1));
		assert_ok!(TemplateModule::add_validator(Origin::signed(a1), b2));
		//assert_ok!(TemplateModule::add_validator(Origin::signed(a1), b3)); // 这里是为了测试重复数据 查找的问题。原来用的binary_search的，有问题
		let vlist =TemplateModule::validators();
		println!("vlist:{:?}",vlist);

		assert_ok!(TemplateModule::remove_validator(Origin::signed(a1), b1));

		assert_ok!(TemplateModule::remove_validator(Origin::signed(a1), b3));

		let vlist =TemplateModule::validators();

		println!("vlist:{:?}",vlist);
		assert_eq!(vlist, vec![b2]);

	});
}


#[test]
fn correct_add_dup_validator_value() {
	new_test_ext().execute_with(|| {

		let v20= "hello".as_bytes();
		let a1 =H160([1;20]);
		let b1 =H160([2u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]);
		let b2 =H160([1u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]);
		let b3 =H160([2u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]);
		assert_ok!(TemplateModule::add_validator(Origin::signed(a1), b1));
		assert_ok!(TemplateModule::add_validator(Origin::signed(a1), b2));
		assert_noop!(TemplateModule::add_validator(Origin::signed(a1), b3),Error::<Test>::Duplicated);
		let vlist =TemplateModule::validators();
		println!("vlist:{:?}",vlist);

	});
}*/


#[test]
fn correct_remove_dup_u64_validator_value() {
	new_test_ext().execute_with(|| {



		let v20= "hello".as_bytes();
		let a1 =1 as u64;
		let b1 = 2 as u64;
		let b2 =3 as u64;
		let b3 = 2 as u64;
		assert_ok!(TemplateModule::add_validator(Origin::signed(a1), b1));
		assert_ok!(TemplateModule::add_validator(Origin::signed(a1), b2));
		//assert_ok!(TemplateModule::add_validator(Origin::signed(a1), b3));
		let vlist =TemplateModule::validators();
		println!("vlist:{:?}",vlist);

		assert_ok!(TemplateModule::remove_validator(Origin::signed(a1), b1));

		assert_ok!(TemplateModule::remove_validator(Origin::signed(a1), b3));

		let vlist =TemplateModule::validators();

		println!("vlist:{:?}",vlist);
		//assert_eq!(vlist, Some(vec![3]));

	});
}



/*#[test]
fn correct_remove_validator_value_no_validator() {
	new_test_ext().execute_with(|| {
		assert_noop!(TemplateModule::remove_validator(Origin::signed(1), 2 as u64), Error::<Test>::NoValidators);
	});
}*/