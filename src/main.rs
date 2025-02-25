use eframe::egui;
use egui_extras::{Column, TableBuilder};

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            TableBuilder::new(ui)
                .columns(Column::auto(), 3) // Create a 3-column table
                .header(20.0, |mut header| {
                    header.col(|ui| { ui.label("Something"); });
                    header.col(|ui| { ui.label("Name"); });
                    header.col(|ui| { ui.label("Age"); });
                })
                .body(|mut body| {
                    for i in 0..5 {
                        body.row(18.0, |mut row| {
                            row.col(|ui| { ui.label(format!("{}", i)); });
                            row.col(|ui| { ui.label(format!("Person {}", i)); });
                            row.col(|ui| { ui.label(format!("{}", 20 + i)); });
                        });
                    }
                });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Egui Extras Example", options, Box::new(|_cc| Box::new(MyApp)))
}
