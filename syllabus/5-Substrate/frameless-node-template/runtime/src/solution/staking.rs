use sp_runtime::DispatchOutcome;

use super::{currency::CryptoCurrency, Dispatchable};
use crate::shared::*;

/// The configuration trait for this module.
pub trait Config {
	/// Some type that can provide the currency functionality to this module.
	type Currency: CryptoCurrency;
}

/// Just a type alias to make it easier to access the balance type coming in from
/// `Config::Currency::Balance`. Try using `Config::Currency::Balance` directly and see why it
/// won't work. Ruminate a lot on this, make sure you get it!
type BalanceOf<T> = <<T as Config>::Currency as CryptoCurrency>::Balance;

/// Just a wrapper for this module's implementations.
///
/// Note that this struct is itself public, but the internal implementations are not. The public
/// interface of each module is its `Call` (followed by calling `dispatch` on it), not `Module`.
pub struct Module<T: Config>(sp_std::marker::PhantomData<T>);
impl<T: Config> Module<T> {
	fn bond(sender: AccountId, amount: BalanceOf<T>) -> DispatchOutcome {
		<T::Currency as CryptoCurrency>::reserve(sender, amount)
	}
}

/// This module's `Call` enum.
///
/// Contains all of the operations, and possible arguments (except `sender`, of course).
pub enum Call<T: Config> {
	/// Bond `amount` form the `sender`, if they have enough free balance.
	Bond { amount: BalanceOf<T> },
}

impl<T: Config> Dispatchable for Call<T> {
	fn dispatch(self, sender: AccountId) -> DispatchOutcome {
		match self {
			Call::Bond { amount } => Module::<T>::bond(sender, amount),
		}
	}
}
