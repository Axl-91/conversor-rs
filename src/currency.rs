//! Currency definitions and the core conversion math.
//!
//! This module has no dependency on the network or the UI, which makes the
//! conversion logic easy to test in isolation (see the tests at the bottom).

use std::fmt;

/// The currencies supported by the converter.
///
/// The enum derives `Copy` / `Eq` so it can be stored and compared cheaply by
/// egui's `selectable_value` widgets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Currency {
    /// Argentine peso.
    Ars,
    /// US dollar.
    Usd,
    /// Euro.
    Eur,
}

impl Currency {
    /// All supported currencies, in the order shown in the selector.
    pub(crate) const ALL: [Self; 3] = [Self::Ars, Self::Usd, Self::Eur];

    /// Returns the ISO 4217 code of the currency.
    pub(crate) fn code(self) -> &'static str {
        match self {
            Self::Ars => "ARS",
            Self::Usd => "USD",
            Self::Eur => "EUR",
        }
    }

    /// Returns a human readable name for the currency.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Ars => "Argentine peso",
            Self::Usd => "US dollar",
            Self::Eur => "Euro",
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

/// Converts an amount between two currencies using rates quoted against USD.
///
/// Both `from_rate` and `to_rate` are expressed as *units of currency per
/// 1 USD*. The amount is first divided by the source rate to obtain USD, then
/// multiplied by the target rate.
///
/// # Examples
///
/// ```text
/// convert(1000.0, /* ARS per USD */ 1547.21, /* EUR per USD */ 0.90729)
/// // ≈ 0.59 EUR
/// ```
pub(crate) fn convert(amount: f64, from_rate: f64, to_rate: f64) -> f64 {
    amount / from_rate * to_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_ars_to_eur_through_usd() {
        let result = convert(1000.0, 1547.21, 0.907_29);
        assert!((result - 0.586_40).abs() < 0.000_01);
    }

    #[test]
    fn converting_to_same_currency_is_identity() {
        let result = convert(250.0, 1547.21, 1547.21);
        assert!((result - 250.0).abs() < 0.000_001);
    }

    #[test]
    fn currency_codes_and_names_match() {
        assert_eq!(Currency::Ars.code(), "ARS");
        assert_eq!(Currency::Usd.name(), "US dollar");
        assert_eq!(Currency::Eur.to_string(), "EUR");
    }
}
