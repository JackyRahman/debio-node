//! Autogenerated weights for labs
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-13, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=labs
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/labs/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for labs.
pub trait WeightInfo {
	fn register_lab() -> Weight;
	fn update_lab() -> Weight;
	fn update_lab_verification_status() -> Weight;
	fn deregister_lab() -> Weight;
	fn update_admin_key() -> Weight;
}

/// Weights for labs using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Labs Labs (r:1 w:1)
	// Storage: Labs LabsByCountryRegionCity (r:1 w:1)
	// Storage: Labs LabCount (r:1 w:1)
	// Storage: Labs LabCountByCountryRegionCity (r:1 w:1)
	fn register_lab() -> Weight {
		58_508_000_u64
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Labs Labs (r:1 w:1)
	// Storage: Labs LabsByCountryRegionCity (r:2 w:2)
	// Storage: Labs LabCountByCountryRegionCity (r:2 w:2)
	fn update_lab() -> Weight {
		89_151_000_u64
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: Labs LabVerifierKey (r:1 w:0)
	// Storage: Labs Labs (r:1 w:1)
	fn update_lab_verification_status() -> Weight {
		38_023_000_u64
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Labs Labs (r:1 w:1)
	// Storage: Labs LabsByCountryRegionCity (r:1 w:1)
	// Storage: Labs LabCountByCountryRegionCity (r:1 w:1)
	// Storage: Labs LabCount (r:1 w:1)
	fn deregister_lab() -> Weight {
		66_150_000_u64
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Labs LabVerifierKey (r:1 w:1)
	fn update_admin_key() -> Weight {
		22_008_000_u64
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Labs Labs (r:1 w:1)
	// Storage: Labs LabsByCountryRegionCity (r:1 w:1)
	// Storage: Labs LabCount (r:1 w:1)
	// Storage: Labs LabCountByCountryRegionCity (r:1 w:1)
	fn register_lab() -> Weight {
		58_508_000_u64
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	// Storage: Labs Labs (r:1 w:1)
	// Storage: Labs LabsByCountryRegionCity (r:2 w:2)
	// Storage: Labs LabCountByCountryRegionCity (r:2 w:2)
	fn update_lab() -> Weight {
		89_151_000_u64
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	// Storage: Labs LabVerifierKey (r:1 w:0)
	// Storage: Labs Labs (r:1 w:1)
	fn update_lab_verification_status() -> Weight {
		38_023_000_u64
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: Labs Labs (r:1 w:1)
	// Storage: Labs LabsByCountryRegionCity (r:1 w:1)
	// Storage: Labs LabCountByCountryRegionCity (r:1 w:1)
	// Storage: Labs LabCount (r:1 w:1)
	fn deregister_lab() -> Weight {
		66_150_000_u64
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	// Storage: Labs LabVerifierKey (r:1 w:1)
	fn update_admin_key() -> Weight {
		22_008_000_u64
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
