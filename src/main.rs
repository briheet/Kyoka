use std::env;

use dotenv::dotenv;
use eframe::egui;

fn main() {
    // Load env for data n ol
    dotenv().ok();

    // Setup logging
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }

    pretty_env_logger::init();

    log::info!("started the porject");

    // Setup gui
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Kyoka",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
    .ok();

    let name = env::var("NAME");

    match name {
        Ok(val) => println!("name: {}", val),
        Err(e) => println!("err: {}", e),
    }

    log::info!("exiting...");
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.label("Top Panel");
        });

        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.label("Left panel");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("gogo gaga");
        });
    }
}
