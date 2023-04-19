// Template adopted from https://github.com/paritytech/substrate/blob/master/.maintain/frame-weight-template.hbs

//! Autogenerated weights for pallet_nonfungible
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-30, STEPS: `50`, REPEAT: 80, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/release/unique-collator
// benchmark
// pallet
// --pallet
// pallet-nonfungible
// --wasm-execution
// compiled
// --extrinsic
// *
// --template=.maintain/frame-weight-template.hbs
// --steps=50
// --repeat=80
// --heap-pages=4096
// --output=./pallets/nonfungible/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_nonfungible.
pub trait WeightInfo {
	fn create_item() -> Weight;
	fn create_multiple_items(b: u32, ) -> Weight;
	fn create_multiple_items_ex(b: u32, ) -> Weight;
	fn burn_item() -> Weight;
	fn burn_recursively_self_raw() -> Weight;
	fn burn_recursively_breadth_plus_self_plus_self_per_each_raw(b: u32, ) -> Weight;
	fn transfer_raw() -> Weight;
	fn approve() -> Weight;
	fn approve_from() -> Weight;
	fn check_allowed_raw() -> Weight;
	fn burn_from() -> Weight;
	fn set_token_property_permissions(b: u32, ) -> Weight;
	fn set_token_properties(b: u32, ) -> Weight;
	fn delete_token_properties(b: u32, ) -> Weight;
	fn token_owner() -> Weight;
	fn set_allowance_for_all() -> Weight;
	fn allowance_for_all() -> Weight;
	fn repair_item() -> Weight;
}

