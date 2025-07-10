mod balances;
mod support;
mod system;

use crate::{support::Dispatch, types::Extrinsic};

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
	use crate::{RuntimeCall, support};

	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = support::Extrinsic<AccountId, RuntimeCall>;
	pub type Header = support::Header<BlockNumber>;
	pub type Block = support::Block<Header, Extrinsic>;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
	BalancesTransfer { to: types::AccountId, amount: types::Balance },
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}

	// Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		/*
			- Increment the system's block number.
			- Check that the block number of the incoming block matches the current block number,
			  or return an error.
			- Iterate over the extrinsics in the block...
				- Increment the nonce of the caller.
				- Dispatch the extrinsic using the `caller` and the `call` contained in the extrinsic.
				- Handle errors from `dispatch` same as we did for individual calls: printing any
				  error and capturing the result.
				- You can extend the error message to include information like the block number and
				  extrinsic number.
		*/

		self.system.inc_block_number();
		if block.header.block_number != self.system.block_number() {
			return Err("Incoming block does not match currect block number");
		}

		for (i, types::Extrinsic { call, caller }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(&caller);
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}
		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		/*
			Use a match statement to route the `runtime_call` to call the appropriate function in
			our pallet. In this case, there is only `self.balances.transfer`.

			Your `runtime_call` won't contain the caller information which is needed to make the
			`transfer` call, but you have that information from the arguments to the `dispatch`
			function.

			You should propagate any errors from the call back up this function.
		*/
		match runtime_call {
			RuntimeCall::BalancesTransfer { to, amount } => {
				self.balances.transfer(caller, to, amount)?;
			},
		}
		Ok(())
	}
}

fn main() {
	/* Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
	let mut runtime = Runtime::new();

	let alice = "alice".to_string();
	let bob = "bob".to_string();

	/* Set the balance of `alice` to 100, allowing us to execute other transactions. */
	runtime.balances.set_balance(&alice, 100);

	/*
		Replace the logic with a new `Block`.
			- Set the block number to 1 in the `Header`.
			- Move your existing transactions into extrinsic format, using the
			  `Extrinsic` and `RuntimeCall`.
	*/
	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![Extrinsic {
			call: RuntimeCall::BalancesTransfer { to: bob.clone(), amount: 30 },
			caller: alice.clone(),
		}],
	};

	/*
		Use your `runtime` to call the `execute_block` function with your new block.
		If the `execute_block` function returns an error, you should panic!
		We `expect` that all the blocks being executed must be valid.
	*/
	runtime.execute_block(block_1).expect("invalid block");

	println!("{:#?}", runtime);
}
