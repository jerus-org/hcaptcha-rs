# Example: hcaptcha-wasm

Simple example project using `wasm-bindgen` to embed a [Hcaptcha](https://www.hcaptcha.com/) widget into your WebAssembly application.

## Creating the project

Project was created using wasm-pack:

```sh
wasm-pack new hcaptcha-wasm

```

Then edited to create the example.

## Running the example

The code compiles and runs with the current node lts version (v20.18.0).

The test module runs the example using wasm-pack test:

```sh
nvm use v20.18.0
wasm-pack test --node

```
