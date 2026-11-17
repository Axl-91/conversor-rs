//! User-interface drawing and interaction.
//!
//! Every function in this module only *draws* widgets and reports what the
//! user did via [`Action`]. It never modifies the application state directly,
//! which keeps the UI separate from the business logic in [`crate::app`].

use eframe::egui::{self, Color32, RichText};

use crate::{api::ExchangeRates, currency::Currency, theme};

/// The visual style of the message shown in the result area.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MessageKind {
    /// An informational hint shown before the first conversion.
    Info,
    /// A successful conversion result.
    Success,
    /// An error, such as invalid input or unavailable rates.
    Error,
}

/// Something the user requested by interacting with the interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Action {
    /// Nothing happened.
    None,
    /// The user pressed Convert (or Enter in the amount field).
    Convert,
    /// The user pressed the close (✖) button.
    Close,
}

/// Draws the whole interface and returns the requested [`Action`].
pub(crate) fn draw(
    ui: &mut egui::Ui,
    rates: Option<&ExchangeRates>,
    amount: &mut String,
    from: &mut Currency,
    to: &mut Currency,
    message: &str,
    message_kind: MessageKind,
) -> Action {
    let mut action = Action::None;

    egui::CentralPanel::default().show(ui, |ui| {
        if draw_header(ui) == Action::Close {
            action = Action::Close;
        }
        ui.add_space(10.0);

        draw_rates_card(ui, rates);
        ui.add_space(14.0);

        draw_currency_selectors(ui, from, to);
        ui.add_space(12.0);

        ui.label(
            RichText::new("💵 Amount")
                .color(theme::ACCENT_LIGHT)
                .strong(),
        );
        let amount_field = ui.add(
            egui::TextEdit::singleline(amount)
                .desired_width(f32::INFINITY)
                .hint_text(
                    RichText::new("e.g. 1000").font(egui::TextStyle::Heading.resolve(ui.style())),
                )
                .font(egui::TextStyle::Heading)
                .vertical_align(egui::Align::Center),
        );
        // Pressing Enter in the amount field is a handy shortcut for Convert.
        if amount_field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter)) {
            action = Action::Convert;
        }

        ui.add_space(12.0);

        ui.vertical_centered(|ui| {
            let convert_button = egui::Button::new(
                RichText::new("Convert")
                    .size(16.0)
                    .strong()
                    .color(theme::ON_ACCENT),
            )
            .fill(theme::ACCENT)
            .corner_radius(egui::CornerRadius::same(8))
            .min_size(egui::vec2(140.0, 36.0));
            if ui.add(convert_button).clicked() {
                action = Action::Convert;
            }
        });

        ui.add_space(14.0);
        draw_result(ui, message, message_kind);
    });

    action
}

/// Draws the colored header with the title and the close (✖) button.
fn draw_header(ui: &mut egui::Ui) -> Action {
    let mut action = Action::None;

    egui::Frame::new()
        .fill(theme::ACCENT)
        .corner_radius(egui::CornerRadius::same(10))
        .inner_margin(egui::Margin::symmetric(14, 10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("💱 Currency Converter")
                        .size(20.0)
                        .strong()
                        .color(theme::ON_ACCENT),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let close_button =
                        egui::Button::new(RichText::new("✖").size(15.0).color(theme::ON_ACCENT))
                            .fill(theme::ACCENT_DARK)
                            .corner_radius(egui::CornerRadius::same(8));
                    if ui
                        .add(close_button)
                        .on_hover_text("Close application")
                        .clicked()
                    {
                        action = Action::Close;
                    }
                });
            });
        });

    action
}

/// Draws the tinted card showing the rate date and the current rates.
fn draw_rates_card(ui: &mut egui::Ui, rates: Option<&ExchangeRates>) {
    let Some(rates) = rates else {
        ui.colored_label(
            theme::ERROR,
            "⚠ Exchange rates unavailable — check your connection and restart the app.",
        );
        return;
    };

    egui::Frame::new()
        .fill(theme::tinted(theme::ACCENT, 22))
        .stroke(egui::Stroke::new(1.0, theme::tinted(theme::ACCENT, 90)))
        .corner_radius(egui::CornerRadius::same(10))
        .inner_margin(egui::Margin::symmetric(14, 10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("📈 Live exchange rates")
                        .color(theme::ACCENT)
                        .strong(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("as of {}", rates.date()))
                            .size(11.0)
                            .color(theme::INFO),
                    );
                });
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(format!("1 USD = {:.2} ARS", rates.rate(Currency::Ars)));
                ui.separator();
                ui.label(format!("1 USD = {:.4} EUR", rates.rate(Currency::Eur)));
            });
        });
}

