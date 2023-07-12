#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod tipos;

use frame_support::traits::{Currency, Get};
use tipos::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, Blake2_128Concat, ensure, traits::tokens::Balance};//, sp_runtime};
	use frame_system::pallet_prelude::*;
	//use sp_runtime::BoundedVec;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type LargoMinimoNombreProyecto: Get<u32>;

		#[pallet::constant]
		type LargoMaximoNombreProyecto: Get<u32>;

		type Currency: Currency<Self::AccountId>; // Pueden no utilizarlo.
	}

	#[pallet::storage]
	pub type Proyectos<T> =
		StorageMap<_, Blake2_128Concat, BoundedString<T>, BalanceDe<T>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProyectoCreado { quien: T::AccountId, nombre: NombreProyecto<T> },
		ProyectoApoyado { nombre: NombreProyecto<T>, cantidad: BalanceDe<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Nombre de archivo muy largo.
		NombreMuyLargo,
		/// Nombre de archivo muy corto.
		NombreMuyCorto,
		/// El usuario quiso apoyar un proyecto con más fondos de los que dispone.
		FondosInsuficientes,
		/// El usuario quiso apoyar un proyecto inexistente.
		ProyectoNoExiste,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Crea un proyecto.
		pub fn crear_proyecto(origen: OriginFor<T>, nombre: String) -> DispatchResult {
			// Completar este método.
			//todo!()
			ensure!(nombre.len() >= T::LargoMinimoNombreProyecto::get() as usize, Error::<T>::NombreMuyCorto);
			ensure!(nombre.len() <= T::LargoMaximoNombreProyecto::get() as usize, Error::<T>::NombreMuyLargo);

			let quien = ensure_signed(origen)?;
			let nombre: NombreProyecto<T> = nombre.try_into().unwrap();

			let balance = Proyectos::<T>::get(nombre.clone());
			let _proyecto = Proyectos::<T>::set(nombre.clone(), balance);

			Self::deposit_event(Event::ProyectoCreado { quien, nombre });
			Ok(())
		}

		pub fn apoyar_proyecto(
			origen: OriginFor<T>,
			nombre: String,
			cantidad: BalanceDe<T>,
		) -> DispatchResult {
			// Completar este método.
			//todo!()
			let quien = ensure_signed(origen)?;
			let nombre: NombreProyecto<T> = nombre.try_into().unwrap();
			ensure!(Proyectos::<T>::contains_key(nombre.clone()), Error::<T>::ProyectoNoExiste);
			ensure!(cantidad <= T::Currency::total_balance(&quien), Error::<T>::FondosInsuficientes);

			let _proyecto = Proyectos::<T>::set(nombre.clone(), cantidad);

			Self::deposit_event(Event::ProyectoApoyado { nombre, cantidad });
			Ok(())			
		}
	}
}
