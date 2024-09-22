use eframe::egui::{Color32, Pos2, Stroke};
use eframe::emath::Rect;
use egui::epaint::PathStroke;
use egui::{Align2, FontId, Shape, Vec2};
use std::iter::StepBy;
use std::ops::RangeInclusive;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Graph",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
}

fn y(x: f32) -> f32 {
    x.powi(2)
}

struct MyEguiApp {
    graph_config: GraphConfig,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            graph_config: GraphConfig {
                screen_rect: cc.egui_ctx.screen_rect(),
                offset: Pos2::new(0.0, 0.0),
            },
        }
    }
}

fn round_to_nearest(a: i64, b: i64) -> i64 {
    let a = a as f64;
    let b = b as f64;
    ((a / b).round() * b) as i64
}

const GRID_SIZE: f32 = 25.0;
const SCALE: f32 = 1.0;
const MAJOR_LINE_COLOR: Color32 = Color32::from_rgb(73, 73, 73);
const MINOR_LINE_COLOR: Color32 = Color32::from_rgb(44, 44, 44);

const NUM_LABEL_OFFSET: f32 = -0.45;

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());
            let rect = response.rect;
            let change = response.drag_motion();
            self.graph_config.offset += Vec2::new(-change.x, change.y) / 8.0;
            self.graph_config.screen_rect = rect;

            let center_pos = GraphPoint { x: 0.0, y: 0.0 }.point_on_screen(self.graph_config);
            let center_x = rect.left() + (rect.width() / 2.0);
            let center_y = rect.top() + (rect.height() / 2.0);

            println!("X: {} Y: {} Center Pos: {}", center_x, center_y, center_pos);

            let graph_width = rect.width() as i32 / GRID_SIZE as i32 + 1;
            let graph_height = rect.height() as i32 / GRID_SIZE as i32 + 1;

            let mut draw_queue: Vec<Shape> = vec![];

            for x in ((-graph_width / 2)..=graph_width / 2).map(|x| x as f32 * GRID_SIZE) {
                let x_pos = center_x + x - (self.graph_config.offset.x.fract() * GRID_SIZE);
                draw_queue.push(Shape::LineSegment {
                    points: [
                        Pos2::new(x_pos, rect.top()),
                        Pos2::new(x_pos, rect.bottom()),
                    ],
                    stroke: PathStroke::from(Stroke::new(1.0, MINOR_LINE_COLOR)),
                });
            }

            for y in ((-graph_height / 2)..=graph_height / 2).map(|x| x as f32 * GRID_SIZE) {
                let y_pos = center_y + y + self.graph_config.offset.y.fract() * GRID_SIZE;
                draw_queue.push(Shape::LineSegment {
                    points: [
                        Pos2::new(rect.left(), y_pos),
                        Pos2::new(rect.right(), y_pos),
                    ],
                    stroke: PathStroke::from(Stroke::new(1.0, MINOR_LINE_COLOR)),
                });
            }

            painter.line_segment(
                [
                    Pos2::new(center_pos.x, rect.top()),
                    Pos2::new(center_pos.x, rect.bottom()),
                ],
                Stroke::new(2.0, MAJOR_LINE_COLOR),
            );
            painter.line_segment(
                [
                    Pos2::new(rect.left(), center_pos.y),
                    Pos2::new(rect.right(), center_pos.y),
                ],
                Stroke::new(2.0, MAJOR_LINE_COLOR),
            );
            let mut prev_line: Option<GraphPoint> = None;
            let mut set_prev_line = false;
            let offset = self.graph_config.offset;
            for i in (-(rect.width() / 2.0 + GRID_SIZE * 2.0) + (offset.x * GRID_SIZE)) as i64
                ..((rect.width() / 2.0 + GRID_SIZE * 2.0) + (offset.x * GRID_SIZE)) as i64
            {
                if !set_prev_line {
                    prev_line = Some(GraphPoint {
                        x: i as f32 / GRID_SIZE,
                        y: y(i as f32 / GRID_SIZE),
                    });
                    set_prev_line = true;
                    continue;
                }
                let p1 = GraphPoint {
                    x: i as f32 / GRID_SIZE,
                    y: y(i as f32 / GRID_SIZE),
                };
                painter.line_segment(
                    [
                        prev_line.unwrap().point_on_screen(self.graph_config),
                        p1.point_on_screen(self.graph_config),
                    ],
                    Stroke::new(2.0, Color32::RED),
                );
                prev_line = Some(p1);
            }
            for i in create_draw_text_iter(graph_width, offset.x) {
                let text_pos = GraphPoint {
                    x: i as f32 - if i == 0 { -NUM_LABEL_OFFSET } else { 0.0 },
                    y: NUM_LABEL_OFFSET,
                }
                .point_on_screen(self.graph_config);
                painter.text(
                    text_pos,
                    Align2::CENTER_CENTER,
                    i.to_string(),
                    FontId::default(),
                    Color32::WHITE,
                );
            }
            for i in create_draw_text_iter(graph_height, offset.y) {
                if i == 0 {
                    continue;
                }
                let text_pos = GraphPoint {
                    x: NUM_LABEL_OFFSET,
                    y: i as f32,
                }
                .point_on_screen(self.graph_config);
                painter.text(
                    text_pos,
                    Align2::CENTER_CENTER,
                    i.to_string(),
                    FontId::default(),
                    Color32::WHITE,
                );
            }
        });
    }
}

fn create_draw_text_iter(axis_length: i32, offset: f32) -> StepBy<RangeInclusive<i64>> {
    (round_to_nearest((-(axis_length / 2 + 1) + offset as i32) as i64, 2)
        ..=round_to_nearest(((axis_length / 2 + 1) + offset as i32) as i64, 2))
        .step_by(2)
}

#[derive(Clone, Copy)]
struct GraphConfig {
    pub screen_rect: Rect,
    pub offset: Pos2,
}

impl GraphConfig {
    pub fn center_pos(&self) -> Pos2 {
        Pos2::new(
            self.screen_rect.left() + (self.screen_rect.width() / 2.0),
            self.screen_rect.top() + (self.screen_rect.height() / 2.0),
        )
    }
}

#[derive(Clone, Copy)]
struct GraphPoint {
    pub x: f32,
    pub y: f32,
}

impl GraphPoint {
    pub fn point_on_screen(&self, graph_config: GraphConfig) -> Pos2 {
        let center = graph_config.center_pos();
        Pos2::new(
            center.x + ((self.x - graph_config.offset.x) * GRID_SIZE),
            center.y + ((-(self.y - graph_config.offset.y)) * GRID_SIZE),
        )
    }
}
