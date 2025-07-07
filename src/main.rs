mod exchanges;
use core::panic;
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
    log::info!("Execution starts!");

    // Check for type, need to push to another function for handling soon
    const NET_TYPE: &str = "IS_TESTNET";

    let _is_testnet = match env::var(NET_TYPE) {
        Ok(val) => val.to_lowercase() == "true",
        Err(e) => {
            log::error!("Can't determine if testnet or not: {}", e);
            false
        }
    };

    // Spawn a thread UwU
    std::thread::spawn(move || {
        // Get username and password
        let username = match env::var("USERNAME") {
            Ok(val) => val,
            Err(e) => {
                log::error!("Can't determine the username: {}", e);
                panic!("Missing USERNAME env variable");
            }
        };

        let password = match env::var("password")

        let login = exchanges::Logon::new(30);
    });

    // Setup gui
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Kyoka",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
    .ok();
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
