#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use ccntool_gui::EguiSandbox;
use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use image::load_from_memory;

    tracing_subscriber::fmt::init();

    let icon = load_from_memory(include_bytes!("../../assets/HSDCIT.png"))
        .expect("Failed to open icon path");
    let (icon_width, icon_height) = icon.clone().into_rgb8().dimensions();

    let native_options = eframe::NativeOptions {
        decorated: false,
        follow_system_theme: true,
        initial_window_size: Some(egui::vec2(300.0, 210.0)),
        min_window_size: Some(egui::vec2(300.0, 210.0)),
        max_window_size: Some(egui::vec2(640.0, 480.0)),
        transparent: true,
        app_id: Some(String::from("TDQU")),
        icon_data: Some(eframe::IconData {
            rgba: icon.into_bytes(),
            width: icon_width,
            height: icon_height,
        }),
        drag_and_drop_support: true,
        ..Default::default()
    };

    eframe::run_native(
        "TDQU",
        native_options,
        Box::new(|cc| Box::new(EguiSandbox::new(cc))),
    )
    .unwrap();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "TDQU",
            web_options,
            Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
