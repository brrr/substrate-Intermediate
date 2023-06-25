//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
// use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_support::{ BoundedVec};
// use sp_runtime::{
//     transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
//     RuntimeDebug,
// };

// #[benchmarks]
mod benchmarks {
    use super::*;
    use frame_support::log;
    use sp_std::any::TypeId;
    // #[benchmark]
    // fn do_something() {
    //     let value = 100u32.into();
    //     let caller: T::AccountId = whitelisted_caller();
    //     #[extrinsic_call]
    //     do_something(RawOrigin::Signed(caller), value);
    //
    //     assert_eq!(Something::<T>::get(), Some(value));
    // }

    // #[benchmark]
    // fn create_claim(){
    //
    // }

    benchmarks! {
        create_claim {
            let d in 0 .. T::MaxClaimLength::get();
            log::info!("============>{:?}", d);
            // let claim = vec![0; d as usize];
            let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
            log::info!("claim============>{:?}", claim.len());
            let caller: T::AccountId = whitelisted_caller();
        }: _(RawOrigin::Signed(caller), claim)

        revoke_claim {
            let d in 0 .. T::MaxClaimLength::get();
            let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
            let caller: T::AccountId = whitelisted_caller();
            assert!(Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone()).is_ok());
        }: _(RawOrigin::Signed(caller.clone()), claim.clone())

        transfer_claim {
            let d in 0 .. T::MaxClaimLength::get();
            let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
            let caller: T::AccountId = whitelisted_caller();
            let target: T::AccountId = account("another_seed", 0, 0);
            assert!(Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone()).is_ok());
        }: _(RawOrigin::Signed(caller), target, claim)
        // Log::info
        impl_benchmark_test_suite!(PoeModule, crate::mock::new_test_ext(), crate::mock::Test);
    }

    // #[benchmark]
    // fn cause_error() {
    //     Something::<T>::put(100u32);
    //     let caller: T::AccountId = whitelisted_caller();
    //     #[extrinsic_call]
    //     cause_error(RawOrigin::Signed(caller));
    //
    //     assert_eq!(Something::<T>::get(), Some(101u32));
    // }

    // impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
