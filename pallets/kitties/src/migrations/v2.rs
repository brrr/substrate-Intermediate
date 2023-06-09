pub use crate::pallet::*;
use frame_support::{
    pallet_prelude::*,
    storage::StoragePrefixedMap,
    traits::GetStorageVersion,
    weights::Weight,
    migration::storage_key_iter, Blake2_128Concat
};
use crate::{ Kitty, KittyId };
use frame_system::pallet_prelude::*;

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Default, TypeInfo, MaxEncodedLen)]
pub struct  KittyV1 {
    pub dna: [u8; 16],
    pub name: [u8; 4]
}

pub fn migrate<T: Config>() -> Weight {
    let on_chain_version = Pallet::<T>::on_chain_storage_version();//StorageVersion::get::<Pallet<T>>();
    let current_version = Pallet::<T>::current_storage_version();
    if on_chain_version != 1 {
        return Weight::zero();
    }

    if current_version != 2 {
        return Weight::zero();
    }

    let module = Kitties::<T>::module_prefix();
    let item = Kitties::<T>::storage_prefix();

    for (index, kitty) in storage_key_iter::<KittyId, KittyV1, Blake2_128Concat>(module, item).drain() {
        // let kitty = kitty.unwrap();
        // let kitty_id = index as u32;
        let newKitty = Kitty {
            dna: kitty.dna,
            name: *b"already2"
        };
        Kitties::<T>::insert(index, &newKitty);
    }
    Weight::zero()
}