#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;


#[frame_support::pallet]
pub mod pallet {
	
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

	

	#[pallet::storage]
	pub type Partido1<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type Partido2<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type Partido3<T> = StorageValue<_, u32>;



	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		VotoExitoso(T::AccountId ),
		RegistroExitoso(u32 ),
	
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {


		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn votar_partido1(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			match Partido1::<T>::get() 	{
				None => Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					Partido1::<T>::put(new);
					Self::deposit_event(Event::VotoExitoso(who));

					Ok(())
				},
			}
		
		}

		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn votar_partido2(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			match Partido2::<T>::get() 	{
				None => Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					Partido2::<T>::put(new);
					Self::deposit_event(Event::VotoExitoso(who));

					Ok(())
				},	
			}		

		}

		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn votar_partido3(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			match Partido3::<T>::get() 	{
				None => Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					Partido3::<T>::put(new);
					Self::deposit_event(Event::VotoExitoso(who));

					Ok(())
				},
			}

		}


		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn registrar_candidato1(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Partido1::<T>::put(0);
			Self::deposit_event(Event::RegistroExitoso(1));

					Ok(())

		}


		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn registrar_candidato2(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Partido2::<T>::put(0);
			Self::deposit_event(Event::RegistroExitoso(2));

					Ok(())

		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn registrar_candidato3(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Partido3::<T>::put(0);
			Self::deposit_event(Event::RegistroExitoso(3));

					Ok(())

		}













	}
}
