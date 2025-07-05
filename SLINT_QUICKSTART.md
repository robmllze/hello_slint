# Slint + Rust Quick Start Guide

A cheat sheet for creating and running a new Slint application for both native desktop and the web.

## 1. Prerequisites

Ensure your Rust toolchain and required build tools are installed.

```sh
# Update Rust to the latest version
rustup update

# Install the project generator tool
cargo install cargo-generate

# Install the WebAssembly build tool
cargo install wasm-pack
```

**Recommended:** Install the official [Slint VS Code Extension](https://marketplace.visualstudio.com/items?itemName=Slint.slint) for live-previews and syntax highlighting.

## 2. Create a New Project

Generate a new Slint project from the official template.

```sh
# Run the generator
cargo generate --git https://github.com/slint-ui/slint-rust-template

# When prompted, enter your project name (e.g., "hello_slint")
? Project Name: hello_slint

# Navigate into your new project
cd hello_slint
```

> **Note:** Open `Cargo.toml` and verify `name = "hello_slint"` is set correctly.

## 3. Define the UI (`ui/app-window.slint`)

Describe your user interface in the `.slint` file.

```slint
import { Button } from "std-widgets.slint";

export component App inherits Window {
    in-out property <string> greeting: "Hello, Slint!";
    in-out property <int> counter: 0;
    callback button-pressed();

    VerticalLayout {
        padding: 20px;
        spacing: 10px;
        Text {
            text: greeting;
        }

        Button {
            text: "Click me (Pressed \{counter} times)";
            clicked => {
                button-pressed()
            }
        }
    }
}
```

## 4. Write the Logic (`src/main.rs`)

Connect your UI to Rust code to handle events and state.

```rust
slint::include_modules!();

fn main() {
    let ui = App::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_button_pressed(move || {
        let ui = ui_handle.upgrade().unwrap();
        let current_counter = ui.get_counter();
        ui.set_counter(current_counter + 1);
    });

    ui.run().unwrap();
}
```

## 5. Run Native Desktop App

Test your application on your desktop.

```sh
cargo run
```

## 6. Build & Run for the Web (PWA)

Compile your app to WebAssembly and serve it locally.

#### A. Configure for Web

1.  **Add WASM Target:** (One-time setup)

```sh
rustup target add wasm32-unknown-unknown
```

2.  **Edit `Cargo.toml`:** Add the `[lib]` section and `wasm-bindgen` dependency to make your project a web-compatible library.

```toml
[package]
name = "hello_slint"
version = "0.1.0"
edition = "2021"

[dependencies]
slint = "1.8.0"
wasm-bindgen = "0.2.100"

[build-dependencies]
slint-build = "1.8.0"

[lib]
crate-type = ["cdylib", "rlib"]
```

3.  **Create Library Entrypoint (`src/lib.rs`):**

```rust
use wasm_bindgen::prelude::*;
slint::include_modules!();

#[wasm_bindgen(start)]
pub fn run_app() {
    // ... paste the entire contents of your main() function here ...
    // (the `let ui = ...` and `ui.run()` parts)
}
```

4.  **Simplify `src/main.rs`:**

```rust
use hello_slint::run_app; // Use your project's name here
fn main() { run_app(); }
```

#### B. Build the Web Package

Compile your app and package it for deployment. This creates a `public/pkg/` directory.

```sh
wasm-pack build --target web --out-dir public
```

#### C. Create the Host `index.html` File

Create an `index.html` in the `public/` directory.

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Hello Slint</title>
  </head>
  <body>
    <canvas id="canvas"></canvas>
    <script type="module">
      import init from "/pkg/hello_slint.js";
      async function run() {
        await init();
      }
      run();
    </script>
  </body>
</html>
```

#### D. Serve Locally

Start a simple web server from your project's root directory.

```sh
cd public
python3 -m http.server
```

Open your browser to `http://0.0.0.0:8000`. Stop the server with `Ctrl+C`.

## 7. Deploy on Firebase

```sh
firebase login --reauth
firebase init
firebase deploy
```
