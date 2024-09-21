# claim.it

<p align="center">
  <img src="https://github.com/turboflakes/claimit/blob/main/gh_header.png?raw=true">
</p>

**claim.it** ― It's a Decentralised Application (DApp) to easy lookup and claim child bounties from Polkadot and Kusama network. **claim.it** works for anyone looking to easily track and claim child bounties. **claim.it** is written in Rust and compiled to WASM to run entirely in the browser ([Subxt](https://github.com/paritytech/subxt) + [Yew](https://yew.rs/) + [TailwindCSS](https://tailwindcss.com/)).

## ✨ Included Features

- [&check;] Support Polkadot and Kusama network;
- [&check;] Light client first with optional switch to an RPC connection (default to IBP provider);
- [&check;] Mobile first support;
- [&check;] Onboard view in three steps for first time users;
- [&check;] Synchronize all on chain child bounties and filter by description;
- [&check;] Add any beneficiary account;
- [&check;] Switch view betweeen Total balance, Total awarded, Total Pending, Total claimable;
- [&check;] External links to Subsquare or Polkassembly child bounty;
- [&check;] Load accounts identity via people chain;
- [&check;] Signing via PolkadotJS, Talisman, Subwallet, Polkagate;

## 🚧 Work In Progress

- [] Disable accounts from being included in the claiming batch;
- [] Abstract the switch between networks. Have all accounts in the same view (TBD);
- [] Support additional wallets (TBD);

## Development / Build from Source

If you'd like to build from source, first install Rust.

```bash
#!/bin/bash
curl https://sh.rustup.rs -sSf | sh
```

If Rust is already installed run

```bash
#!/bin/bash
rustup update
```

Verify Rust installation by running

```bash
#!/bin/bash
rustc --version
```

Once done, finish installing the support software

```bash
#!/bin/bash
sudo apt install build-essential git clang libclang-dev pkg-config libssl-dev
```

Add WebAssembly target to your development environment

```bash
#!/bin/bash
rustup target add wasm32-unknown-unknown
```

Install Trunk

```bash
#!/bin/bash
cargo install --locked trunk
```

Build `claimit` by cloning this repository

```bash
#!/bin/bash
git clone http://github.com/turboflakes/claimit
```

Finally Use `trunk` to build and serve the app

```bash
#!/bin/bash
trunk serve
```

This project uses Tailwind CSS to write and generate styles, so to be able to get the project fully operational also install Tailwind CSS via npm or yarn

```bash
#!/bin/bash
npm install tailwindcss
```

Open a new terminal window and run 

```bash
#!/bin/bash
npm run watch
```

## Inspiration

Projects that had influence in **claim.it** design.

- <a href="https://github.com/paritytech/subxt/tree/master/examples/wasm-example" target="_blank">wasm-example</a> - This is a small WASM app using the Yew UI framework to showcase how to use Subxt's features in a WASM environment.
- <a href="https://github.com/yewstack/yew/tree/master/examples/function_todomvc" target="_blank">TodoMVC</a> - This is an implementation of TodoMVC for Yew using function components and hooks.

## Collaboration

Have an idea for a new feature, a fix or you found a bug, please open an [issue](https://github.com/turboflakes/crunch/issues) or submit a [pull request](https://github.com/turboflakes/crunch/pulls).

Any feedback is welcome.

## About

**claim.it** - was made by **Turboflakes**. Visit us at <a href="https://turboflakes.io" target="_blank" rel="noreferrer">turboflakes.io</a> to know more about our work.

If you like this project
  - 🚀 Share our work 
  - ✌️ Visit us at <a href="https://turboflakes.io" target="_blank" rel="noreferrer">turboflakes.io</a>
  - ✨ Or you could also star the Github project :)

### License

**claim.it** - The entire code within this repository is licensed under the [Apache License 2.0](./LICENSE).