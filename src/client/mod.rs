use eframe::NativeOptions;
use egui::ViewportBuilder;

pub struct WindowSettings {
    title: String,
    options: NativeOptions,
}

impl WindowSettings {
    pub fn new() -> Self {
        Self {
            title: "Lab DB".to_string(),
            options: NativeOptions {
                viewport: ViewportBuilder::default(),
                ..Default::default()
            }
        }
    }
}

pub struct App {
    window_settings: WindowSettings
}

impl App {
    pub fn init() -> Self {
        Self {
            window_settings: WindowSettings::new()
        }
    }

    pub fn run(&mut self) -> eframe::Result {
        eframe::run_native(
        &self.window_settings.title,
        self.window_settings.options.clone(),
        Box::new(|_cc| {
            Ok(Box::<App>::default())
        }),
    )
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            window_settings: WindowSettings::new()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello world");
        });
    }
}
