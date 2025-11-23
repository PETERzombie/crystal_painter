//! Central application state for the modular Crystal Painter.

use eframe::egui::{self, Color32, Pos2};
use crate::app::painter::CanvasPainter;
use crate::app::brushes::{BrushKind, crystal, drip, blotter};
use crate::app::ui;

use std::time::Instant;

// Imported from blotter module
use crate::app::brushes::blotter::{Blot, BlotShape};

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
    pub blotter: blotter::BlotterBrush,

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

    // blotter UI settings
    pub halo_offset_percent: f32,
    pub deposit_rate_ms: f32,
    pub blot_shape: BlotShape,
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
            blotter: blotter::BlotterBrush::new(),

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

            halo_offset_percent: 0.0,
            deposit_rate_ms: 40.0,
            blot_shape: BlotShape::Circle,
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        if self.should_destroy {
            self.destroy_canvas();
            self.should_destroy = false;
        }

        if self.should_exit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // Draw UI
        ui::top_bar::show(self, ctx);

        // Canvas panel
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(self.canvas_bg))
            .show(ctx, |ui| {
                let rect = ui.available_rect_before_wrap();
                let (_, response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

                let pointer_pos = response.interact_pointer_pos();
                let down = response.dragged();
                let released = response.drag_stopped();

                if down {
                    if let Some(p) = pointer_pos {
                        match self.active_brush {
                            BrushKind::Crystal | BrushKind::Drip => {
                                if self.current_points.last() != Some(&p) {
                                    self.current_points.push(p);
                                }
                            }

                            BrushKind::Blotter => {
                                self.blots.push(Blot {
                                    pos: p,
                                    radius: self.blotter.props.radius,
                                    halo_radius: self.blotter.props.halo_radius,
                                    shape: self.blot_shape,
                                    color: self.current_color,
                                    softness: self.blotter.props.softness,
                                    opacity: self.blotter.props.opacity,
                                });
                            }
                        }
                    }
                }

                if released {
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
                        BrushKind::Drip => {}
                        BrushKind::Blotter => {}
                    }

                    self.current_points.clear();
                }

                let painter = ui.painter();

                CanvasPainter::paint_background(&painter, rect, self.canvas_bg);
                CanvasPainter::paint_strokes(&painter, &self.strokes, 2.0);
                CanvasPainter::paint_blots(&painter, &self.blots);
                CanvasPainter::paint_active_path(&painter, &self.current_points);
                CanvasPainter::paint_overlay(&painter, rect, &self.strokes, &self.blots);

                if !self.paused {
                    self.crystal
                        .growth_step(&mut self.strokes, self.growth_speed, self.contain_growth);
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
