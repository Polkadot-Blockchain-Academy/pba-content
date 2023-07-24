use parity_scale_codec::{Decode, Encode};
use sp_std::prelude::*;

pub trait StorageValue {
	/// The type of value that this storage value holds.
	///
	/// It can be anything that is encode- and decode-able.
	type Value: Encode + Decode;

	/// The final storage key of `Self` as a storage value.
	fn raw_storage_key() -> Vec<u8>;

	/// Get the underlying value. If it doesn't exist, return `None`.
	fn get() -> Option<Self::Value> {
		let key = Self::raw_storage_key();
		sp_io::storage::get(&key)
			.and_then(|raw_value| <Self::Value as Decode>::decode(&mut &*raw_value).ok())
	}

	/// Check if the value exists in storage.
	fn exists() -> bool {
		// Could be implemented more efficiently, guess how, and why?
		Self::get().is_some()
	}

	/// Set a new value into the storage.
	fn set(new_value: Self::Value) {
		let key = Self::raw_storage_key();
		sp_io::storage::set(&key, &new_value.encode())
	}

	/// Remove any value stored in this storage value.
	///
	/// noop if nothing exists.
	fn clear() {
		let key = Self::raw_storage_key();
		sp_io::storage::clear(&key)
	}

	/// Mutate the value in place based on the given `f`.
	///
	/// `f` will see `None` if the value doesn't exist, and `Some` if it does.
	///
	/// If the value doesn't exists, but `f` mutates the given `None` to `Some(_)`, it will be
	/// created.
	///
	/// If the value exists, but it is mutated to `None`, it will be removed.
	fn mutate(f: impl FnOnce(&mut Option<Self::Value>)) {
		let mut maybe_value = Self::get();
		f(&mut maybe_value);
		if let Some(value) = maybe_value {
			Self::set(value);
		} else {
			Self::clear()
		}
	}
}

pub trait StorageMap {
	/// The key type of this map.
	type Key: Encode + Clone;
	/// The value type of the map.
	type Value: Encode + Decode;

	/// The final storage key of the given `Self::key`.
	fn raw_storage_key(key: Self::Key) -> Vec<u8>;

	/// Get the value associated with `key`.
	fn get(key: Self::Key) -> Option<Self::Value> {
		let key = Self::raw_storage_key(key);
		sp_io::storage::get(&key)
			.and_then(|raw_value| <Self::Value as Decode>::decode(&mut &*raw_value).ok())
	}

	/// Check if the value exists in storage.
	fn exists(key: Self::Key) -> bool {
		// Could be implemented more efficiently, guess how, and why?
		Self::get(key).is_some()
	}

	/// Set a new `value` into the storage associated with `key`.
	fn set(key: Self::Key, value: Self::Value) {
		let key = Self::raw_storage_key(key);
		sp_io::storage::set(&key, value.encode().as_ref())
	}

	/// Remove any value associated with `key` from the storage.
	fn clear(key: Self::Key) {
		let key = Self::raw_storage_key(key);
		sp_io::storage::clear(&key)
	}

	/// Mutate the value associated with `key` in place based on the given `f`.
	///
	/// `f` will see `None` if the value doesn't exist, and `Some` if it does.
	///
	/// If the value doesn't exists, but `f` mutates the given `None` to `Some(_)`, it will be
	/// created.
	///
	/// If the value exists, but it is mutated to `None`, it will be removed.
	fn mutate(key: Self::Key, f: impl FnOnce(&mut Option<Self::Value>)) {
		let mut maybe_value = Self::get(key.clone());
		f(&mut maybe_value);
		if let Some(value) = maybe_value {
			Self::set(key.clone(), value);
		} else {
			Self::clear(key)
		}
	}
}
