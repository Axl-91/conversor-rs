//! Application state and conversion workflow.
//!
//! This module owns the data shown in the window. It delegates all drawing to
//! the [`ui`] module and applies the actions the user triggers.

use std::error::Error;

use eframe::egui;

use crate::{
    api::ExchangeRates,
    currency::Currency,
    ui::{self, Action, MessageKind},
};

/// The state of the currency converter application.
///
/// This type implements [`eframe::App`], which is the entry point egui calls
/// once per frame to draw the interface.
pub(crate) struct CurrencyConverter {
    /// The rates fetched on startup, or `None` if fetching failed.
    pub(crate) exchange_rates: Option<ExchangeRates>,
    /// Raw text entered by the user in the amount field.
    pub(crate) amount: String,
    /// The currency to convert from.
    pub(crate) from: Currency,
    /// The currency to convert to.
    pub(crate) to: Currency,
    /// The message displayed in the result area.
    pub(crate) message: String,
    /// The style of `message`.
    pub(crate) message_kind: MessageKind,
}

impl CurrencyConverter {
    /// Builds a new converter from the result of fetching the exchange rates.
    pub(crate) fn new(rates: Result<ExchangeRates, Box<dyn Error>>) -> Self {
        match rates {
            Ok(rates) => Self {
                exchange_rates: Some(rates),
                amount: String::new(),
                from: Currency::Ars,
                to: Currency::Usd,
                message: "Enter an amount and press Convert.".to_owned(),
                message_kind: MessageKind::Info,
            },
            Err(error) => Self {
                exchange_rates: None,
                amount: String::new(),
                from: Currency::Ars,
                to: Currency::Usd,
                message: format!("Could not load exchange rates: {error}"),
                message_kind: MessageKind::Error,
            },
        }
    }

    /// Performs the conversion using the current input and selected currencies.
    ///
    /// The result (or an error message) is stored in `message` so the next
    /// frame can display it.
    fn convert(&mut self) {
        let Some(rates) = &self.exchange_rates else {
            self.set_error("Exchange rates are not available. Please restart the app.");
            return;
        };

        // Accept both `.` and `,` as the decimal separator.
        let amount = match self.amount.trim().replace(',', ".").parse::<f64>() {
            Ok(value) if value.is_finite() => value,
            _ => {
                self.set_error("Please enter a valid number.");
                return;
            }
        };

        let converted = rates.convert(amount, self.from, self.to);
        self.message = format!(
            "{amount:.2} {} = {converted:.2} {}",
            self.from.code(),
            self.to.code()
        );
        self.message_kind = MessageKind::Success;
    }

    /// Stores an error message to be shown in the result area.
    fn set_error(&mut self, message: &str) {
        self.message = message.to_owned();
        self.message_kind = MessageKind::Error;
    }
}

impl eframe::App for CurrencyConverter {
    /// Draws the interface and reacts to the actions it reports.
    fn ui(&mut self, ctx_ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let action = ui::draw(
            ctx_ui,
            self.exchange_rates.as_ref(),
            &mut self.amount,
            &mut self.from,
            &mut self.to,
            &self.message,
            self.message_kind,
        );

        match action {
            Action::None => {}
            Action::Convert => self.convert(),
            Action::Close => ctx_ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close),
        }
    }
}
