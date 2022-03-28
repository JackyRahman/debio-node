//! Autogenerated weights for genetic_analysis_orders
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-28, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=genetic-analysis-orders
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/genetic-analysis-orders/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for genetic_analysis_orders.
pub trait WeightInfo {
	fn create_genetic_analysis_order() -> Weight;
	fn cancel_genetic_analysis_order() -> Weight;
	fn set_genetic_analysis_order_paid() -> Weight;
	fn fulfill_genetic_analysis_order() -> Weight;
	fn set_genetic_analysis_order_refunded() -> Weight;
	fn update_escrow_key() -> Weight;
}

/// Weights for genetic_analysis_orders using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: GeneticAnalystServices GeneticAnalystServices (r:1 w:0)
	// Storage: GeneticAnalysts GeneticAnalysts (r:1 w:0)
	// Storage: GeneticData GeneticDataById (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: GeneticAnalysis GeneticAnalysisStorage (r:1 w:1)
	// Storage: GeneticAnalysis GeneticAnalysisByOwner (r:1 w:1)
	// Storage: GeneticAnalysis GeneticAnalysisByGeneticAnalyst (r:1 w:1)
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrdersBySeller (r:1 w:1)
	// Storage: GeneticAnalysisOrders PendingGeneticAnalysisOrdersBySeller (r:1 w:1)
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrdersByCustomer (r:1 w:1)
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrders (r:0 w:1)
	// Storage: GeneticAnalysisOrders LastGeneticAnalysisOrderByCustomer (r:0 w:1)
	fn create_genetic_analysis_order() -> Weight {
		129_797_000_u64
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrders (r:1 w:1)
	// Storage: GeneticAnalysis GeneticAnalysisStorage (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn cancel_genetic_analysis_order() -> Weight {
		49_122_000_u64
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: GeneticAnalysisOrders EscrowKey (r:1 w:0)
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrders (r:1 w:1)
	// Storage: System Account (r:2 w:0)
	// Storage: GeneticAnalysisOrders PalletAccount (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: GeneticAnalysisOrders TotalEscrowAmount (r:0 w:1)
	fn set_genetic_analysis_order_paid() -> Weight {
		63_591_000_u64
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: GeneticAnalysisOrders EscrowKey (r:1 w:0)
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrders (r:1 w:1)
	// Storage: GeneticAnalysis GeneticAnalysisStorage (r:1 w:0)
	// Storage: GeneticAnalysisOrders PalletAccount (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: GeneticAnalysisOrders TotalEscrowAmount (r:0 w:1)
	fn fulfill_genetic_analysis_order() -> Weight {
		70_090_000_u64
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: GeneticAnalysisOrders EscrowKey (r:1 w:0)
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrders (r:1 w:1)
	// Storage: GeneticAnalysis GeneticAnalysisStorage (r:1 w:0)
	// Storage: GeneticAnalysisOrders PalletAccount (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: GeneticAnalysisOrders TotalEscrowAmount (r:0 w:1)
	fn set_genetic_analysis_order_refunded() -> Weight {
		71_448_000_u64
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: GeneticAnalysisOrders EscrowKey (r:1 w:1)
	fn update_escrow_key() -> Weight {
		23_644_000_u64
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: GeneticAnalystServices GeneticAnalystServices (r:1 w:0)
	// Storage: GeneticAnalysts GeneticAnalysts (r:1 w:0)
	// Storage: GeneticData GeneticDataById (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: GeneticAnalysis GeneticAnalysisStorage (r:1 w:1)
	// Storage: GeneticAnalysis GeneticAnalysisByOwner (r:1 w:1)
	// Storage: GeneticAnalysis GeneticAnalysisByGeneticAnalyst (r:1 w:1)
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrdersBySeller (r:1 w:1)
	// Storage: GeneticAnalysisOrders PendingGeneticAnalysisOrdersBySeller (r:1 w:1)
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrdersByCustomer (r:1 w:1)
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrders (r:0 w:1)
	// Storage: GeneticAnalysisOrders LastGeneticAnalysisOrderByCustomer (r:0 w:1)
	fn create_genetic_analysis_order() -> Weight {
		129_797_000_u64
			.saturating_add(RocksDbWeight::get().reads(12_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrders (r:1 w:1)
	// Storage: GeneticAnalysis GeneticAnalysisStorage (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn cancel_genetic_analysis_order() -> Weight {
		49_122_000_u64
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: GeneticAnalysisOrders EscrowKey (r:1 w:0)
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrders (r:1 w:1)
	// Storage: System Account (r:2 w:0)
	// Storage: GeneticAnalysisOrders PalletAccount (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: GeneticAnalysisOrders TotalEscrowAmount (r:0 w:1)
	fn set_genetic_analysis_order_paid() -> Weight {
		63_591_000_u64
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: GeneticAnalysisOrders EscrowKey (r:1 w:0)
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrders (r:1 w:1)
	// Storage: GeneticAnalysis GeneticAnalysisStorage (r:1 w:0)
	// Storage: GeneticAnalysisOrders PalletAccount (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: GeneticAnalysisOrders TotalEscrowAmount (r:0 w:1)
	fn fulfill_genetic_analysis_order() -> Weight {
		70_090_000_u64
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: GeneticAnalysisOrders EscrowKey (r:1 w:0)
	// Storage: GeneticAnalysisOrders GeneticAnalysisOrders (r:1 w:1)
	// Storage: GeneticAnalysis GeneticAnalysisStorage (r:1 w:0)
	// Storage: GeneticAnalysisOrders PalletAccount (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: GeneticAnalysisOrders TotalEscrowAmount (r:0 w:1)
	fn set_genetic_analysis_order_refunded() -> Weight {
		71_448_000_u64
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: GeneticAnalysisOrders EscrowKey (r:1 w:1)
	fn update_escrow_key() -> Weight {
		23_644_000_u64
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
