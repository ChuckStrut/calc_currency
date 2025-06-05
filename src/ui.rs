use crate::currency;
use std::collections::HashMap;

pub struct CalculatorApp {
    pub rates: HashMap<String, f64>,
    pub amount: String,
    pub from_currency: String,
    pub to_currency: String,
    pub result: String,
}

impl CalculatorApp {
    pub async fn new() -> Self {
        let data = currency::fetch_latest_rates().await.unwrap_or_else(|_| {
            eprintln!("Failed to fetch exchange rates");
            currency::ExchangeResponse {
                base: "EUR".to_string(),
                date: "".to_string(),
                rates: HashMap::new(),
            }
        });

        Self {
            rates: data.rates,
            amount: String::new(),
            from_currency: "USD".to_string(),
            to_currency: "ZAR".to_string(),
            result: "".to_string(),
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Multi-Calculator");

            ui.horizontal(|ui| {
                ui.label("From:");
                ui.text_edit_singleline(&mut self.from_currency);
                ui.label("To:");
                ui.text_edit_singleline(&mut self.to_currency);
            });

            ui.horizontal(|ui| {
                ui.label("Amount:");
                ui.text_edit_singleline(&mut self.amount);
                if ui.button("Convert").clicked() {
                    let amt = self.amount.parse::<f64>().unwrap_or(0.0);
                    if let (Some(from), Some(to)) = (
                        self.rates.get(&self.from_currency),
                        self.rates.get(&self.to_currency),
                    ) {
                        self.result = format!("{:.2}", (to / from) * amt);
                    } else {
                        self.result = "Conversion Failed".to_string();
                    }
                }
            });

            ui.label(format!(
                "{} {} = {} {}",
                self.amount, self.from_currency, self.result, self.to_currency
            ));
        });
    }
}
