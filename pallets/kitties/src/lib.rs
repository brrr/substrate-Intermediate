// SPDX-License-Identifier: MIT

//! # kitties Pallet
//!
//! substrate 进阶课程之kitty模块(第三课)
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `create` - 创建kitty
//!
//! - `breed` - 孵化kitty
//!
//! - `transfer` - 转移kitty

#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
mod migrations;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_support::PalletId;
	use frame_system::pallet_prelude::*;
	use frame_system::{
		offchain::{
			AppCrypto, CreateSignedTransaction, SendUnsignedTransaction,
			SignedPayload, Signer, SigningTypes,
		},
	};
	use sp_runtime::{
		transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
		RuntimeDebug,
	};
	use sp_io::hashing::blake2_128;
	use frame_support::traits::{Randomness, Currency, ExistenceRequirement};
	use frame_support::traits::tokens::Balance;
	use sp_runtime::traits::AccountIdConversion;
	// use sp_core::blake2_128;
	use migrations;
	// use sp_runtime::offchain::StorageValueRef;
	use frame_support::inherent::Vec;
	use sp_runtime::{
		offchain::{
			storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
		},
		traits::Zero,
	};
	use sp_io::offchain_index;
	use sp_std::str;
	use serde::{Deserialize, Deserializer};

	// use codec::{Decode, Encode};



	use sp_core::crypto::KeyTypeId;


	pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"btc!");
	pub mod crypto {
		use super::KEY_TYPE;
		use sp_core::sr25519::Signature as Sr25519Signature;
		use sp_runtime::{
			app_crypto::{app_crypto, sr25519},
			traits::Verify,
			MultiSignature, MultiSigner,
		};
		app_crypto!(sr25519, KEY_TYPE);

		pub struct TestAuthId;

		impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}

		// implemented for mock runtime in test
		impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
		{
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}
	}

	pub type KittyId = u32;
	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


	#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Default, TypeInfo, MaxEncodedLen)]
	pub struct  Kitty {
		pub dna: [u8; 16],
		pub name: [u8; 8]
	}

	pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config  + CreateSignedTransaction<Call<Self>>{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type Currency: Currency<Self::AccountId>;
		#[pallet::constant]
		type KittyPrice: Get<BalanceOf<Self>>;
		type PalletId: Get<PalletId>;
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type NextKittyId<T> = StorageValue<_, KittyId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyId, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_parents)]
	pub type KittyParents<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, (KittyId, KittyId), OptionQuery>;


	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, T::AccountId>;


	#[pallet::storage]
	#[pallet::getter(fn kitty_on_sale)]
	pub type KittyOnSale<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, ()>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyCreated { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyBred { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyTransferred { who: T::AccountId, recipient: T::AccountId,  kitty_id: KittyId },
		KittyOnSale { who: T::AccountId, kitty_id: KittyId },
		KittyBought { who: T::AccountId, kitty_id: KittyId}
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		InvalidKittyId,
		SameKittyId,
		NotOwner,
		AlreadyOnSale,
		AlreadyOwned,
		NotOnSale
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		/// Validate unsigned call to this module.
		///
		/// By default unsigned transactions are disallowed, but implementing the validator
		/// here we make sure that some particular calls (the ones produced by offchain worker)
		/// are being whitelisted and marked as valid.
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			const UNSIGNED_TXS_PRIORITY: u64 = 100;
			let valid_tx = |provide| ValidTransaction::with_tag_prefix("my-pallet")
				.priority(UNSIGNED_TXS_PRIORITY) // please define `UNSIGNED_TXS_PRIORITY` before this line
				.and_provides([&provide])
				.longevity(3)
				.propagate(true)
				.build();

			// match call {
			// 	Call::submit_data_unsigned { key: _ } => valid_tx(b"my_unsigned_tx".to_vec()),
			// 	_ => InvalidTransaction::Call.into(),
			// }

			match call {
				Call::unsigned_extrinsic_with_signed_payload {
					ref payload,
					ref signature
				} => {
					if !SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone()) {
						return InvalidTransaction::BadProof.into();
					}
					valid_tx(b"unsigned_extrinsic_with_signed_payload".to_vec())
				},
				_ => InvalidTransaction::Call.into(),
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> Weight {
			migrations::v1::migrate::<T>();
			migrations::v2::migrate::<T>();
			Weight::zero()
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!("OCW ==>  entry offchain_worker at {:?}", block_number);
			// Reading back the offchain indexing value. This is exactly the same as reading from
			// ocw local storage.
			let key = Self::derived_key(block_number);
			let storage_ref = StorageValueRef::persistent(&key);

			if let Ok(Some(data)) = storage_ref.get::<IndexingData>() {
				// log::info!("local storage data: {:?}, {:?}",
				// str::from_utf8(&data.0).unwrap_or("error"), data.1);
				log::info!("local storage data: {:?}", str::from_utf8(&data.0).unwrap_or("error"));

				log::info!("======>11111");
				//模拟当某种Index存储存在，且处理后，再次向链上发起交易
				let number: u64 = 42;
				// Retrieve the signer to sign the payload
				let signer = Signer::<T, T::AuthorityId>::any_account();
				log::info!("======>22222");

				// `send_unsigned_transaction` is returning a type of `Option<(Account<T>, Result<(), ()>)>`.
				//	 The returned result means:
				//	 - `None`: no account is available for sending transaction
				//	 - `Some((account, Ok(())))`: transaction is successfully sent
				//	 - `Some((account, Err(())))`: error occurred when sending the transaction
				if let Some((_, res)) = signer.send_unsigned_transaction(
					// this line is to prepare and return payload
					|acct| Payload { number, public: acct.public.clone() },
					|payload, signature| Call::unsigned_extrinsic_with_signed_payload { payload, signature },
				) {
					match res {
						Ok(()) => {log::info!("OCW ==> unsigned tx with signed payload successfully sent.");}
						Err(()) => {log::error!("OCW ==> sending unsigned tx with signed payload failed.");}
					};
				} else {
					// The case of `None`: no account is available for sending
					log::error!("OCW ==> No local account available");
				}
			} else {
				log::info!("Error reading from local storage.");
			}


		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {


		#[pallet::call_index(5)]
		#[pallet::weight(0)]
		pub fn save_to_offchain_indexing(origin: OriginFor<T>, msg: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let key = Self::derived_key(frame_system::Module::<T>::block_number());
			// let data = IndexingData(b"submit_number_unsigned".to_vec(), number);
			let data = IndexingData(msg);
			offchain_index::set(&key, &data.encode());
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(0)]
		pub fn unsigned_extrinsic_with_signed_payload(origin: OriginFor<T>, payload: Payload<T::Public>, _signature: T::Signature,) -> DispatchResult {
			ensure_none(origin)?;

			log::info!("OCW ==> in call unsigned_extrinsic_with_signed_payload: {:?}", payload.number);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create(origin: OriginFor<T>, name: [u8; 8]) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			let kitty_id = Self::get_next_id()?;
			let dna = Self::random_value(&who);
			let kitty = Kitty{dna, name};

			let price = T::KittyPrice::get();
			// T::Currency::reserve(&who, price)?;
			T::Currency::transfer(&who, &Self::get_account_id(), price, ExistenceRequirement::KeepAlive)?;

			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);

			Self::deposit_event(Event::KittyCreated { who, kitty_id, kitty});
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn breed(origin: OriginFor<T>, kitty_id_1: KittyId, kitty_id_2: KittyId, name: [u8; 8]) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);
			ensure!(Kitties::<T>::contains_key(kitty_id_1), Error::<T>::InvalidKittyId);
			ensure!(Kitties::<T>::contains_key(kitty_id_2), Error::<T>::InvalidKittyId);

			let kitty_id = Self::get_next_id()?;
			let kitty_1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
			let kitty_2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

			let selector = Self::random_value(&who);
			// let mut data = [0u8; 16];
			let dna = [0u8; 16];

			// for i in 0..kitty_1.0.len() {
			// 	data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & selector[i]);
			// }
			let kitty = Kitty{dna, name};

			let price = T::KittyPrice::get();
			T::Currency::transfer(&who,&Self::get_account_id(), price, ExistenceRequirement::KeepAlive);

			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			KittyParents::<T>::insert(kitty_id, (kitty_id_1, kitty_id_2));

			Self::deposit_event(Event::KittyBred { who, kitty_id, kitty});
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn transfer(origin: OriginFor<T>, recipient: T::AccountId, kitty_id: KittyId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Kitties::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);
			let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			ensure!(owner == who, Error::<T>::NotOwner);

			KittyOwner::<T>::set(kitty_id,  Option::Some(recipient.clone()));

			Self::deposit_event(Event::KittyTransferred { who, recipient, kitty_id});
			Ok(())

		}


		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn sale(origin: OriginFor<T>, kitty_id: KittyId)-> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Kitties::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);
			let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			ensure!(owner == who, Error::<T>::NotOwner);
			ensure!(Self::kitty_on_sale(kitty_id).is_none(), Error::<T>::AlreadyOnSale);
			<KittyOnSale<T>>::insert(kitty_id, ());
			Self::deposit_event(Event::KittyOnSale {who, kitty_id });
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn buy(origin: OriginFor<T>, kitty_id: KittyId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::kitties(kitty_id).ok_or(Error::<T>::InvalidKittyId);
			let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			ensure!(who != owner, Error::<T>::AlreadyOwned);
			ensure!(Self::kitty_on_sale(kitty_id).is_some(), Error::<T>::NotOnSale);

			let price = T::KittyPrice::get();
			// T::Currency::reserve(&who, price);
			// T::Currency::unreserve(&owner, price);
			T::Currency::transfer(&who, &owner, price, ExistenceRequirement::KeepAlive);

			KittyOwner::<T>::set(kitty_id,  Option::Some(who.clone()));
			KittyOnSale::<T>::remove(kitty_id);

			Self::deposit_event(Event::KittyBought {who, kitty_id });

			Ok(())
		}

	}

	const ONCHAIN_TX_KEY: &[u8] = b"kitties_pallet::indexing1";

	#[derive(Debug, Deserialize, Encode, Decode, Default)]
	struct IndexingData(Vec<u8>);

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct Payload<Public> {
		number: u64,
		public: Public,
	}

	impl<T: SigningTypes> SignedPayload<T> for Payload<T::Public> {
		fn public(&self) -> T::Public {
			self.public.clone()
		}
	}

	impl <T: Config> Pallet<T> {
		fn derived_key(block_number: T::BlockNumber) -> Vec<u8> {
			let a =block_number.using_encoded(|encoded_bn| {
				ONCHAIN_TX_KEY.clone().into_iter()
					.chain(b"/".into_iter())
					.chain(encoded_bn)
					.copied()
					.collect::<Vec<u8>>()
			});
			log::info!("==============>{:?}", a);
			log::info!("==============>{:?}", str::from_utf8(&a));
			a
		}

		fn get_next_id() -> Result<KittyId, DispatchError>{
			NextKittyId::<T>::try_mutate(|next_id| -> Result<KittyId, DispatchError> {
				let current_id = *next_id;
				*next_id = next_id.checked_add(1).ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
				Ok(current_id)
			})
		}

		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}

		fn get_account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}
	}

	// #[pallet::hooks]
	// impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
	// 	/// Offchain worker entry point.
	// 	fn offchain_worker(_block_number: T::BlockNumber) {
	// 		log::info!("OCW ==> Failed in offchain_unsigned_tx");
	// 	}
	// }
}
