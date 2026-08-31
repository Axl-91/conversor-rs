use eframe::egui;

const USD_TO_ARS_RATE: f64 = 1400.0;

struct CurrencyConverter {
    amount: String,
    result: String,
    ars_to_usd: bool,
}

impl Default for CurrencyConverter {
    fn default() -> Self {
        Self {
            amount: String::new(),
            result: String::new(),
            ars_to_usd: true,
        }
    }
}

impl eframe::App for CurrencyConverter {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Currency Converter");

                        ui.label(format!("Base rate: 1 USD = {:.2} ARS", USD_TO_ARS_RATE));

                        ui.add_space(15.0);

                        ui.radio_value(&mut self.ars_to_usd, true, "ARS to USD");
                        ui.radio_value(&mut self.ars_to_usd, false, "USD to ARS");

                        ui.add_space(10.0);

                        let str_amount_type = if self.ars_to_usd { "ARS" } else { "USD" };

                        ui.label(format!("{} amount: ", str_amount_type));

                        ui.text_edit_singleline(&mut self.amount);

                        ui.add_space(10.0);

                        if ui.button("Convert").clicked() {
                            let amount_result = self.amount.trim().replace(',', ".").parse::<f64>();

                            match amount_result {
                                Ok(amount) => {
                                    let converted_amount = if self.ars_to_usd {
                                        amount / USD_TO_ARS_RATE
                                    } else {
                                        amount * USD_TO_ARS_RATE
                                    };

                                    self.result = if self.ars_to_usd {
                                        format!("{:.2} ARS = {:.2} USD", amount, converted_amount)
                                    } else {
                                        format!("{:.2} USD = {:.2} ARS", amount, converted_amount)
                                    };
                                }

                                Err(_) => {
                                    self.result = "Please enter a valid number.".to_string();
                                }
                            }
                        }

                        ui.add_space(10.0);

                        ui.separator();

                        if !self.result.is_empty() {
                            ui.add_space(10.0);
                            ui.heading(&self.result);
                        } else {
                            ui.add_space(10.0);
                            ui.heading("No convertion made");
                        }
                    })
                },
            )
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([360.0, 260.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Currency Converter",
        options,
        Box::new(|_creation_context| Ok(Box::new(CurrencyConverter::default()))),
    )
}