/// Weights for pallet_nonfungible using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Nonfungible TokensMinted (r:1 w:1)
	/// Proof: Nonfungible TokensMinted (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:1 w:1)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenProperties (r:1 w:1)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	/// Storage: Common CollectionPropertyPermissions (r:1 w:0)
	/// Proof: Common CollectionPropertyPermissions (max_values: None, max_size: Some(16726), added: 19201, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenData (r:0 w:1)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:1)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	fn create_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `390`
		//  Estimated: `59511`
		// Minimum execution time: 23_081_000 picoseconds.
		Weight::from_parts(23_551_000, 59511)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: Nonfungible TokensMinted (r:1 w:1)
	/// Proof: Nonfungible TokensMinted (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:1 w:1)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenProperties (r:200 w:200)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	/// Storage: Common CollectionPropertyPermissions (r:1 w:0)
	/// Proof: Common CollectionPropertyPermissions (max_values: None, max_size: Some(16726), added: 19201, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenData (r:0 w:200)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:200)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// The range of component `b` is `[0, 200]`.
	fn create_multiple_items(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `390`
		//  Estimated: `24232 + b * (35279 ±0)`
		// Minimum execution time: 4_557_000 picoseconds.
		Weight::from_parts(5_994_058, 24232)
			// Standard Error: 4_326
			.saturating_add(Weight::from_parts(7_369_489, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 35279).saturating_mul(b.into()))
	}
	/// Storage: Nonfungible TokensMinted (r:1 w:1)
	/// Proof: Nonfungible TokensMinted (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:200 w:200)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenProperties (r:200 w:200)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	/// Storage: Common CollectionPropertyPermissions (r:1 w:0)
	/// Proof: Common CollectionPropertyPermissions (max_values: None, max_size: Some(16726), added: 19201, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenData (r:0 w:200)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:200)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// The range of component `b` is `[0, 200]`.
	fn create_multiple_items_ex(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `390`
		//  Estimated: `21692 + b * (37819 ±0)`
		// Minimum execution time: 4_533_000 picoseconds.
		Weight::from_parts(2_822_660, 21692)
			// Standard Error: 3_650
			.saturating_add(Weight::from_parts(9_100_706, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 37819).saturating_mul(b.into()))
	}
	/// Storage: Nonfungible TokenData (r:1 w:1)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenChildren (r:1 w:0)
	/// Proof: Nonfungible TokenChildren (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokensBurnt (r:1 w:1)
	/// Proof: Nonfungible TokensBurnt (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:1 w:1)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible Allowance (r:1 w:0)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:1)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenProperties (r:0 w:1)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	fn burn_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `412`
		//  Estimated: `12611`
		// Minimum execution time: 23_528_000 picoseconds.
		Weight::from_parts(24_680_000, 12611)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: Nonfungible TokenChildren (r:1 w:0)
	/// Proof: Nonfungible TokenChildren (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenData (r:1 w:1)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokensBurnt (r:1 w:1)
	/// Proof: Nonfungible TokensBurnt (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:1 w:1)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible Allowance (r:1 w:0)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:1)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenProperties (r:0 w:1)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	fn burn_recursively_self_raw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `412`
		//  Estimated: `12611`
		// Minimum execution time: 29_770_000 picoseconds.
		Weight::from_parts(30_114_000, 12611)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: Nonfungible TokenChildren (r:401 w:200)
	/// Proof: Nonfungible TokenChildren (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(860), added: 3335, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenData (r:201 w:201)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokensBurnt (r:1 w:1)
	/// Proof: Nonfungible TokensBurnt (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:2 w:2)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible Allowance (r:201 w:0)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:201)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenProperties (r:0 w:201)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	/// The range of component `b` is `[0, 200]`.
	fn burn_recursively_breadth_plus_self_plus_self_per_each_raw(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1530 + b * (58 ±0)`
		//  Estimated: `18290 + b * (10097 ±0)`
		// Minimum execution time: 31_413_000 picoseconds.
		Weight::from_parts(31_865_000, 18290)
			// Standard Error: 980_032
			.saturating_add(Weight::from_parts(205_236_443, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 10097).saturating_mul(b.into()))
	}
	/// Storage: Nonfungible TokenData (r:1 w:1)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:2 w:2)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible Allowance (r:1 w:0)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:2)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	fn transfer_raw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `412`
		//  Estimated: `10144`
		// Minimum execution time: 9_307_000 picoseconds.
		Weight::from_parts(10_108_000, 10144)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: Nonfungible TokenData (r:1 w:0)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Allowance (r:1 w:1)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	fn approve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `358`
		//  Estimated: `5064`
		// Minimum execution time: 11_507_000 picoseconds.
		Weight::from_parts(11_771_000, 5064)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Nonfungible TokenData (r:1 w:0)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Allowance (r:1 w:1)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	fn approve_from() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `313`
		//  Estimated: `5064`
		// Minimum execution time: 11_558_000 picoseconds.
		Weight::from_parts(11_789_000, 5064)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Nonfungible Allowance (r:1 w:0)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	fn check_allowed_raw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `394`
		//  Estimated: `2532`
		// Minimum execution time: 2_668_000 picoseconds.
		Weight::from_parts(2_877_000, 2532)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Nonfungible Allowance (r:1 w:1)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenData (r:1 w:1)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenChildren (r:1 w:0)
	/// Proof: Nonfungible TokenChildren (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokensBurnt (r:1 w:1)
	/// Proof: Nonfungible TokensBurnt (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:1 w:1)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:1)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenProperties (r:0 w:1)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	fn burn_from() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `527`
		//  Estimated: `12611`
		// Minimum execution time: 30_713_000 picoseconds.
		Weight::from_parts(31_160_000, 12611)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: Common CollectionPropertyPermissions (r:1 w:1)
	/// Proof: Common CollectionPropertyPermissions (max_values: None, max_size: Some(16726), added: 19201, mode: MaxEncodedLen)
	/// The range of component `b` is `[0, 64]`.
	fn set_token_property_permissions(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `281`
		//  Estimated: `19201`
		// Minimum execution time: 2_360_000 picoseconds.
		Weight::from_parts(2_396_000, 19201)
			// Standard Error: 43_257
			.saturating_add(Weight::from_parts(12_085_808, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Nonfungible TokenProperties (r:1 w:1)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	/// Storage: Common CollectionPropertyPermissions (r:1 w:0)
	/// Proof: Common CollectionPropertyPermissions (max_values: None, max_size: Some(16726), added: 19201, mode: MaxEncodedLen)
	/// The range of component `b` is `[0, 64]`.
	fn set_token_properties(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `616 + b * (261 ±0)`
		//  Estimated: `54480`
		// Minimum execution time: 12_543_000 picoseconds.
		Weight::from_parts(12_686_000, 54480)
			// Standard Error: 52_286
			.saturating_add(Weight::from_parts(6_894_785, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Nonfungible TokenProperties (r:1 w:1)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	/// Storage: Common CollectionPropertyPermissions (r:1 w:0)
	/// Proof: Common CollectionPropertyPermissions (max_values: None, max_size: Some(16726), added: 19201, mode: MaxEncodedLen)
	/// The range of component `b` is `[0, 64]`.
	fn delete_token_properties(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `653 + b * (33291 ±0)`
		//  Estimated: `54480`
		// Minimum execution time: 12_352_000 picoseconds.
		Weight::from_parts(12_523_000, 54480)
			// Standard Error: 70_401
			.saturating_add(Weight::from_parts(21_959_228, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Nonfungible TokenData (r:1 w:0)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	fn token_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `358`
		//  Estimated: `2532`
		// Minimum execution time: 4_797_000 picoseconds.
		Weight::from_parts(5_499_000, 2532)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Nonfungible CollectionAllowance (r:0 w:1)
	/// Proof: Nonfungible CollectionAllowance (max_values: None, max_size: Some(111), added: 2586, mode: MaxEncodedLen)
	fn set_allowance_for_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_303_000 picoseconds.
		Weight::from_parts(6_712_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Nonfungible CollectionAllowance (r:1 w:0)
	/// Proof: Nonfungible CollectionAllowance (max_values: None, max_size: Some(111), added: 2586, mode: MaxEncodedLen)
	fn allowance_for_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `2586`
		// Minimum execution time: 3_798_000 picoseconds.
		Weight::from_parts(4_017_000, 2586)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Nonfungible TokenProperties (r:1 w:1)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	fn repair_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `300`
		//  Estimated: `35279`
		// Minimum execution time: 5_531_000 picoseconds.
		Weight::from_parts(5_750_000, 35279)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Nonfungible TokensMinted (r:1 w:1)
	/// Proof: Nonfungible TokensMinted (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:1 w:1)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenProperties (r:1 w:1)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	/// Storage: Common CollectionPropertyPermissions (r:1 w:0)
	/// Proof: Common CollectionPropertyPermissions (max_values: None, max_size: Some(16726), added: 19201, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenData (r:0 w:1)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:1)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	fn create_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `390`
		//  Estimated: `59511`
		// Minimum execution time: 23_081_000 picoseconds.
		Weight::from_parts(23_551_000, 59511)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: Nonfungible TokensMinted (r:1 w:1)
	/// Proof: Nonfungible TokensMinted (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:1 w:1)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenProperties (r:200 w:200)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	/// Storage: Common CollectionPropertyPermissions (r:1 w:0)
	/// Proof: Common CollectionPropertyPermissions (max_values: None, max_size: Some(16726), added: 19201, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenData (r:0 w:200)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:200)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// The range of component `b` is `[0, 200]`.
	fn create_multiple_items(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `390`
		//  Estimated: `24232 + b * (35279 ±0)`
		// Minimum execution time: 4_557_000 picoseconds.
		Weight::from_parts(5_994_058, 24232)
			// Standard Error: 4_326
			.saturating_add(Weight::from_parts(7_369_489, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 35279).saturating_mul(b.into()))
	}
	/// Storage: Nonfungible TokensMinted (r:1 w:1)
	/// Proof: Nonfungible TokensMinted (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:200 w:200)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenProperties (r:200 w:200)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	/// Storage: Common CollectionPropertyPermissions (r:1 w:0)
	/// Proof: Common CollectionPropertyPermissions (max_values: None, max_size: Some(16726), added: 19201, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenData (r:0 w:200)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:200)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// The range of component `b` is `[0, 200]`.
	fn create_multiple_items_ex(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `390`
		//  Estimated: `21692 + b * (37819 ±0)`
		// Minimum execution time: 4_533_000 picoseconds.
		Weight::from_parts(2_822_660, 21692)
			// Standard Error: 3_650
			.saturating_add(Weight::from_parts(9_100_706, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((4_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 37819).saturating_mul(b.into()))
	}
	/// Storage: Nonfungible TokenData (r:1 w:1)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenChildren (r:1 w:0)
	/// Proof: Nonfungible TokenChildren (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokensBurnt (r:1 w:1)
	/// Proof: Nonfungible TokensBurnt (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:1 w:1)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible Allowance (r:1 w:0)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:1)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenProperties (r:0 w:1)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	fn burn_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `412`
		//  Estimated: `12611`
		// Minimum execution time: 23_528_000 picoseconds.
		Weight::from_parts(24_680_000, 12611)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: Nonfungible TokenChildren (r:1 w:0)
	/// Proof: Nonfungible TokenChildren (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenData (r:1 w:1)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokensBurnt (r:1 w:1)
	/// Proof: Nonfungible TokensBurnt (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:1 w:1)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible Allowance (r:1 w:0)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:1)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenProperties (r:0 w:1)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	fn burn_recursively_self_raw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `412`
		//  Estimated: `12611`
		// Minimum execution time: 29_770_000 picoseconds.
		Weight::from_parts(30_114_000, 12611)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: Nonfungible TokenChildren (r:401 w:200)
	/// Proof: Nonfungible TokenChildren (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// Storage: Common CollectionById (r:1 w:0)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(860), added: 3335, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenData (r:201 w:201)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokensBurnt (r:1 w:1)
	/// Proof: Nonfungible TokensBurnt (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:2 w:2)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible Allowance (r:201 w:0)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:201)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenProperties (r:0 w:201)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	/// The range of component `b` is `[0, 200]`.
	fn burn_recursively_breadth_plus_self_plus_self_per_each_raw(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1530 + b * (58 ±0)`
		//  Estimated: `18290 + b * (10097 ±0)`
		// Minimum execution time: 31_413_000 picoseconds.
		Weight::from_parts(31_865_000, 18290)
			// Standard Error: 980_032
			.saturating_add(Weight::from_parts(205_236_443, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().reads((4_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
			.saturating_add(RocksDbWeight::get().writes((4_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 10097).saturating_mul(b.into()))
	}
	/// Storage: Nonfungible TokenData (r:1 w:1)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:2 w:2)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible Allowance (r:1 w:0)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:2)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	fn transfer_raw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `412`
		//  Estimated: `10144`
		// Minimum execution time: 9_307_000 picoseconds.
		Weight::from_parts(10_108_000, 10144)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: Nonfungible TokenData (r:1 w:0)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Allowance (r:1 w:1)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	fn approve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `358`
		//  Estimated: `5064`
		// Minimum execution time: 11_507_000 picoseconds.
		Weight::from_parts(11_771_000, 5064)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Nonfungible TokenData (r:1 w:0)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible Allowance (r:1 w:1)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	fn approve_from() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `313`
		//  Estimated: `5064`
		// Minimum execution time: 11_558_000 picoseconds.
		Weight::from_parts(11_789_000, 5064)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Nonfungible Allowance (r:1 w:0)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	fn check_allowed_raw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `394`
		//  Estimated: `2532`
		// Minimum execution time: 2_668_000 picoseconds.
		Weight::from_parts(2_877_000, 2532)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Nonfungible Allowance (r:1 w:1)
	/// Proof: Nonfungible Allowance (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenData (r:1 w:1)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenChildren (r:1 w:0)
	/// Proof: Nonfungible TokenChildren (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokensBurnt (r:1 w:1)
	/// Proof: Nonfungible TokensBurnt (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Nonfungible AccountBalance (r:1 w:1)
	/// Proof: Nonfungible AccountBalance (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Nonfungible Owned (r:0 w:1)
	/// Proof: Nonfungible Owned (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: Nonfungible TokenProperties (r:0 w:1)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	fn burn_from() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `527`
		//  Estimated: `12611`
		// Minimum execution time: 30_713_000 picoseconds.
		Weight::from_parts(31_160_000, 12611)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: Common CollectionPropertyPermissions (r:1 w:1)
	/// Proof: Common CollectionPropertyPermissions (max_values: None, max_size: Some(16726), added: 19201, mode: MaxEncodedLen)
	/// The range of component `b` is `[0, 64]`.
	fn set_token_property_permissions(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `281`
		//  Estimated: `19201`
		// Minimum execution time: 2_360_000 picoseconds.
		Weight::from_parts(2_396_000, 19201)
			// Standard Error: 43_257
			.saturating_add(Weight::from_parts(12_085_808, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Nonfungible TokenProperties (r:1 w:1)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	/// Storage: Common CollectionPropertyPermissions (r:1 w:0)
	/// Proof: Common CollectionPropertyPermissions (max_values: None, max_size: Some(16726), added: 19201, mode: MaxEncodedLen)
	/// The range of component `b` is `[0, 64]`.
	fn set_token_properties(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `616 + b * (261 ±0)`
		//  Estimated: `54480`
		// Minimum execution time: 12_543_000 picoseconds.
		Weight::from_parts(12_686_000, 54480)
			// Standard Error: 52_286
			.saturating_add(Weight::from_parts(6_894_785, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Nonfungible TokenProperties (r:1 w:1)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	/// Storage: Common CollectionPropertyPermissions (r:1 w:0)
	/// Proof: Common CollectionPropertyPermissions (max_values: None, max_size: Some(16726), added: 19201, mode: MaxEncodedLen)
	/// The range of component `b` is `[0, 64]`.
	fn delete_token_properties(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `653 + b * (33291 ±0)`
		//  Estimated: `54480`
		// Minimum execution time: 12_352_000 picoseconds.
		Weight::from_parts(12_523_000, 54480)
			// Standard Error: 70_401
			.saturating_add(Weight::from_parts(21_959_228, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Nonfungible TokenData (r:1 w:0)
	/// Proof: Nonfungible TokenData (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	fn token_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `358`
		//  Estimated: `2532`
		// Minimum execution time: 4_797_000 picoseconds.
		Weight::from_parts(5_499_000, 2532)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Nonfungible CollectionAllowance (r:0 w:1)
	/// Proof: Nonfungible CollectionAllowance (max_values: None, max_size: Some(111), added: 2586, mode: MaxEncodedLen)
	fn set_allowance_for_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_303_000 picoseconds.
		Weight::from_parts(6_712_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Nonfungible CollectionAllowance (r:1 w:0)
	/// Proof: Nonfungible CollectionAllowance (max_values: None, max_size: Some(111), added: 2586, mode: MaxEncodedLen)
	fn allowance_for_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `2586`
		// Minimum execution time: 3_798_000 picoseconds.
		Weight::from_parts(4_017_000, 2586)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Nonfungible TokenProperties (r:1 w:1)
	/// Proof: Nonfungible TokenProperties (max_values: None, max_size: Some(32804), added: 35279, mode: MaxEncodedLen)
	fn repair_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `300`
		//  Estimated: `35279`
		// Minimum execution time: 5_531_000 picoseconds.
		Weight::from_parts(5_750_000, 35279)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}

