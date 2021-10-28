use super::*;

#[allow(unused)]
use crate::Pallet as Labs;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {    
	set_eth_address {
        let eth_address = T::EthereumAddress::default();
		let caller: T::AccountId = whitelisted_caller();
	}: set_eth_address(
        RawOrigin::Signed(caller),
        eth_address
    )
}

impl_benchmark_test_suite! {Labs, crate::mock::ExternalityBuilder::build(), crate::mock::Test}