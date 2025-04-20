use eframe::{egui, CreationContext, App};
use serde::Deserialize;
use std::{fs};

// =======================
// CONFIG STRUCT & LOADING
// =======================

#[derive(Deserialize, Debug)]
struct AppConfig {
    username: String,
    password: String,
    image_path: String,
}

impl AppConfig {
    fn load() -> Self {
        let config_data = fs::read_to_string("resources/config.json")
            .expect("Non riesco a leggere config.json");
        serde_json::from_str(&config_data)
            .expect("Errore nel parsing di config.json")
    }
}

// =============
// MAIN APP LOGIC
// =============

pub struct StoreTournamentTracker {
    config: AppConfig,
    input_username: String,
    input_password: String,
    image_texture: Option<egui::TextureHandle>,
}

impl StoreTournamentTracker {
    fn new(cc: &CreationContext<'_>) -> Self {
        let config = AppConfig::load();

        // Load image from path
        let mut image_texture = None;
        if let Ok(image_bytes) = fs::read(&config.image_path) {
            if let Ok(image) = image::load_from_memory(&image_bytes) {
                let size = [image.width() as usize, image.height() as usize];
                let rgba = image.to_rgba8();
                let pixels = rgba.into_vec();

                let texture = cc.egui_ctx.load_texture(
                    "config_image",
                    egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                    egui::TextureOptions::default(),
                );
                image_texture = Some(texture);
            }
        }

        Self {
            config,
            input_username: String::new(),
            input_password: String::new(),
            image_texture,
        }
    }
}

impl App for StoreTournamentTracker {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.image_texture {
                ui.image(texture);
            }

            ui.add_space(20.0);

            ui.label("Username:");
            ui.text_edit_singleline(&mut self.input_username);

            ui.label("Password:");
            ui.add(egui::TextEdit::singleline(&mut self.input_password).password(true));

            ui.add_space(10.0);

            if ui.button("Login").clicked() {
                if self.input_username == self.config.username
                    && self.input_password == self.config.password
                {
                    println!("✅ Login avvenuto con successo");
                } else {
                    println!("❌ Credenziali errate");
                }
            }
        });
    }
}

// ========
// MAIN 🧨
// ========

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Store Tournament Tracker",
        options,
        Box::new(|cc| Box::new(StoreTournamentTracker::new(cc))),
    )
}
