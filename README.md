# StardustSoftwareNZ.github.io
Stardust Software NZ Website

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
