# claimeer

Claimeer ― It's a simple UI to easy lookup and claim child bounties from Polkadot and Kusama network. Claimeer is written in Rust and compiled to WASM to run entirely in the browser (Subxt + Yew + TailwindCSS).

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

Build `claimeer` by cloning this repository

```bash
#!/bin/bash
git clone http://github.com/turboflakes/claimeer
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
