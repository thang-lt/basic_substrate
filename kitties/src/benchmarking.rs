//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {
		let caller: T::AccountId = whitelisted_caller();
		let to = account("recicer",0,0);
		let dna = vec![1,2,3];
		let _ = Kitties::<T>::create_kitty(RawOrigin::Signed(caller).into(), dna)
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
