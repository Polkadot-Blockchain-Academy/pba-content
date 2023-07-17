use crate::mock::*;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		assert!(true);
	});
}
