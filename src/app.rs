use balas::{Balas, NodeState};
use egui::{Align2, Color32, FontFamily, FontId, Pos2, RichText, Stroke};
use serde_json;
use std::collections::HashMap;
use std::path::Path;

type State = HashMap<String, balas::NodeState>;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BinaryTreeApp {
    num_vars: usize,
    obj: Vec<f64>,
    obj_label: Option<String>,
    balas: Option<Balas<f64>>,
    step: usize,
    states: Vec<State>,
}

impl Default for BinaryTreeApp {
    fn default() -> Self {
        Self {
            num_vars: 1,
            obj: vec![],
            obj_label: None,
            balas: None,
            step: 0,
            states: vec![],
        }
    }
}

const MAX_VARS: usize = 9;

impl BinaryTreeApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        // Load previous app state (if any).
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }
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
                // egui::widgets::global_dark_light_mode_switch(ui);
                ui.menu_button("File", |ui| {
                    if ui.button("Open file…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.load_recording(&path);
                        }
                    }

                    if ui.button("Close").clicked() {
                        self.balas = None;
                        self.num_vars = 1;
                        self.obj = vec![];
                        self.obj_label = None;
                        self.states = vec![];
                    }

                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                match &self.balas {
                    Some(balas) => {
                        ui.label("  playback:");
                        ui.add(egui::widgets::Slider::new(
                            &mut self.step,
                            0..=balas.recording.len() - 1,
                        ));
                    }
                    None => {
                        ui.label("  # vars:");
                        ui.add(egui::widgets::Slider::new(&mut self.num_vars, 1..=MAX_VARS));
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.tree_view(ui);
        });
    }
}

impl BinaryTreeApp {
    fn load_recording(&mut self, path: &Path) {
        if let Ok(serialized) = std::fs::read_to_string(path) {
            if let Ok(balas) = serde_json::from_str::<Balas<f64>>(&serialized) {
                self.num_vars = balas.coefficients.len();
                self.obj = balas.coefficients.clone();
                let label = self
                    .obj
                    .iter()
                    .enumerate()
                    .map(|(i, c)| {
                        format!(
                            "{c}x{}",
                            char::from_u32('\u{2080}' as u32 + (i + 1) as u32).unwrap()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(" + ");

                self.obj_label = Some(label);
                self.step = 0;
                self.states = vec![];
                let mut state = State::new();
                for record in &balas.recording {
                    state.insert(record.node.clone(), record.state.clone());
                    match record.state {
                        NodeState::ImpossibleChildren => {
                            println!("{}", record.node);
                            for xtra in crate::Binary::new(self.num_vars - record.node.len()) {
                                state.insert(record.node.clone() + &xtra, NodeState::Skipped);
                            }
                        }
                        _ => (),
                    }
                    self.states.push(state.clone());
                }
                self.balas = Some(balas);
            }
        }
    }

    fn tree_view(&mut self, ui: &mut egui::Ui) {
        if let Some(objective) = &self.obj_label {
            ui.label(RichText::new(format!("z = {objective}")).size(20.0));
        }
        let r = ui.available_rect_before_wrap();
        let y_spacing = r.height() / (self.num_vars + 1) as f32;
        let painter = ui.painter_at(r);
        let edge_pen = egui::Stroke::new(1.0, egui::Color32::BLACK);
        // let max_nodes = 2usize.pow(self.num_vars as u32);
        // let min_x_spacing = r.width() / max_nodes as f32;

        let mut nodes = HashMap::<(usize, usize), egui::Pos2>::new();
        let font_id = FontId {
            size: 24.0,
            family: FontFamily::Proportional,
        };
        let states = match self.states.get(self.step) {
            Some(states) => states.clone(),
            None => State::new(),
        };

        for var in 0..self.num_vars + 1 {
            let y = ((var as f32) + 0.5) * y_spacing + r.min.y;
            let num_nodes = 2usize.pow(var as u32);
            let x_spacing = r.width() / num_nodes as f32;
            for node in 0..num_nodes {
                let pt = egui::Pos2 {
                    x: x_spacing * (node as f32 + 0.5) + r.min.x,
                    y,
                };
                nodes.insert((var, node), pt);
            }
            if var > 0 {
                let c = char::from_u32('\u{2080}' as u32 + var as u32).unwrap();
                painter.text(
                    Pos2 {
                        x: r.width() / 2.0 + 10.0,
                        y: y - y_spacing / 2.0,
                    },
                    Align2::CENTER_CENTER,
                    format!("x{c}"),
                    font_id.clone(),
                    Color32::BLACK,
                );
            }
        }
        let mut ordered: Vec<(&(usize, usize), &egui::Pos2)> = nodes.iter().collect();
        ordered.sort_by_key(|((a, b), _)| (a, b));
        for ((var, node), pt) in ordered {
            let radius = y_spacing.min(r.width() / (2usize.pow(*var as u32) as f32)) / 3.0;
            let font_id = FontId {
                size: radius / 2.5,
                family: FontFamily::Proportional,
            };
            let base = match *var {
                0 => String::new(),
                _ => {
                    format!("{node:0width$b}", width = var)
                }
            };

            let (fill, pen) = match states.get(&base) {
                Some(NodeState::Active) => (Color32::LIGHT_YELLOW, Stroke::new(4.0, Color32::GOLD)),
                Some(NodeState::Visited) => (Color32::LIGHT_BLUE, Stroke::new(1.0, Color32::BLACK)),
                Some(NodeState::Infeasible) => (Color32::RED, Stroke::new(1.0, Color32::GRAY)),
                Some(NodeState::Fathomed) => (Color32::GREEN, Stroke::new(1.0, Color32::GRAY)),
                Some(NodeState::ImpossibleChildren) => {
                    (Color32::LIGHT_GRAY, Stroke::new(1.0, Color32::GRAY))
                }
                Some(NodeState::Skipped) => (Color32::DARK_GRAY, Stroke::new(1.0, Color32::GRAY)),
                _ => (Color32::LIGHT_YELLOW, Stroke::new(1.0, Color32::BLACK)),
            };
            let child0 = (var + 1, node * 2);
            let child1 = (var + 1, node * 2 + 1);
            if let Some(child_pt) = nodes.get(&child0) {
                painter.line_segment([*pt, *child_pt], edge_pen);
            }
            if let Some(child_pt) = nodes.get(&child1) {
                painter.line_segment([*pt, *child_pt], edge_pen);
            }
            // radius for all nodes at this level
            painter.circle(*pt, radius, fill, pen);

            let text = format!("{base:×<width$}", width = self.num_vars);
            painter.text(*pt, Align2::CENTER_CENTER, text, font_id, Color32::BLACK);
        }
    }
}
