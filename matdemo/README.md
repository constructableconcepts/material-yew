# matdemo

A demo web application for Material Yew components.

## Prerequisites
- [Rust](https://rustup.rs/) (with wasm32 target)
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)

## Build and Run

1. Install Trunk if you haven't:
   ```bash
   cargo install trunk
   ```
2. Add the wasm32 target if needed:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. Build and serve the app:
   ```bash
   trunk serve --open
   ```
   This will open the demo at http://localhost:8080

## Project Structure
- `src/main.rs`: App entry point
- `src/pages.rs`: Demo pages for each component
- `index.html`: Main HTML file
- `Trunk.toml`: Trunk configuration

## Notes
- The demo showcases all available Material Yew components with simple usage examples.
