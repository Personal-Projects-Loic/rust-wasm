# Rust and Web Assembly

The goal of the project is to learn how to encapsulate rust code in a web page using wasm compilation

# Project built with

```bash
cargo new --lib project-name
```

You **MUST** use a lib, for this project, tbh i don't know yet why...

# Installed packages

```bash
cargo install wasm-pack
cargo install cargo-make
cargo install simple-http-server
```

# How to use it

- Create a static directory at the root of the cargo project
- Add an index.html :

```html
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8" />
        <title>Project-name</title>
        <script type="module">
            import init from "./wasm.js";
            init();
        </script>
        <link rel="shortcut icon" href="#" />
    </head>
    <body></body>
</html>
```

- We're using **yew** and **wasm-bindgen** to compile and use rust with web-assembly, so you need to add it in your Cargo.toml :

```Makefile
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
yew = "0.17"
wasm-bindgen = "0.2"
```

- Now, create a Makefile.toml: cargo-make task runner enables to define and configure sets of tasks and run them as a flow.
There's a lot of exemples on internet, so look it up.

# Bulding commmands

```bash
cargo make build
```
- Generate configuration files in ./static/ directory needed by wasm

```bash
cargo make serve
```
- Starts the server, go to http://localhost:3000
