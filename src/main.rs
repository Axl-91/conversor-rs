use eframe::egui;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ExchangeRate {
    date: String,
    base: String,
    quote: String,
    rate: f64,
}

fn get_exchange_rate() -> Result<ExchangeRate, Box<dyn std::error::Error>> {
    let url = "https://api.frankfurter.dev/v2/rate/USD/ARS";

    let response = reqwest::blocking::get(url)?
        .error_for_status()?
        .json::<ExchangeRate>()?;

    Ok(response)
}

struct CurrencyConverter {
    exchange_rate: Option<ExchangeRate>,
    amount: String,
    result: String,
    ars_to_usd: bool,
}

impl CurrencyConverter {
    fn new(exchange_rate: Result<ExchangeRate, Box<dyn std::error::Error>>) -> Self {
        match exchange_rate {
            Ok(rate) => Self {
                exchange_rate: Some(rate),
                amount: String::new(),
                result: String::new(),
                ars_to_usd: true,
            },

            Err(error) => Self {
                exchange_rate: None,
                amount: String::new(),
                result: format!("Could not get exchange rate: {error}"),
                ars_to_usd: true,
            },
        }
    }

    fn convert(&mut self) {
        let Some(exchange_rate) = &self.exchange_rate else {
            self.result = "Exchange rate is not available.".to_string();
            return;
        };

        let amount = match self.amount.trim().replace(',', ".").parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                self.result = "Please enter a valid number.".to_string();
                return;
            }
        };

        let converted_amount = if self.ars_to_usd {
            amount / exchange_rate.rate
        } else {
            amount * exchange_rate.rate
        };

        self.result = if self.ars_to_usd {
            format!("{:.2} ARS = {:.2} USD", amount, converted_amount)
        } else {
            format!("{:.2} USD = {:.2} ARS", amount, converted_amount)
        };
    }
}

impl eframe::App for CurrencyConverter {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Currency Converter");

                if let Some(exchange_rate) = &self.exchange_rate {
                    ui.label(format!(
                        "Rate from {}\n1 {} = {:.2} {}",
                        exchange_rate.date,
                        exchange_rate.base,
                        exchange_rate.rate,
                        exchange_rate.quote
                    ));
                } else {
                    ui.colored_label(egui::Color32::RED, "Exchange rate unavailable");
                }

                ui.add_space(15.0);

                ui.radio_value(&mut self.ars_to_usd, true, "ARS to USD");

                ui.radio_value(&mut self.ars_to_usd, false, "USD to ARS");

                ui.add_space(10.0);

                ui.label("Amount:");

                ui.text_edit_singleline(&mut self.amount);

                ui.add_space(10.0);

                if ui.button("Convert").clicked() {
                    self.convert();
                }

                ui.add_space(10.0);

                if !self.result.is_empty() {
                    ui.separator();
                    ui.heading(&self.result);
                }
            })
        });
    }
}

fn main() -> eframe::Result<()> {
    let exchange_rate = get_exchange_rate();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([360.0, 260.0])
            .with_min_inner_size([320.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Currency Converter",
        options,
        Box::new(|_creation_context| Ok(Box::new(CurrencyConverter::new(exchange_rate)))),
    )
}
