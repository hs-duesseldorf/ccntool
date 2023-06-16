use crate::custom_frame::custom_window_frame;
use ccntool_core::{connectdb, myquery, queryall};

use eframe::egui;
use egui::{FontId, FontTweak, RichText};
use egui_dropdown::DropDownBox;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct EguiSandbox {
    buf: String,
    dcim_url: String,
    error: String,
    my_password: String,
    my_username: String,
    ports: Vec<String>,
    results: Vec<String>,
    settings_toggler: bool,
}

impl EguiSandbox {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "hsd".into(),
            egui::FontData::from_static(include_bytes!("../../assets/HSDSans-Regular.ttf")).tweak(
                FontTweak {
                    scale: 1.0,
                    y_offset_factor: 0.15,
                    y_offset: 0.0,
                    baseline_offset_factor: 0.0,
                },
            ),
        );

        fonts
            .families
            .insert(egui::FontFamily::Name("hsdfont".into()), vec!["hsd".into()]);

        fonts.font_data.insert(
            "emoji".to_owned(),
            egui::FontData::from_static(include_bytes!("../../assets/Bacon.ttf")).tweak(
                FontTweak {
                    scale: 0.90,
                    y_offset_factor: 0.3,
                    y_offset: 0.0,
                    baseline_offset_factor: 1.0,
                },
            ),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .push("emoji".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self {
            buf: String::new(),
            dcim_url: String::new(),
            error: String::new(),
            my_password: String::new(),
            my_username: String::new(),
            ports: vec![],
            results: vec![],
            settings_toggler: false,
        }
    }
}

impl eframe::App for EguiSandbox {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        custom_window_frame(ctx, frame, "TDQU", |ui| {
            ui.add(DropDownBox::from_iter(
                &self.ports,
                "portselector",
                &mut self.buf,
                |ui, text| ui.selectable_label(false, text),
            ));

            egui::Grid::new("Functionbuttons").show(ui, |ui| {
                if ui.button("Query").clicked() {
                    self.results.clear();

                    let un = match &self.my_username.len() {
                        0 => Option::None,
                        _ => Some(self.my_username.clone()),
                    };

                    let pw = match &self.my_password.len() {
                        0 => Option::None,
                        _ => Some(self.my_password.clone()),
                    };

                    let burl = match &self.dcim_url.len() {
                        0 => Option::None,
                        _ => Some(self.dcim_url.clone()),
                    };

                    let conn = match connectdb(un, pw, burl) {
                        Ok(pool) => pool,
                        Err(error) => {
                            let error_message = format!("Connection timeout: {error}");
                            self.error = error_message;
                            eprintln!("Error: {}", error);
                            return;
                        }
                    };

                    // TODO: catch if self.results[6] is = 1
                    // Switchname: 1
                    self.results = match myquery(conn, &self.buf) {
                        Ok(rows) => {
                            self.error.clear();
                            rows
                        }
                        Err(error) => {
                            let error_message = format!("Received garbage: {error}");
                            self.error = error_message;
                            eprintln!("Error: {error}");
                            return;
                        }
                    };
                }

                if ui.button("Clear").clicked() {
                    self.results.clear();
                    self.error.clear();
                    self.buf = String::new();
                }

                if ui.add(egui::widgets::Button::new("⚙")).clicked() {
                    self.settings_toggler = true;
                }
            });

            if self.settings_toggler {
                let settings_window = egui::Window::new("Settings")
                    .collapsible(false)
                    .title_bar(false);
                settings_window.show(ui.ctx(), |ui| {
                    ui.group(|ui| {
                        egui::Grid::new("Settingsgrid").show(ui, |ui| {
                            ui.label("Theme: ");
                            egui::widgets::global_dark_light_mode_buttons(ui);
                            ui.end_row();
                            ui.label("Username:");
                            ui.add(egui::TextEdit::singleline(&mut self.my_username));
                            ui.end_row();
                            ui.label("Password:");
                            ui.add(egui::TextEdit::password(
                                egui::TextEdit::singleline(&mut self.my_password),
                                true,
                            ));
                            ui.end_row();
                            ui.label("DCIM URL:");
                            ui.add(egui::TextEdit::singleline(&mut self.dcim_url));
                            ui.end_row();
                            if ui.button("Close").clicked() {
                                let un = match &self.my_username.len() {
                                    0 => Option::None,
                                    _ => Some(self.my_username.clone()),
                                };

                                let pw = match &self.my_password.len() {
                                    0 => Option::None,
                                    _ => Some(self.my_password.clone()),
                                };

                                let burl = match &self.dcim_url.len() {
                                    0 => Option::None,
                                    _ => Some(self.dcim_url.clone()),
                                };

                                self.ports = queryall(
                                    connectdb(un, pw, burl).expect("Can't connect to database!"),
                                )
                                .unwrap();
                                self.settings_toggler = false;
                            }
                        });
                    });
                });
            }

            if self.results.is_empty() {
                ui.label(
                    RichText::new("Click butt0n, receive    🥓").font(FontId::proportional(14.0)),
                );
            } else {
                let mut text: String = format!(
                    r#"Switchname: {}
IP: {}
Switchport: {}
Description: {}"#,
                    self.results[0], self.results[3], self.results[2], self.results[1],
                );

                ui.add(egui::TextEdit::multiline(&mut text).desired_width(f32::INFINITY));
                // TODO: have this be evaluated via env as well
                let url: String = format!(
                    "https://{}/devices.php?DeviceID={}",
                    self.dcim_url, self.results[4]
                );
                ui.hyperlink_to("View switch on openDCIM", url);
            }

            if !self.error.is_empty() {
                ui.label(
                    RichText::new(&self.error)
                        .font(FontId::proportional(14.0))
                        .color(egui::Color32::RED),
                );
            }
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn on_close_event(&mut self) -> bool {
        self.results.clear();
        self.buf = String::new();
        self.error.clear();
        self.settings_toggler = false;

        true
    }
}
