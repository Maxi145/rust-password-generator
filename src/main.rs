#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use password_generator::ComplexPasswordGenerator;
mod password_generator;

struct AppState {
    password_length: usize,
    include_uppercase: bool,
    include_lowercase: bool,
    include_numbers: bool,
    include_symbols: bool,
    num_passwords: usize,
    generated_passwords: Vec<String>,
    password_generator: Box<ComplexPasswordGenerator>,
    copied_password: Option<String>,
    all_passwords_copied: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            password_length: 12,
            include_uppercase: true,
            include_lowercase: true,
            include_numbers: false,
            include_symbols: false,
            num_passwords: 1,
            generated_passwords: Vec::new(),
            password_generator: Box::new(ComplexPasswordGenerator::new(true, true, false, false)),
            copied_password: None,
            all_passwords_copied: false,
        }
    }

    fn update_generator(&mut self) {
        self.password_generator = Box::new(ComplexPasswordGenerator::new(
            self.include_uppercase,
            self.include_lowercase,
            self.include_numbers,
            self.include_symbols,
        ));
    }

    fn is_generation_enabled(&self) -> bool {
        self.include_uppercase || self.include_lowercase || self.include_numbers || self.include_symbols
    }
}

fn main() -> eframe::Result {
    let icon = include_bytes!("../resources/icon.png");
    let image = image::load_from_memory(icon).expect("Failed to load icon");
    let rgba = image.to_rgba8();
    let (icon_width, icon_height) = rgba.dimensions();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(egui::IconData {
                rgba: rgba.into_raw(),
                width: icon_width,
                height: icon_height,
            })
            .with_inner_size([450.0, 400.0])
            .with_min_inner_size([380.0, 300.0])
            .with_max_inner_size([700.0, 1100.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Password Generator",
        options,
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();

            style.visuals.widgets.noninteractive.fg_stroke.width = 2.0; 
            style.visuals.widgets.inactive.fg_stroke.width = 2.0; 
            style.spacing.icon_width = 20.0; 

            style.text_styles = [
                (egui::TextStyle::Heading, egui::FontId::new(30.0, egui::FontFamily::Proportional)),
                (egui::TextStyle::Body, egui::FontId::new(20.0, egui::FontFamily::Proportional)),
                (egui::TextStyle::Monospace, egui::FontId::new(20.0, egui::FontFamily::Monospace)),
                (egui::TextStyle::Button, egui::FontId::new(20.0, egui::FontFamily::Proportional)),
                (egui::TextStyle::Small, egui::FontId::new(15.0, egui::FontFamily::Proportional)),
            ]
            .into();

            cc.egui_ctx.set_style(style);

            Ok(Box::new(AppState::new()))
        }),
    )
}

const PRIMARY_COLOR: egui::Color32 = egui::Color32::from_rgb(225, 225, 225);

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::CollapsingHeader::new("Settings")
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new("settings_grid")
                    .num_columns(2)
                    .spacing(egui::Vec2::new(10.0, 10.0))
                    .show(ui, |ui| {
                        ui.label("Password Length:");
                        ui.add(egui::Slider::new(&mut self.password_length, 4..=20));
                        ui.end_row();

                        ui.label("Include Uppercase:");
                        ui.add(egui::Checkbox::without_text(&mut self.include_uppercase));
                        ui.end_row();

                        ui.label("Include Lowercase:");
                        ui.add(egui::Checkbox::without_text(&mut self.include_lowercase));
                        ui.end_row();

                        ui.label("Include Numbers:");
                        ui.add(egui::Checkbox::without_text(&mut self.include_numbers));
                        ui.end_row();

                        ui.label("Include Symbols:");
                        ui.add(egui::Checkbox::without_text(&mut self.include_symbols));
                        ui.end_row();

                        ui.label("Number of Passwords:");
                        ui.add(egui::Slider::new(&mut self.num_passwords, 1..=50));
                        ui.end_row();
                    });
            });


            ui.separator();

            ui.horizontal(|ui| {
                egui::Grid::new("button_grid")
                    .num_columns(2)
                    .spacing(egui::Vec2::new(10.0, 10.0))
                    .show(ui, |ui| {
                        let generate_passwords_button = egui::Button::new(
                            egui::RichText::new("🔄 Generate Passwords")
                                .color(egui::Color32::from_rgb(24, 24, 27))
                        )
                        .fill(PRIMARY_COLOR)
                        .corner_radius(6.0);

                        if ui.add_enabled(self.is_generation_enabled(), generate_passwords_button).clicked() {
                            self.all_passwords_copied = false;
                            self.update_generator();
                            self.generated_passwords = (0..self.num_passwords)
                                .map(|_| self.password_generator.generate_password(self.password_length))
                                .collect();
                        }

                        let copy_all_passwords_button = egui::Button::new(
                            egui::RichText::new("📋 Copy All Passwords")
                                .color(egui::Color32::from_rgb(205, 205, 205))
                        )
                        .fill(egui::Color32::from_rgb(39, 39, 42))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(63, 63, 70)))
                        .corner_radius(6.0);

                        if ui.add_enabled(!self.generated_passwords.is_empty(), copy_all_passwords_button).clicked() {
                            let all_passwords = self.generated_passwords.join("\n");
                            ui.ctx().copy_text(all_passwords);
                            self.all_passwords_copied = true;
                        }
            
                    });
            });

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("ℹ Scroll and right-click on a password to copy it")
                        .size(14.0)
                        .color(egui::Color32::from_rgb(150, 150, 150))
                );
            });

            if !self.generated_passwords.is_empty() {
                ui.separator();
                egui::Frame::group(ui.style())
                    .inner_margin(egui::Margin::same(10))
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink(false)
                            .show(ui, |ui| {
                                ui.with_layout(
                                    egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                                    |ui| {
                                        for password in &self.generated_passwords {
                                            ui.horizontal(|ui| {
                                                ui.set_width(ui.available_width());
                                                let response = ui.monospace(password);
                                                
                                                if response.clicked() {
                                                    ui.ctx().copy_text(password.clone());
                                                    self.copied_password = Some(password.clone());
                                                    self.all_passwords_copied = false;
                                                }

                                                if self.all_passwords_copied || Some(password) == self.copied_password.as_ref() {
                                                    ui.label(egui::RichText::new("✅").color(PRIMARY_COLOR));
                                                }
                                            });
                                        }
                                    },
                                );
                            });
                    });
            }
        });
    }
}