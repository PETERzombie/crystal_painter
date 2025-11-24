// app/state.rs
//! Central application state for the modular Crystal Painter.

use eframe::egui::{self, Color32, Pos2};
use crate::app::painter::CanvasPainter;
use crate::app::brushes::{BrushKind, crystal, drip, blotter};
use crate::app::brushes::blotter_props::BlotterProps;
use crate::app::ui;

use std::time::Instant;

use crate::app::brushes::blotter::Blot;

pub struct AppState {
    // UI
    pub current_color: Color32,
    pub canvas_bg: Color32,
    pub swatches: Vec<Color32>,
    pub selected_swatch: Option<usize>,

    // brush selection
    pub active_brush: BrushKind,

    // brushes
    pub crystal: crystal::CrystalBrush,
    pub drip: drip::DripBrush,

    // new blotter engine + props
    pub blotter: blotter::Blotter,
    pub blotter_props: BlotterProps,

    // canvas stored elements
    pub strokes: Vec<crystal::StrokeData>,
    pub blots: Vec<Blot>,

    // pointer path tracking
    pub current_points: Vec<Pos2>,
    pub last_tick: Instant,

    // simulation controls
    pub paused: bool,
    pub contain_growth: bool,
    pub growth_speed: f32,
    pub auto_grow: bool,

    // control panel
    pub should_destroy: bool,
    pub should_exit: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_color: Color32::from_rgb(255, 255, 255),
            canvas_bg: Color32::from_rgb(59, 47, 47),

            swatches: vec![
                Color32::from_rgb(120, 200, 240),
                Color32::from_rgb(255, 160, 140),
            ],
            selected_swatch: None,

            active_brush: BrushKind::Crystal,

            crystal: crystal::CrystalBrush::new(),
            drip: drip::DripBrush::new(),

            blotter: blotter::Blotter::new(),
            blotter_props: BlotterProps::default(),

            strokes: Vec::new(),
            blots: Vec::new(),

            current_points: Vec::new(),
            last_tick: Instant::now(),

            paused: true,
            contain_growth: false,
            growth_speed: 0.35,
            auto_grow: false,

            should_destroy: false,
            should_exit: false,
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Handle destroy request
        if self.should_destroy {
            self.destroy_canvas();
            self.should_destroy = false;
        }

        // Handle exit request
        if self.should_exit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // --- UI ---
        ui::top_bar::show(self, ctx);

        // --- Canvas Panel ---
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(self.canvas_bg))
            .show(ctx, |ui| {
                let rect = ui.available_rect_before_wrap();
                let (_, response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

                let pointer_pos = response.interact_pointer_pos();
                let is_down = response.dragged();
                let is_released = response.drag_stopped();

                // Start stroke for blotter
                if let BrushKind::Blotter = self.active_brush {
                    if response.drag_started() {
                        if let Some(pos) = pointer_pos {
                            self.blotter.begin_stroke(pos);
                        }
                    }
                }

                // ---------- DRAWING ----------
                if is_down {
                    if let Some(pos) = pointer_pos {
                        match self.active_brush {
                            BrushKind::Crystal | BrushKind::Drip => {
                                if self.current_points.last() != Some(&pos) {
                                    self.current_points.push(pos);
                                }
                            }

                            BrushKind::Blotter => {
                                let new_blots = self.blotter.tick(
                                    pos,
                                    &self.blotter_props,
                                    self.current_color,
                                );
                                self.blots.extend(new_blots);
                            }
                        }
                    }
                }

                // ---------- FINISH STROKE ----------
                if is_released {
                    match self.active_brush {
                        BrushKind::Crystal => {
                            if self.current_points.len() >= 2 {
                                let mut data =
                                    crystal::StrokeData::new(self.current_color, self.auto_grow);

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
                        }

                        BrushKind::Drip => {
                            // Drip draws live; no commit needed
                        }

                        BrushKind::Blotter => {
                            self.blotter.end_stroke();
                        }
                    }

                    self.current_points.clear();
                }

                // ---------- PAINT ----------
                let painter = ui.painter();

                CanvasPainter::paint_background(&painter, rect, self.canvas_bg);
                CanvasPainter::paint_strokes(&painter, &self.strokes, 2.0);
                CanvasPainter::paint_blots(&painter, &self.blots);
                CanvasPainter::paint_active_path(&painter, &self.current_points);
                CanvasPainter::paint_overlay(&painter, rect, &self.strokes, &self.blots);

                // ---------- AUTO GROW ----------
                if !self.paused {
                    let now = Instant::now();
                    if now.duration_since(self.last_tick).as_millis() > 16 {
                        self.crystal.growth_step(
                            &mut self.strokes,
                            self.growth_speed,
                            self.contain_growth,
                        );
                        self.last_tick = now;
                    }
                }
            });

        ctx.request_repaint();
    }
}

impl AppState {
    pub fn destroy_canvas(&mut self) {
        self.strokes.clear();
        self.blots.clear();
        self.current_points.clear();
    }

    pub fn exit_request(&mut self) {
        self.should_exit = true;
    }
}
