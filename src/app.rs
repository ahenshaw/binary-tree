/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BinaryTreeApp {
    num_vars: usize,
}

impl Default for BinaryTreeApp {
    fn default() -> Self {
        Self { num_vars: 4 }
    }
}

const MAX_LEVEL: usize = 10;

impl BinaryTreeApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for BinaryTreeApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.0);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add(
                    egui::widgets::Slider::new(&mut self.num_vars, 1..=MAX_LEVEL).prefix("Vars:"),
                );

                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.tree_view(ui);
        });
    }
}
impl BinaryTreeApp {
    fn tree_view(&self, ui: &mut egui::Ui) {
        let r = ui.available_rect_before_wrap();
        let y_spacing = r.height() / self.num_vars as f32;
        let painter = ui.painter();
        let pen = egui::Stroke::new(1.0, egui::Color32::RED);
        let max_nodes = 2usize.pow(self.num_vars as u32);
        let min_x_spacing = r.width() / max_nodes as f32;

        let radius = 0.8 * y_spacing.min(min_x_spacing);

        for var in 0..self.num_vars {
            let y = ((var as f32) + 0.5) * y_spacing;
            let num_nodes = 2usize.pow(var as u32);
            let x_spacing = r.width() / num_nodes as f32;
            for node in 0..num_nodes {
                painter.circle(
                    egui::Pos2 {
                        x: x_spacing * (node as f32 + 0.5),
                        y,
                    },
                    radius,
                    egui::Color32::BLACK,
                    pen,
                );
            }
        }
    }
}
