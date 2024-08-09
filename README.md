# Wonop UI - Tailwind components for YEW (BETA)

Wonop UI is a parameterized UI framework that leverages Tailwind CSS for use with the Yew framework in Rust. It provides a set of customizable components and utilities to streamline the development of web applications using Yew.

## Features

- Seamless integration with Yew framework
- Utilizes Tailwind CSS for rapid and flexible styling
- Parameterized components for easy customization
- Responsive design out of the box

## Installation

To use Wonop UI in your Yew project, add the following to your `Cargo.toml`:

```toml
[dependencies]
wonopui = { git = "https://github.com/wonop-io/wonopui.git", branch="main" }
```

Then initialise Tailwind CSS:

```bash
npx tailwindcss init
```

Add the following to your `tailwind.config.js`:

```js
module.exports = {
  content: ["./src/**/*.rs", "./target/wonopui.json", "./target/tailwindcss.txt"],
  theme: {
    extend: {},
  },
  plugins: [],
};
```

Add the following to your `index.html`:

```html
<link data-trunk rel="tailwind-css" href="tailwind.css" />
```

## Usage

TODO: To be written

