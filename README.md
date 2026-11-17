# Conversor-rs

A small desktop currency converter written in Rust with `eframe` and `egui`. 

It downloads live USD→ARS and USD→EUR rates from the [Frankfurter](https://frankfurter.dev) API at startup, stores them relative to USD, and uses them to convert between Argentine pesos, US dollars and euros in either direction. 

## Requirements

- Rust (edition 2024) and Cargo
- An internet connection when the application starts, to download the exchange rates

## Build and run

```sh
cargo build --release
cargo run --release
```

The executable is placed in `target/release/conversor-rs` (`conversor-rs.exe` on Windows).

## Usage

Choose the source and target currencies, type an amount and press Enter or click Convert. 

## How it works

At startup the app requests `GET https://api.frankfurter.dev/v2/rate/USD/ARS` and `GET https://api.frankfurter.dev/v2/rate/USD/EUR`. 

Both rates are stored as units of currency per 1 USD (USD itself is stored as 1.0), so any pair can be converted with `amount / rate(from) * rate(to)`. 

Rates are fetched once, so restart the app to refresh them. If the API cannot be reached, the app still opens but shows an error and disables conversions.

## Project structure

- `src/main.rs` — entry point and module declarations
- `src/api.rs` — exchange-rate API client and rate snapshot
- `src/currency.rs` — supported currencies and conversion math, with unit tests
- `src/app.rs` — application state and conversion workflow
- `src/ui.rs` — interface drawing and user actions
- `src/theme.rs` — shared color palette

## Tests and documentation

```sh
cargo test
cargo doc --no-deps --open
```
