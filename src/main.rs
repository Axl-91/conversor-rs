//! # Conversor-rs
//!
//! A small but polished desktop currency converter built with `eframe` and
//! `egui`. On startup it fetches live exchange rates from the
//! [Frankfurter](https://frankfurter.dev) API and lets the user convert
//! between three currencies:
//!
//! - **ARS** — Argentine peso
//! - **USD** — US dollar
//! - **EUR** — Euro
//!
//! ## Module layout
//!
//! - [`api`] fetches and stores the exchange rates.
//! - [`currency`] defines the supported currencies and the conversion math.
//! - [`app`] holds the application state and the `eframe::App` implementation.
//! - [`ui`] draws the interface and reports user actions.
//! - [`theme`] centralizes the color palette used by the UI.
//!
//! ## Running
//!
//! ```sh
//! cargo run --release
//! ```
//!
//! An internet connection is required when the application starts in order to
//! retrieve the exchange rates.

mod api;
mod app;
mod currency;
mod theme;
mod ui;

use api::ExchangeRates;
use app::CurrencyConverter;
use eframe::egui;

fn main() -> eframe::Result<()> {
    // Fetch the rates once, before the window opens, and hand them to the app.
    let exchange_rates = ExchangeRates::fetch();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([440.0, 430.0])
            .with_min_inner_size([400.0, 390.0])
            .with_close_button(true),
        ..Default::default()
    };

    eframe::run_native(
        "Currency Converter",
        options,
        Box::new(|_creation_context| Ok(Box::new(CurrencyConverter::new(exchange_rates)))),
    )
}
