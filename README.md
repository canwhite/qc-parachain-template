# Polkadot SDK's Parachain Template

test for Polkadot parachain

## env: macos

1. To install Homebrew:

```
1) install brew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"

2) test
brew --version

```

2. Support for Apple Silicon

```
brew install protobuf
```

3. brew update
4. brew install openssl
5. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
6. source ~/.cargo/env
7. rustup default stable
8. rustup update
9. rustup target add wasm32-unknown-unknown
10. rustup update nightly
11. rustup target add wasm32-unknown-unknown --toolchain nightly
12. brew install cmake

## run

1. add utility Tools

```
1) cargo install --locked staging-chain-spec-builder@10.0.0
2) cargo install --locked polkadot-omni-node@0.5.0
```

2. get repo and build

```
1) git clone -b stable2412 https://github.com/paritytech/polkadot-sdk-parachain-template.git parachain-template
2) cd parachain-template
3) cargo build --release --locked
```

3. Generate the chain specification file of your parachain:

```
chain-spec-builder create -t development \
--relay-chain paseo \
--para-id 1000 \
--runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
named-preset development
```

4. Start the omni node with the generated chain spec. You'll start it in development mode (without a relay chain config), producing and finalizing blocks:

run:

```
polkadot-omni-node --chain ./chain_spec.json --dev
```

## interact

1. open [polkadot app](https://polkadot.js.org/apps/#/explorer)
2. link custom node

```
Connect to your local node:

1) Scroll to the bottom and select Development
2) Choose Custom
3) Enter ws://localhost:9944 in the input field
4) Click the Switch button
```

## test

```
argo test --package custom-pallet
```
