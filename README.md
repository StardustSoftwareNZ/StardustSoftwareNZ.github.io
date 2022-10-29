# StardustSoftwareNZ.github.io
Stardust Software NZ Website

## Organisation 

This repository contains these directories. 

```
.
├── src
├── Cargo.toml
├── index.html
├── LICENSE
├── README.md
├── styles.css
└── Trunk.toml
```

- [**srcs**](src) contains the source files for the website.
- [**Cargo.toml**](Cargo.toml) contains the dependencies for the website.
- [**index.html**](index.html) loads the rust wasm file.
- [**LICENSE**](LICENSE) biolerplate MIT license.
- [**README.md**](README.md) this file.
- [**styles.css**](styles.css) contains the css for the website.
- [**Trunk.toml**](Trunk.toml) contains config for the trunk utility.

## Rust Frontend 

Build a simple Rust frontend web application using Yew. 

Tutorial https://www.youtube.com/watch?v=MddGbXgIt2E&t=3s

## Installations 

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
$ cargo new website 
```

## Serve 

Run `trunk serve` to run the application on your local machine.

```bash
$ trunk serve
```
