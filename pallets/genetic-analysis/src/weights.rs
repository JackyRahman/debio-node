//! Autogenerated weights for genetic_analysis
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-18, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=genetic-analysis
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/genetic-analysis/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for genetic_analysis.
pub trait WeightInfo { 
	fn reject_genetic_analysis() -> Weight; 
	fn process_genetic_analysis() -> Weight; 
	fn submit_genetic_analysis() -> Weight; 
}

/// Weights for genetic_analysis using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>); 
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> { 
	// Storage: GeneticAnalysis GeneticAnalysisStorage (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrders (r:1 w:1) 
	// Storage: GeneticAnalysisOrders PendingGeneticAnalysisOrdersBySeller (r:1 w:1) 
	fn reject_genetic_analysis() -> Weight { 
		114_752_000_u64 
			.saturating_add(T::DbWeight::get().reads(4_u64)) 
			.saturating_add(T::DbWeight::get().writes(3_u64)) 
	}
	// Storage: GeneticAnalysis GeneticAnalysisStorage (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	fn process_genetic_analysis() -> Weight { 
		18_511_000_u64 
			.saturating_add(T::DbWeight::get().reads(2_u64)) 
			.saturating_add(T::DbWeight::get().writes(1_u64)) 
	}
	// Storage: GeneticAnalysis GeneticAnalysisStorage (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	fn submit_genetic_analysis() -> Weight { 
		31_029_000_u64 
			.saturating_add(T::DbWeight::get().reads(2_u64)) 
			.saturating_add(T::DbWeight::get().writes(1_u64)) 
	}
}

// For backwards compatibility and tests
impl WeightInfo for () { 
	// Storage: GeneticAnalysis GeneticAnalysisStorage (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrders (r:1 w:1) 
	// Storage: GeneticAnalysisOrders PendingGeneticAnalysisOrdersBySeller (r:1 w:1) 
	fn reject_genetic_analysis() -> Weight { 
		114_752_000_u64
			.saturating_add(RocksDbWeight::get().reads(4_u64)) 
			.saturating_add(RocksDbWeight::get().writes(3_u64)) 
	} 
	// Storage: GeneticAnalysis GeneticAnalysisStorage (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	fn process_genetic_analysis() -> Weight { 
		18_511_000_u64
			.saturating_add(RocksDbWeight::get().reads(2_u64)) 
			.saturating_add(RocksDbWeight::get().writes(1_u64)) 
	} 
	// Storage: GeneticAnalysis GeneticAnalysisStorage (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	fn submit_genetic_analysis() -> Weight { 
		31_029_000_u64
			.saturating_add(RocksDbWeight::get().reads(2_u64)) 
			.saturating_add(RocksDbWeight::get().writes(1_u64)) 
	} 
}
