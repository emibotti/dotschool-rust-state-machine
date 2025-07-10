use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	/// Get the owner (if any) of a claim.
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(&claim)
	}

	/// Create a new claim on behalf of the `caller`.
	/// This function will return an error if someone already has claimed that content.
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		/* Check that a `claim` does not already exist. If so, return an error. */
		if self.claims.contains_key(&claim) {
			return Err(&"this content is already claimed");
		}
		/* `insert` the claim on behalf of `caller`. */
		self.claims.insert(claim, caller);
		Ok(())
	}

	/// Revoke an existing claim on some content.
	/// This function should only succeed if the caller is the owner of an existing claim.
	/// It will return an error if the claim does not exist, or if the caller is not the owner.
	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let res = self
			.get_claim(&claim)
			.is_some_and(|owner| (*owner) == caller)
			.then(|| self.claims.remove(&claim));

		match res {
			Some(_) => Ok(()),
			None => Err("Failed to revoke the claim"),
		}
	}
}

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum Call<T: Config> {
	CreateClaim { claim: T::Content },
	RevokeClaim { claim: T::Content },
}

/// Implementation of the dispatch logic, mapping from `POECall` to the appropriate underlying
/// function we want to execute.
impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountId;
	type Call = Call<T>;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
		match call {
			Call::CreateClaim { claim } => self.create_claim(caller, claim)?,
			Call::RevokeClaim { claim } => self.revoke_claim(caller, claim)?,
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut proof_of_existence = super::Pallet::<TestConfig>::new();

		let alice = "alice";
		let bob = "bob";
		let content = "image.jpg";

		// Checks initial state
		assert_eq!(proof_of_existence.get_claim(&content), None);
		assert_eq!(
			proof_of_existence.revoke_claim(&alice, &content),
			Err("Failed to revoke the claim")
		);

		// Saves an image that alice owns
		assert_eq!(proof_of_existence.create_claim(&alice, &content), Ok(()));
		assert_eq!(proof_of_existence.get_claim(&content), Some(&alice));

		// Saves the same image to a different owner
		assert_eq!(
			proof_of_existence.create_claim(&bob, &content),
			Err("this content is already claimed")
		);

		// Tries to revoke alice's image in behalf of bob
		assert_eq!(
			proof_of_existence.revoke_claim(&bob, &content),
			Err("Failed to revoke the claim")
		);

		// Revokes successfully alice's image
		assert_eq!(proof_of_existence.revoke_claim(&alice, &content), Ok(()));
	}
}
