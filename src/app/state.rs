//! Central application state for the modular Crystal Painter.
//! This file is the app root for the `crate::app` module.

use eframe::egui::{self, Color32, Pos2};
use crate::app::painter::CanvasPainter;
use crate::app::brushes::{BrushKind, crystal, drip, blotter};
use crate::app::ui;

use std::time::Instant;

pub struct AppState {
    // UI / colors
    pub current_color: Color32,
    pub canvas_bg: Color32,
    pub swatches: Vec<Color32>,
    pub selected_swatch: Option<usize>,

    // brush selection
    pub active_brush: BrushKind,

    // brushes (engines + props)
    pub crystal: crystal::CrystalBrush,
    pub drip: drip::DripBrush,
    pub blotter: blotter::BlotterBrush,

    // canvas stored strokes / blots
    pub strokes: Vec<crystal::StrokeData>,
    pub blots: Vec<blotter::Blot>,

    // interactive state
    pub current_points: Vec<Pos2>,
    pub last_tick: Instant,

    // simulation
    pub paused: bool,
    pub contain_growth: bool,
    pub growth_speed: f32,
    pub auto_grow: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_color: Color32::from_rgb(200, 230, 255),
            canvas_bg: Color32::from_rgb(36, 36, 38),
            swatches: vec![
                Color32::from_rgb(200, 230, 255),
                Color32::from_rgb(120, 200, 240),
                Color32::from_rgb(255, 160, 140),
            ],
            selected_swatch: None,

            active_brush: BrushKind::Crystal,

            crystal: crystal::CrystalBrush::new(),
            drip: drip::DripBrush::new(),
            blotter: blotter::BlotterBrush::new(),

            strokes: Vec::new(),
            blots: Vec::new(),

            current_points: Vec::new(),
            last_tick: Instant::now(),

            paused: false,
            contain_growth: false,
            growth_speed: 0.35,
            auto_grow: false,
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ---- Top Bar (UI)
        ui::top_bar::show(self, ctx);

        // ---- Canvas panel
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(self.canvas_bg))
            .show(ctx, |ui| {
                let rect = ui.available_rect_before_wrap();

                // Allocate interaction region BEFORE taking painter()
                let (_, response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

                // Pointer events
                let pointer_pos = response.interact_pointer_pos();
                let down = response.dragged();        // true only when actively dragging
                let released = response.drag_stopped(); // proper release detection

                // ---- Live preview drawing (temporary lines)
                if down {
                    if let Some(p) = pointer_pos {
                        if self.current_points.is_empty() {
                            self.current_points.push(p);
                        } else {
                            let prev = *self.current_points.last().unwrap();
                            let moved = (p - prev).length();

                            if moved > 1.0 {
                                self.current_points.push(p);
                            }
                        }
                    }
                }

                // ---- Finalize stroke
                if released {
                    if self.current_points.len() >= 2 {
                        let mut data = crystal::StrokeData::new(self.current_color, self.auto_grow);

                        for w in self.current_points.windows(2) {
                            let a = w[0];
                            let b = w[1];
                            let dv = b - a;
                            let dir = if dv.length_sq() > 1e-6 {
                                dv.normalized()
                            } else {
                                egui::Vec2::new(1.0, 0.0)
                            };
                            data.add_segment(a, b, dir);
                        }

                        self.strokes.push(data);
                    }
                    self.current_points.clear();
                }

                // ---- Take painter AFTER ui usage ends (fixes borrow error)
                let painter = ui.painter();

                // ---- Painting pipeline
                CanvasPainter::paint_background(&painter, rect, self.canvas_bg);
                CanvasPainter::paint_strokes(&painter, &self.strokes, 2.0);
                CanvasPainter::paint_blots(&painter, &self.blots);
                CanvasPainter::paint_active_path(&painter, &self.current_points);
                CanvasPainter::paint_overlay(&painter, rect, &self.strokes, &self.blots);

                // ---- Simulation tick (crystal growth)
                if !self.paused {
                    self.crystal
                        .growth_step(&mut self.strokes, self.growth_speed, self.contain_growth);
                }
            });

        ctx.request_repaint();
    }
}
