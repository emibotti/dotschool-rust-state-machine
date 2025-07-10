use std::{collections::BTreeMap, ops::AddAssign};

use num::{CheckedAdd, One, Zero};

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<AccountID, BlockNumber, Nonce> {
	/// The current block number.
	block_number: BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<AccountID, Nonce>,
}

impl<AccountID, BlockNumber, Nonce> Pallet<AccountID, BlockNumber, Nonce>
where
	AccountID: Ord + Clone,
	BlockNumber: Zero + Copy + AddAssign + One,
	Nonce: Zero + CheckedAdd + One + Copy,
{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += BlockNumber::one()
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &AccountID) {
		self.nonce.insert(
			who.clone(),
			*self.nonce.get(&who.clone()).unwrap_or(&Nonce::zero()) + Nonce::one(),
		);
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		/*
			- Increment the current block number.
			- Increment the nonce of `alice`.

			- Check the block number is what we expect.
			- Check the nonce of `alice` is what we expect.
			- Check the nonce of `bob` is what we expect.
		*/

		let mut system_pallet = super::Pallet::<String, u32, u32>::new();
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
