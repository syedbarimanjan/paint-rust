use eframe::egui::{self, Color32, Painter};


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "paint app in rust",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

// #[derive(Default)]
struct MyApp {
    circle_pos: Vec<egui::Pos2>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            circle_pos: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("my paint app written in rust the lords lang");
            let save_ctx_response = ctx.pointer_hover_pos();
            ui.heading(format!("{:?}",save_ctx_response.unwrap_or_default().x));
            let pointer_pos = ctx.pointer_interact_pos();
            let painter = ui.painter();
            let pointer_state = ui.input(|i| i.pointer.any_down());
            if pointer_state {
                if let Some(pos) = pointer_pos {
                    self.circle_pos.push(pos);
                }
            }
            for pos in &self.circle_pos {
                painter.circle(*pos, 15.0, egui::Color32::RED, egui::Stroke::new(2.0,Color32::RED));
            }
        });
    }
}