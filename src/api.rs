//! Exchange-rate API client and rate snapshot.
//!
//! This module is the only place that talks to the network. It downloads the
//! rates once at startup and stores them in a form that makes any currency
//! pair convertible without further requests.

use std::collections::HashMap;
use std::error::Error;

use serde::Deserialize;

use crate::currency::{self, Currency};

/// A single exchange rate as returned by the Frankfurter `/v2/rate` endpoint.
///
/// The API answers requests of the form `GET /v2/rate/{base}/{quote}` with a
/// JSON body such as:
///
/// ```json
/// { "date": "2026-11-17", "base": "USD", "quote": "ARS", "rate": 1547.21 }
/// ```
#[derive(Debug, Deserialize)]
struct ExchangeRate {
    /// The date the rate refers to (ISO 8601, `YYYY-MM-DD`).
    date: String,
    /// The base currency code.
    #[allow(dead_code)]
    base: String,
    /// The quoted currency code.
    #[allow(dead_code)]
    quote: String,
    /// How many units of `quote` correspond to one unit of `base`.
    rate: f64,
}

/// Fetches the exchange rate between USD and a single `quote` currency.
///
/// # Errors
///
/// Returns an error if the request fails or the response is malformed.
fn fetch_rate(quote: &str) -> Result<ExchangeRate, Box<dyn Error>> {
    let url = format!("https://api.frankfurter.dev/v2/rate/USD/{quote}");

    let response = reqwest::blocking::get(url)?
        .error_for_status()?
        .json::<ExchangeRate>()?;

    Ok(response)
}

/// A snapshot of all exchange rates needed by the converter.
///
/// Rates are kept as "units of currency per 1 USD". USD is inserted explicitly
/// with a rate of `1.0` so that every pair can be handled by the same formula.
pub(crate) struct ExchangeRates {
    /// The date the rates refer to (taken from the API response).
    date: String,
    /// Mapping of currency code to its value relative to one US dollar.
    rates: HashMap<String, f64>,
}

impl ExchangeRates {
    /// Fetches the current USD → ARS and USD → EUR rates.
    ///
    /// # Errors
    ///
    /// Returns an error if either HTTP request fails, the response status is
    /// not successful, or the JSON body cannot be deserialized.
    pub(crate) fn fetch() -> Result<Self, Box<dyn Error>> {
        let ars = fetch_rate("ARS")?;
        let eur = fetch_rate("EUR")?;

        let rates = HashMap::from([
            (Currency::Usd.code().to_owned(), 1.0),
            (Currency::Ars.code().to_owned(), ars.rate),
            (Currency::Eur.code().to_owned(), eur.rate),
        ]);

        Ok(Self {
            date: ars.date,
            rates,
        })
    }

    /// Returns the date the rates refer to, formatted by the API.
    pub(crate) fn date(&self) -> &str {
        &self.date
    }

    /// Returns the value of one unit of `currency` expressed in USD.
    pub(crate) fn rate(&self, currency: Currency) -> f64 {
        self.rates[currency.code()]
    }

    /// Converts `amount` from one currency to another using this snapshot.
    pub(crate) fn convert(&self, amount: f64, from: Currency, to: Currency) -> f64 {
        currency::convert(amount, self.rate(from), self.rate(to))
    }
}
