# Thippy ‒- A Smart Contracts Parachain

This is a node implementation of Thippy, a Substrate parachain for smart contracts.


## Developing Smart Contracts for Thippy

This node is based on the 'parachain-template', which we configured to use Substrate's smart contracts module ‒ the `contracts` pallet.
This `contracts` pallet takes smart contracts as WebAssembly blobs and defines an API
for everything a smart contract needs (storage access, …).
As long as a programming language compiles to WebAssembly and there exists an implementation
of this API in it, you can write a smart contract for this pallet ‒ and thus for Thippy ‒ in
that language.

This is a list of languages you can currently choose from:

* [ask!](https://github.com/patractlabs/ask) for Assembly Script
* The [Solang](https://github.com/hyperledger-labs/solang) compiler for Solidity

There are also different user interfaces and command-line tools you can use to deploy
or interact with contracts:

* [polkadot-js](https://polkadot.js.org/apps/)

If you are looking for a quickstart, we can recommend
[ink!'s Guided Tutorial for Beginners](https://docs.substrate.io/tutorials/v3/ink-workshop/pt1/).

## Rococo Deployment

We have a live deployment of the Thippy parachain on [Rococo](https://wiki.polkadot.network/docs/build-pdk#rococo-testnet) ‒
a testnet for Polkadot and Kusama parachains.
You can interact with the network through Polkadot JS Apps,
[click here for a direct link to Thippy](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-Thippy-rpc.polkadot.io#/explorer).

The Thippy parachain uses the Rococo relay chain's native token (ROC) instead of having its own token.
Due to this you'll need ROC in order to deploy contracts on Thippy.

As a first step, you should create an account. See [here](https://wiki.polkadot.network/docs/learn-account-generation)
for a detailed guide.

As a second step, you have to get ROC testnet tokens through the [Rococo Faucet](https://wiki.polkadot.network/docs/learn-DOT#getting-rococo-tokens).
This is a chat room in which you need to write:
```
!drip YOUR_SS_58_ADDRESS:1002
```
The number `1002` is the parachain id of Thippy on Rococo, by supplying it the faucet will teleport ROC
tokens directly to your account on the parachain.

If everything worked out, the teleported ROC tokens will show up under 
[the "Accounts" tab for Thippy](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-Thippy-rpc.polkadot.io#/accounts).

Once you have ROC on Thippy you can deploy a contract as you would normally.
If you're unsure about this, our [guided tutorial](https://docs.substrate.io/tutorials/v3/ink-workshop/pt1/) 
will clarify that for you in no time.


## Run it Locally

### Installation

You need to have executables of both Polkadot and Thippy.

Binary releases can be found here:
[Polkadot releases](https://github.com/ng8eke/polkadot/releases),
[Thippy releases](https://github.com/ng8eke/Thippy/releases).

Alternatively you can install (or build) the projects yourself:

* `cargo install --git https://github.com/ng8eke/polkadot.git --force --locked`
* `cargo install --git https://github.com/ng8eke/Thippy.git --force --locked`

The `--locked` flag makes the installation (or build) use the same versions as the `Cargo.lock`
in those repositories ‒ ensuring that the last known-to-work version of the dependencies is used.

### Launching

Starting this project is best done using the [`polkadot-launch`](https://github.com/ng8eke/polkadot-launch) tool ‒
it starts Polkadot and registers the Thippy parachain on it automatically.

To use `polkadot-launch` you must ensure that you're using Node.js `v14.x` or newer. If
you're on macOS, you must also ensure that your machine's firewall is disabled. You can
do this by going to "System Preferences" > "Security & Privacy" > "Firewall" and ensuring
that it's off.

Now that you have the prerequisites:
1. Install it with `yarn global add polkadot-launch` or `npm i polkadot-launch -g`
1. Check that the paths in `polkadot-launch/config.json` point to the `polkadot` and `Thippy` executables
1. Launch the network with `polkadot-launch polkadot-launch/config.json`

At this point you should be able to use [Polkadot JS Apps](https://polkadot.js.org/apps/)
to connect to the Polkadot relay chain nodes as well as the Thippy collator.

## Building from source

Follow the [official installation steps](https://docs.substrate.io/v3/getting-started/installation/)
to set up all Substrate prerequisites.

Afterwards you can install this node via

```bash
git clone https://github.com/ng8eke/Thippy.git
cd Thippy/
cargo build --release --locked 
```

### Unstable Features

If you're the type of person that likes to drink your soup before it cools, you might
also be wondering about how to activate unstable `pallet-contracts` features. To do this
you can run the previous installation command with the following flag: 
`--features contracts-unstable-interface`.
