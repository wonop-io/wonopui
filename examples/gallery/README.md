# WonopUI Gallery

A showcase of WonopUI components built with [Yew](https://yew.rs/) and [Trunk](https://trunkrs.dev/).

## Prerequisites

WonopUI components are styled with Tailwind CSS v4 and use shadcn/ui v4 design patterns. To get full functionality (including animations), you need:

1. **Tailwind CSS v4** - For utility classes
2. **tw-animate-css** - For component animations (Dialog, Popover, Dropdown, etc.)

## Quick Start

### 1. Install Rust and WASM target

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

### 2. Install Tailwind CSS

Follow the [Tailwind CSS installation guide](https://tailwindcss.com/docs/installation).

### 3. Add Animation Support

WonopUI components use shadcn/ui v4 animations. Download `tw-animate.css`:

```bash
curl -sL "https://unpkg.com/tw-animate-css@1.4.0/dist/tw-animate.css" -o tw-animate.css
```

Then import it in your CSS file (before tailwindcss):

```css
@import "./tw-animate.css";
@import "tailwindcss";
```

### 4. Run the Gallery

```bash
trunk serve
```

## Tailwind Configuration

Your `tailwind.config.js` should include the content paths for WonopUI:

```js
module.exports = {
  content: [
    "./src/**/*.rs",
    "./node_modules/wonopui/**/*.rs",  // If using npm
    "../../crates/**/*.rs",             // If using workspace
  ],
  darkMode: "class",
  theme: {
    extend: {},
  },
  plugins: [],
};
```

## CSS File Structure

Your main CSS file should look like:

```css
@config "./tailwind.config.js";
@import "./tw-animate.css";
@import "tailwindcss";
```

---

## Development

For a more thorough explanation of Trunk and its features, please head over to the [repository][trunk].

### Installation

If you don't already have it installed, it's time to install Rust: <https://www.rust-lang.org/tools/install>.
The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now that we have our basics covered, it's time to install the star of the show: [Trunk].
Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

That's it, we're done!

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

## Using this template

There are a few things you have to adjust when adopting this template.

### Remove example code

The code in [src/main.rs](src/main.rs) specific to the example is limited to only the `view` method.
There is, however, a fair bit of Sass in [index.scss](index.scss) you can remove.

### Update metadata

Update the `name`, `version`, `description` and `repository` fields in the [Cargo.toml](Cargo.toml) file.
The [index.html](index.html) file also contains a `<title>` tag that needs updating.

Finally, you should update this very `README` file to be about your app.

### License

The template ships with both the Apache and MIT license.
If you don't want to have your app dual licensed, just remove one (or both) of the files and update the `license` field in `Cargo.toml`.

There are two empty spaces in the MIT license you need to fill out: ``and`Troels F. Rønnow <troels@wonop.com>`.

[trunk]: https://github.com/thedodd/trunk
