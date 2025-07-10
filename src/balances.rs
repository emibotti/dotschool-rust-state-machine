use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	type Balance: CheckedAdd + CheckedSub + Zero + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	pub fn transfer(
		&mut self,
		caller: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> crate::support::DispatchResult {
		/*
			- Get the balance of account `caller`.
			- Get the balance of account `to`.

			- Use safe math to calculate a `new_caller_balance`.
			- Use safe math to calculate a `new_to_balance`.

			- Insert the new balance of `caller`.
			- Insert the new balance of `to`.
		*/

		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

		self.set_balance(&caller, new_caller_balance);
		self.set_balance(&to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Balance = u128;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<TestConfig>::new();

		/* Assert that the balance of `alice` starts at zero. */
		assert_eq!(balances.balance(&String::from("alice")), 0);

		/* Set the balance of `alice` to 100. */
		balances.set_balance(&String::from("alice"), 100);

		/* Assert the balance of `alice` is now 100. */
		assert_eq!(balances.balance(&String::from("alice")), 100);

		/* Assert the balance of `bob` has not changed and is 0. */
		assert_eq!(balances.balance(&String::from("bob")), 0);
	}

	#[test]
	fn transfer_balance() {
		/* Create a test that checks the following:
			- That `alice` cannot transfer funds she does not have.
			- That `alice` can successfully transfer funds to `bob`.
			- That the balance of `alice` and `bob` is correctly updated.
		*/
		let mut balances = super::Pallet::<TestConfig>::new();
		balances.set_balance(&String::from("alice"), 100);

		/* Assert the balance of `alice` is now 100. */
		assert_eq!(balances.balance(&String::from("alice")), 100);

		/* Assert the balance of `bob` has not changed and is 0. */
		assert_eq!(balances.balance(&String::from("bob")), 0);

		assert_eq!(
			balances.transfer(String::from("alice"), String::from("bob"), 150),
			Result::Err("Not enough funds.")
		);

		assert_eq!(
			balances.transfer(String::from("alice"), String::from("bob"), 50),
			Result::Ok(())
		);

		assert_eq!(balances.balance(&String::from("bob")), 50);
		assert_eq!(balances.balance(&String::from("alice")), 50);
	}
}
