use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
	/// The current block number.
	block_number: u32,
	/// A map from an account to their nonce.
	nonce: BTreeMap<String, u32>,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> u32 {
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &String) {
		self.nonce.insert(who.clone(), *self.nonce.get(&who.clone()).unwrap_or(&0) + 1);
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

		let mut system_pallet = super::Pallet::new();
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
