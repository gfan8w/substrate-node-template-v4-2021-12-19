#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
use sp_std::{collections::btree_set::BTreeSet, prelude::*};


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::{Vec};
	use sp_std::{collections::btree_set::BTreeSet, prelude::*};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	// The pallet's storage items.
	#[pallet::storage]
	#[pallet::getter(fn validators)]
	pub type Validators<T: Config> = StorageValue<_, Vec<T::AccountId>,ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		///no Validators
		NoValidators,
		/// Duplicated Validator
		Duplicated,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_validator(origin: OriginFor<T>, validator_id: T::AccountId) -> DispatchResult {

			let validator_set: BTreeSet<_> = <Validators<T>>::get().into_iter().collect();

			ensure!(!validator_set.contains(&validator_id), Error::<T>::Duplicated);

			<Validators<T>>::mutate(|v| v.push(validator_id.clone()));


			// 下面的方法OK
			/*let mut validators: Vec<T::AccountId>;

			if <Validators<T>>::get().is_none() {
				validators =Vec::new();  //vec![validator_id.clone()];
			} else {
				validators = <Validators<T>>::get().unwrap();
			}
			validators.push(validator_id.clone());

			<Validators<T>>::put(validators);*/

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn remove_validator(
			origin: OriginFor<T>,
			validator_id: T::AccountId,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let mut validators = <Validators<T>>::get();
			validators.retain(|v| *v !=validator_id);
			<Validators<T>>::put(validators);

			//这个方法OK，注意已经改为ValueQuery
			/*<Validators<T>>::try_mutate(|valid| match valid {
				Some(v) => {
					if let Some(ind) = v.iter().position(|id| *id == validator_id) {
						v.swap_remove(ind);
						return Ok(());
					}
					Err(())
				}
				_ => Err(())
			}).map_err(|_| Error::<T>::NoValidators)?;*/

			// 这种方法有问题。
			/*<Validators<T>>::try_mutate(|valid| match valid {
                Some(v) => {
                    v.binary_search(&validator_id).map(|i| {
						log::info!("find v idx:{}",i);
						let r =v.remove(i);
						log::info!("removed:{:?}",r);
						r
					});
                    log::info!("v:{:?}", v);
                    Ok(())
                }
                _ => Err(())
            })
            .map_err(|_| Error::<T>::NoValidators)?;*/

			//let mut validators = <Validators<T>>::get().ok_or(Error::<T>::NoValidators)?;

			// Assuming that this will be a PoA network for enterprise use-cases,
			// the validator count may not be too big; the for loop shouldn't be too heavy.
			// In case the validator count is large, we need to find another way. **TODO**
			/*for (i, v) in validators.clone().into_iter().enumerate() {
				if v == validator_id {
					validators.swap_remove(i);
				}
			}
			<Validators<T>>::put(validators);*/

			log::info!("remaining validator:{:?}", <Validators<T>>::get());
			Ok(())
		}


	}
}