/// Draws the "From" and "To" selectors plus a swap button.
///
/// The swap button flips the selected currencies, making it quick to convert
/// in the opposite direction. Its icon is drawn with shapes instead of a font
/// glyph so it never renders as a missing-character box.
fn draw_currency_selectors(ui: &mut egui::Ui, from: &mut Currency, to: &mut Currency) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("From").color(theme::ACCENT_LIGHT).strong());
        draw_currency_combo(ui, "from", from);

        // Match the swap button height to the combo-box button height so all
        // widgets in this row share the same baseline.
        let swap_height = ui.spacing().interact_size.y;
        let swap_button = ui.add_sized(
            egui::vec2(38.0, swap_height),
            egui::Button::new("")
                .fill(theme::tinted(theme::ACCENT, 30))
                .corner_radius(egui::CornerRadius::same(8)),
        );
        draw_swap_arrows(ui, swap_button.rect, theme::ACCENT);
        if swap_button.on_hover_text("Swap currencies").clicked() {
            std::mem::swap(from, to);
        }

        ui.label(RichText::new("To").color(theme::ACCENT_LIGHT).strong());
        draw_currency_combo(ui, "to", to);
    });
}

/// Paints two opposed arrows inside `rect` to represent "swap".
fn draw_swap_arrows(ui: &egui::Ui, rect: egui::Rect, color: Color32) {
    let center = rect.center();
    let half_length = rect.width() * 0.28;
    let row_offset = 4.5;
    let stroke = egui::Stroke::new(2.0, color);

    ui.painter().arrow(
        egui::pos2(center.x - half_length, center.y - row_offset),
        egui::vec2(half_length * 2.0, 0.0),
        stroke,
    );
    ui.painter().arrow(
        egui::pos2(center.x + half_length, center.y + row_offset),
        egui::vec2(-half_length * 2.0, 0.0),
        stroke,
    );
}

/// Draws a single currency combo box identified by `id_salt`.
fn draw_currency_combo(ui: &mut egui::Ui, id_salt: &str, selected: &mut Currency) {
    egui::ComboBox::from_id_salt(id_salt)
        .width(150.0)
        .selected_text(format!("{} — {}", selected.code(), selected.name()))
        .show_ui(ui, |ui| {
            for currency in Currency::ALL {
                ui.selectable_value(
                    selected,
                    currency,
                    format!("{} — {}", currency.code(), currency.name()),
                );
            }
        });
}

/// Draws the result card, tinted according to the kind of message.
fn draw_result(ui: &mut egui::Ui, message: &str, kind: MessageKind) {
    let (color, fill) = match kind {
        MessageKind::Info => (theme::INFO, theme::tinted(theme::INFO, 18)),
        MessageKind::Success => (theme::SUCCESS, theme::tinted(theme::SUCCESS, 22)),
        MessageKind::Error => (theme::ERROR, theme::tinted(theme::ERROR, 22)),
    };

    let frame = egui::Frame::new()
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, theme::tinted(color, 100)))
        .corner_radius(egui::CornerRadius::same(10));

    // Reserve a full-width rectangle and paint the frame over it, so the card
    // always spans the panel regardless of how long the message is.
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 60.0), egui::Sense::hover());
    ui.painter().add(frame.paint(rect));

    let icon = match kind {
        MessageKind::Info => "💡",
        MessageKind::Success => "✅",
        MessageKind::Error => "⚠️",
    };
    ui.put(
        rect.shrink(14.0),
        egui::Label::new(
            RichText::new(format!("{icon}  {message}"))
                .size(16.0)
                .strong()
                .color(color),
        )
        .wrap(),
    );
}
