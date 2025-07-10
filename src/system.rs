use std::{collections::BTreeMap, ops::AddAssign};

use num::{One, Zero};

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + AddAssign + Copy;
	type Nonce: Zero + One + Copy;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one()
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		self.nonce.insert(
			who.clone(),
			*self.nonce.get(&who.clone()).unwrap_or(&T::Nonce::zero()) + T::Nonce::one(),
		);
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		/*
			- Increment the current block number.
			- Increment the nonce of `alice`.

			- Check the block number is what we expect.
			- Check the nonce of `alice` is what we expect.
			- Check the nonce of `bob` is what we expect.
		*/

		let mut system_pallet = super::Pallet::<TestConfig>::new();
		let alice = "alice".to_string();
		let bob = "bob".to_string();

		assert_eq!(system_pallet.block_number(), 0);

		system_pallet.inc_block_number();

		assert_eq!(system_pallet.block_number(), 1);

		system_pallet.inc_nonce(&alice);
		assert_eq!(system_pallet.nonce.get(&alice), Some(&1));
		assert_eq!(system_pallet.nonce.get(&bob), None);
	}
}
