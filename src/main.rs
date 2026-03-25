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
            
            // // let {x,y} = save_ctx_response;
            ui.heading(format!("{:?}",save_ctx_response.unwrap_or_default().x));
            // if let Some(pos) = ctx.pointer_hover_pos() {
            //     ui.heading(format!("Mouse: ({:.1}, {:.1})", pos.x, pos.y));
            // } else {
            //     ui.heading("Mouse: outside window");
            // }
            let pointer_pos = ctx.pointer_interact_pos();
            // let a = egui::Painter::new(ctx.clone(),ctx.top_layer_id().expect("blah"),ctx.content_rect());
            
            let painter = ui.painter();
            let pointer_state = ui.input(|i| i.pointer.any_down());
            // self.circle_pos.x = pointer_pos.unwrap_or_default().x;
            // self.circle_pos.y = pointer_pos.unwrap_or_default().y;
            // painter.circle(self.circle_pos, 10.0, egui::Color32::RED, egui::Stroke::new(2.0,Color32::RED));
            if pointer_state {
                if let Some(pos) = pointer_pos {
                    self.circle_pos.push(pos);
                }
            }
            for pos in &self.circle_pos {
                painter.circle(*pos, 15.0, egui::Color32::RED, egui::Stroke::new(2.0,Color32::RED));
            }

            // else {
            //     painter.circle(egui::Pos2 { x: 100.0, y: 100.0 }, 10.0, egui::Color32::RED, egui::Stroke::new(2.0,Color32::RED));
            // };
            // painter.circle(egui::Pos2 { x: pointer_pos.unwrap_or_default().x, y: pointer_pos.unwrap_or_default().y }, 10.0, egui::Color32::RED, egui::Stroke::new(2.0,Color32::RED))

        });
    }
}