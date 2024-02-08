#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use ccntool_gui::EguiSandbox;
use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        follow_system_theme: true,
        //initial_window_size: Some(egui::vec2(300.0, 210.0)),
        viewport: egui::ViewportBuilder::default()
            .with_title("TDQU")
            .with_app_id("TDQU")
            .with_inner_size([300.0, 210.0])
            .with_max_inner_size([640.0, 480.0])
            .with_decorations(false)
            .with_icon(
                eframe::icon_data::from_png_bytes(include_bytes!("../../assets/HSDCIT.png"))
                    .unwrap(),
            )
            .with_drag_and_drop(true),
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
