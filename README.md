# StardustSoftwareNZ.github.io

Stardust Software NZ Website

## Rust Frontend

Build a simple Rust frontend web application using Yew.

Tutorial https://www.youtube.com/watch?v=MddGbXgIt2E

## Organisation

This repository contains these directories.

```
.
├── docs
├── src
├── Cargo.toml
├── index.html
├── LICENSE
├── README.md
├── styles.css
└── Trunk.toml
```

- [**docs**](docs) contains the production build of the website.
- [**srcs**](src) contains the source files for the website.
- [**Cargo.toml**](Cargo.toml) contains the dependencies for the website.
- [**index.html**](index.html) loads the rust wasm file.
- [**LICENSE**](LICENSE) biolerplate MIT license.
- [**README.md**](README.md) this file.
- [**styles.css**](styles.css) contains the css for the website.
- [**Trunk.toml**](Trunk.toml) contains config for the trunk utility.

## Installation

Trunk is a crate that helps us build and package web applications.

```bash
$ cargo install trunk
```

Add web assembly as a compilation target to rust.

```bash
$ rustup target add wasm32-unknown-unknown
```

Create a package

```bash
$ cargo new my-app
```

Change directory

```bash
$ cd my-app
```

## Serve

Run `trunk serve` to run the application on your local machine.

```bash
$ trunk serve
```

Note: the `Trunk.toml` file contains the configuration for the trunk utility.

## Build

Build the application using `trunk build` and specify the `--release` tag. This generates the deployment ready files in the `docs` directory.

```bash
$ trunk build --release
```
