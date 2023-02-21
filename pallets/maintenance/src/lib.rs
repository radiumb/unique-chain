// Copyright 2019-2022 Unique Network (Gibraltar) Ltd.
// This file is part of Unique Network.

// Unique Network is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Unique Network is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Unique Network. If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::*,
		pallet_prelude::*,
	};
	use frame_support::{
		traits::{QueryPreimage, StorePreimage},
	};
	use frame_system::pallet_prelude::*;
	use sp_core::H256;

	use crate::weights::WeightInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The runtime origin type.
		type RuntimeOrigin: From<RawOrigin<Self::AccountId>>
			+ IsType<<Self as frame_system::Config>::RuntimeOrigin>;

		/// The aggregated call type.
		type RuntimeCall: Parameter
			+ Dispatchable<
				RuntimeOrigin = <Self as Config>::RuntimeOrigin,
				PostInfo = PostDispatchInfo,
			> + GetDispatchInfo
			+ From<frame_system::Call<Self>>;

		/// The preimage provider with which we look up call hashes to get the call.
		type Preimages: QueryPreimage + StorePreimage;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MaintenanceEnabled,
		MaintenanceDisabled,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn is_enabled)]
	pub type Enabled<T> = StorageValue<_, bool, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::enable())]
		pub fn enable(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;

			<Enabled<T>>::set(true);

			Self::deposit_event(Event::MaintenanceEnabled);

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::disable())]
		pub fn disable(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;

			<Enabled<T>>::set(false);

			Self::deposit_event(Event::MaintenanceDisabled);

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::execute_preimage())]
		pub fn execute_preimage(_origin: OriginFor<T>, _hash: H256) -> DispatchResult {
			#[cfg(feature = "governance")]
			{
				let origin = _origin;
				let hash = _hash;
				
				ensure_root(origin)?;

				let len = T::Preimages::len(&hash).ok_or(DispatchError::Unavailable)?;
				let bounded = T::Preimages::pick::<<T as Config>::RuntimeCall>(hash, len);
				let (call, _) =
					T::Preimages::realize(&bounded).map_err(|_| DispatchError::Unavailable)?;

				let result = match call.dispatch(frame_system::RawOrigin::Root.into()) {
					Ok(_) => Ok(()),
					Err(error_and_info) => Err(error_and_info.error),
				};

				result
			}

			#[cfg(not(feature = "governance"))]
			{
				Err(DispatchError::Unavailable)
			}
		}
	}
}
