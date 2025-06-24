mod exchanges;

use dotenv::dotenv;
use eframe::egui;
use std::env;

const LEFT_PANEL_WIDTH: f32 = 40.0;
const LEFT_PANEL_RESIZEABLE: bool = true;

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

    const NET_TYPE: &str = "IS_TESTNET";
    let is_testnet = match env::var(NET_TYPE) {
        Ok(val) => val.to_lowercase() == "true",
        Err(e) => {
            log::error!("Can't determine if testnet or not: {}", e);
            false
        }
    };

    // pool_market_data();

    log::info!("Is testnet: {}", is_testnet);
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
            ui.label("Kyoka");
        });

        egui::SidePanel::left("my_left_panel")
            .resizable(LEFT_PANEL_RESIZEABLE)
            .default_width(LEFT_PANEL_WIDTH)
            .show(ctx, |ui| {
                ui.label("Left panel");
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("gogo gaga");
        });
    }
}
