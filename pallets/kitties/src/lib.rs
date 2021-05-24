#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, StorageValue, StorageDoubleMap,
	traits::Randomness, RuntimeDebug, dispatch::DispatchResult,
};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Kitty(pub [u8; 16]);
#[derive(PartialEq)]		
pub enum KittyGender {
	Male,
	Female,
}

impl Kitty {
	pub fn gender(&self) -> KittyGender {
		if self.0[8] % 2 == 0 {	
			return KittyGender::Male
		}
		KittyGender::Female
	}
}
pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as Kitties {
		/// Stores all the kitties, key is the kitty id
		pub Kitties get(fn kitties): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u8 => Option<Kitty>;
		/// Stores the next kitty ID
		pub NextKittyId get(fn next_kitty_id): u8;
	}
}

decl_event! {
	pub enum Event<T> where
		<T as frame_system::Config>::AccountId,
	{
		/// A kitty is created. \[owner, kitty_id, kitty\]
		KittyCreated(AccountId, u8, Kitty),
		KittyBorn(AccountId, u8, Kitty),
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
		KittiesIdOverflow,
		KittiesHasSameGender,
		KittiesHasDifferentOwner,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		/// Create a new kitty
		#[weight = 1000]
		pub fn create(origin) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let kitty_id = Self::next_kitty_id();

			// TODO: ensure kitty id does not overflow
			if kitty_id.checked_add(1).is_none() {
				return Err(Error::<T>::KittiesIdOverflow.into());
			}
			
			// Generate a random 128bit value
			let dna = (
				<pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
				&sender,
				<frame_system::Module<T>>::extrinsic_index(),
			).using_encoded(blake2_128);
			
			// Create and store kitty
			let kitty = Kitty(dna);
			Kitties::<T>::insert(&sender, kitty_id, kitty.clone());
			NextKittyId::put(kitty_id + 1);

			// Emit event
			Self::deposit_event(RawEvent::KittyCreated(sender, kitty_id, kitty));
			
			Ok(())
		}
	
		#[weight = 1000]
		pub fn breed(origin, kitty1: Kitty, kitty2: Kitty) -> DispatchResult {
		 	let sender = ensure_signed(origin)?;

			let kitty_id = Self::next_kitty_id();

			// Check if kitty id overflow
			if kitty_id.checked_add(1).is_none() {
				return Err(Error::<T>::KittiesIdOverflow.into());
			}

			// Check if kitty1 & kitty2 has same gender
			if kitty1.gender() == kitty2.gender() {
				return Err(Error::<T>::KittiesHasSameGender.into());
			}

			// Check if kitty1 & kitty2 has different owner

			// Crate and store kitty id
			let k1 = kitty1.clone();
			let k2 = kitty2.clone();

			// mix up the dna 

			// let kitty = Kitty(dna);

			// Emit event
			// Self::deposit_event(RawEvent::KittyBorn(sender, kitty_id, kitty));
		 	
			Ok(())
		}

		// #[weight=1000]
		// pub fn transfer_kitty(origin, to: T::AccountId, kitty: Kitty) -> DispatchResult {
			
		// 	Ok(())
		// }
	}
}


// fn breed_kitty(origin, kitty_id_1: T::Hash, kitty_id_2: T::Hash) -> Result{
// 	let sender = ensure_signed(origin)?;

// 	ensure!(<Kitties<T>>::exists(kitty_id_1), "This cat 1 does not exist");
// 	ensure!(<Kitties<T>>::exists(kitty_id_2), "This cat 2 does not exist");

// 	let nonce = <Nonce<T>>::get();
// 	let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
// 		.using_encoded(<T as system::Trait>::Hashing::hash);

// 	let kitty_1 = Self::kitty(kitty_id_1);
// 	let kitty_2 = Self::kitty(kitty_id_2);

// 	let mut final_dna = kitty_1.dna;
// 	for (i, (dna_2_element, r)) in kitty_2.dna.as_ref().iter().zip(random_hash.as_ref().iter()).enumerate() {
// 		if r % 2 == 0 {
// 			final_dna.as_mut()[i] = *dna_2_element;
// 		}
// 	}

// 	let new_kitty = Kitty {
// 		id: random_hash,
// 		dna: final_dna,
// 		price: <T::Balance as As<u64>>::sa(0),
// 		gen: cmp::max(kitty_1.gen, kitty_2.gen) + 1,
// 	};

// 	Self::mint(sender, random_hash, new_kitty)?;

// 	<Nonce<T>>::mutate(|n| *n += 1);

// 	Ok(())
// }

// Create kitty... add exception to that kitty error 
// 3. Design breed feature for pallet-kitties
// 4. Requirements:
//     - A kitty have gender derived from on DNA
//     - Kitty owner can choose two kitties with opposite gender to breed a new kitten
//     - New kitten should inherits the DNA from parents

// Assignment four:
// Design transfer feature for kitties pallet
// Requirements: 
// 	- Kitty owner should be able to transfer kitty to someone else
// 	- 
