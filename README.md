# claimeer

<p align="center">
  <img src="https://github.com/turboflakes/claimeer/blob/main/claimeer_github_header.png?raw=true">
</p>

Claimeer ― It's a simple UI (dApp) to easy lookup and claim child bounties from Polkadot and Kusama network. Claimeer works for anyone looking to easily track and claim child bounties. Claimeer is written in Rust and compiled to WASM to run entirely in the browser (Subxt + Yew + TailwindCSS).

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

## Collaboration

Have an idea for a new feature, a fix or you found a bug, please open an [issue](https://github.com/turboflakes/crunch/issues) or submit a [pull request](https://github.com/turboflakes/crunch/pulls).

Any feedback is welcome.

## About

Claimeer - was made by **Turboflakes**. Visit us at <a href="https://turboflakes.io" target="_blank" rel="noreferrer">turboflakes.io</a> to know more about our work.

If you like this project
  - 🚀 Share our work 
  - ✌️ Visit us at <a href="https://turboflakes.io" target="_blank" rel="noreferrer">turboflakes.io</a>
  - ✨ Or you could also star the Github project :)

Tips are welcome

- Polkadot 14Sqrs7dk6gmSiuPK7VWGbPmGr4EfESzZBcpT6U15W4ajJRf (turboflakes.io)
- Kusama H1tAQMm3eizGcmpAhL9aA9gR844kZpQfkU7pkmMiLx9jSzE (turboflakes.io)

### License

Claimeer - The entire code within this repository is licensed under the [Apache License 2.0](./LICENSE).