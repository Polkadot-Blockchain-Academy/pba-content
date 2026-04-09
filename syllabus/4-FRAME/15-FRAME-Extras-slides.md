---
title: FRAME Extras
description: Additional FRAME Topics
---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# FRAME Extras

---

## Follow Along!

These are topics where it can be easier to just learn by looking through the code and discussing.

---

## Runtime Call Filters

---

## Account Reference Counters

---

## View Functions

View functions are read-only query methods exposed from a pallet. They are callable via the `state_call` RPC — no transaction needed, no state changes.

```rust
#[pallet::view_functions]
impl<T: Config> Pallet<T> {
  /// Query a single storage value.
  pub fn get_value() -> Option<u32> {
    SomeValue::<T>::get()
  }

  /// Query with input arguments.
  pub fn get_value_with_arg(key: u32) -> Option<u32> {
    SomeMap::<T>::get(key)
  }
}
```

---v

### View Functions: Real-World Example

From `pallet-proxy`:

```rust
#[pallet::view_functions]
impl<T: Config> Pallet<T> {
  /// Check if a `RuntimeCall` is allowed for a given `ProxyType`.
  pub fn check_permissions(
    call: <T as Config>::RuntimeCall,
    proxy_type: T::ProxyType,
  ) -> bool {
    proxy_type.filter(&call)
  }
}
```

- Defined in a separate `impl` block with `#[pallet::view_functions]`
- Functions must be `pub` and return a value
- Used by off-chain tools, UIs, and other clients to query pallet state

---

## Tasks (Experimental)

Tasks define background work that can be executed across blocks. They are submitted as unsigned transactions and processed when conditions are met.

```rust
#[pallet::tasks_experimental]
impl<T: Config> Pallet<T> {
  #[pallet::task_list(Numbers::<T>::iter_keys())]
  #[pallet::task_condition(|i| Numbers::<T>::contains_key(i))]
  #[pallet::task_weight(T::WeightInfo::add_number_into_total())]
  #[pallet::task_index(0)]
  pub fn add_number_into_total(i: u32) -> DispatchResult {
    let v = Numbers::<T>::take(i).ok_or(Error::<T>::NotFound)?;
    Total::<T>::mutate(|total| *total += v);
    Ok(())
  }
}
```

---v

### Tasks: Attributes

| Attribute                                  | Purpose                              |
| ------------------------------------------ | ------------------------------------ |
| `#[pallet::task_index(N)]`                 | Unique identifier for this task type |
| `#[pallet::task_condition(\|args\| bool)]` | Predicate: should this task execute? |
| `#[pallet::task_list(iterator)]`           | Iterator of pending task items       |
| `#[pallet::task_weight(expr)]`             | Weight consumed by this task         |

- The macro generates a `Task<T>` enum from all task functions
- Tasks are submitted via `frame_system::do_task` (requires `experimental` feature flag)
- Can be discovered and submitted by offchain workers
- This API is experimental and may change

---

## Pallet Instances
