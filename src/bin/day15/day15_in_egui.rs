// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use aocutils::point::Point;
use eframe::egui::{self, Pos2};
use eframe::emath::Vec2;
use std::collections::{HashMap, VecDeque};
use std::fs;

pub fn egui_main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "AoC2024 - Day 15.1",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            // run the main loop
            Ok(Box::new(Warehouse::new(cc)))
        }),
    )
}

// The model, this is the warehouse from the puzzle code
#[derive(Debug, Clone, Default)]
struct Robot {
    pos: Point<usize>,
    moved_successfully: bool,
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
enum Obstacle {
    Wall,
    Box,
}

#[derive(Default, Debug, Clone)]
struct Warehouse {
    max_x: usize,
    max_y: usize,
    original_robot: Robot,
    robot: Robot,
    original_locations: HashMap<Point<usize>, Obstacle>,
    locations: HashMap<Point<usize>, Obstacle>,
    instructions: Vec<Direction>,
    instruction_queue: VecDeque<Direction>,
    iterations: usize,
    delay: f64,
    running: bool,
    delta: f64,
}

// fixme
impl Warehouse {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        let file = "aoc2024/src/bin/day15/day15_data.txt";
        let mut robot: Robot = Default::default();
        let mut locations: HashMap<Point<usize>, Obstacle> = HashMap::new();
        let mut instructions = Vec::new();
        let contents = fs::read_to_string(file).expect("Can't read the file");
        let mut map = true;
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in contents.lines().enumerate() {
            if line.is_empty() {
                map = false;
                continue;
            }
            if map {
                for (x, c) in line.chars().enumerate() {
                    match c {
                        'O' => {
                            locations.entry(Point { x, y }).or_insert(Obstacle::Box);
                        }
                        '#' => {
                            locations.entry(Point { x, y }).or_insert(Obstacle::Wall);
                        }
                        '@' => {
                            robot.pos.x = x;
                            robot.pos.y = y;
                        }
                        _ => (),
                    }
                }
                max_x = line.len();
                max_y = max_y.max(y);
            } else {
                for i in line.chars() {
                    match i {
                        '^' => instructions.push(Direction::North),
                        '>' => instructions.push(Direction::East),
                        'v' => instructions.push(Direction::South),
                        '<' => instructions.push(Direction::West),
                        _ => (),
                    }
                }
            }
        }
        let instruction_queue = VecDeque::from(instructions.clone());
        let original_robot = robot.clone();
        let original_locations = locations.clone();

        Warehouse {
            max_x,
            max_y,
            original_robot,
            robot,
            original_locations,
            locations,
            instructions,
            instruction_queue,
            iterations: 0,
            delay: 50.0,
            running: true,
            delta: 0.0,
        }
    }

    fn reset_warehouse(&mut self) {
        self.instruction_queue.drain(0..);
        self.instruction_queue = VecDeque::from(self.instructions.clone());
        self.robot = self.original_robot.clone();
        self.locations = self.original_locations.clone();
        self.iterations = 0;
    }

    fn move_robot(&mut self, instruction: Direction) {
        let mut proposed_robot_move = self.robot.pos;
        match instruction {
            Direction::North => proposed_robot_move.y -= 1,
            Direction::East => proposed_robot_move.x += 1,
            Direction::South => proposed_robot_move.y += 1,
            Direction::West => proposed_robot_move.x -= 1,
        }

        if self.move_obstacle(proposed_robot_move, instruction) {
            self.robot.pos = proposed_robot_move;
            self.robot.moved_successfully = true;
        } else {
            self.robot.moved_successfully = false;
        }
    }

    fn move_obstacle(&mut self, proposed_move: Point<usize>, instruction: Direction) -> bool {
        match self.locations.get(&proposed_move) {
            Some(obstacle) => {
                match obstacle {
                    // wall blocks movement
                    Obstacle::Wall => false,
                    // if box { check if box can move, move if yes}
                    Obstacle::Box => {
                        let mut next_move = proposed_move;
                        match instruction {
                            Direction::North => next_move.y -= 1,
                            Direction::East => next_move.x += 1,
                            Direction::South => next_move.y += 1,
                            Direction::West => next_move.x -= 1,
                        }
                        if self.move_obstacle(next_move, instruction) {
                            // insert box at next_move
                            self.locations.entry(next_move).or_insert(Obstacle::Box);
                            // remove the box at proposed_move
                            self.locations.remove(&proposed_move);
                            true
                        } else {
                            false
                        }
                    }
                }
            }
            // No obstacle, free to move
            None => true,
        }
    }
}

impl eframe::App for Warehouse {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Top").show(ctx, |top_ui| {
            if top_ui.button("Reset Warehouse").clicked() {
                self.reset_warehouse();
            }
            // todo ugh! egui has some weirdness with layout esp using the full width,
            //  so I'm using columns. However this makes the stop / start button elongated
            //  and a bit ugly.
            top_ui.columns(3, |cols| {
                cols[0].vertical_centered_justified(|ui| {
                    ui.label(format!("Iterations: {}", self.iterations))
                });
                cols[1].vertical_centered_justified(|ui| {
                    if ui.button("Stop / Start").clicked() {
                        self.running = !self.running;
                    }
                });
                cols[2].vertical_centered_justified(|ui| {
                    ui.add(egui::Slider::new(&mut self.delay, 0.0..=500.0).text("Delay ms"))
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |central_ui| {
            egui::Frame::canvas(central_ui.style()).show(central_ui, |canvas_ui| {

                let pos_on_canvas = | wh_pos: Point<usize> | -> (Pos2, f32) {
                    // input warehouse coordinates
                    // return canvas points centre position, ui canvas spacing
                    let canvas_rect = canvas_ui.max_rect();
                    let spacing = (canvas_rect.width() / self.max_x as f32)
                        .min(canvas_rect.height() / self.max_y as f32);
                    (
                        Pos2 {
                            x: spacing * wh_pos.x as f32 + canvas_rect.min.x + spacing / 2.0,
                            y: spacing * wh_pos.y as f32 + canvas_rect.min.y + spacing / 2.0,
                        },
                        spacing,
                    )
                };

                // Walls and boxes
                for (pos, obstacle) in &self.locations {
                    let (canvas_pos, spacing) = pos_on_canvas(*pos);
                    match obstacle {
                        Obstacle::Wall => {
                            canvas_ui.painter().rect_filled(
                                egui::Rect::from_center_size(
                                    canvas_pos,
                                    Vec2 {
                                        x: spacing,
                                        y: spacing,
                                    },
                                ),
                                egui::CornerRadius::default(),
                                egui::Color32::RED,
                            );
                        }
                        Obstacle::Box => {
                            canvas_ui.painter().rect_filled(
                                egui::Rect::from_center_size(
                                    canvas_pos,
                                    Vec2 {
                                        // 2 point gap
                                        x: spacing - 2.0,
                                        y: spacing - 2.0,
                                    },
                                ),
                                egui::CornerRadius::from(spacing / 5.0f32),
                                egui::Color32::GREEN,
                            );
                        }
                    }
                }

                // Robot
                let (robot_pos, increment) = pos_on_canvas(self.robot.pos);
                if self.robot.moved_successfully {
                    canvas_ui
                        .painter()
                        .circle_filled(robot_pos, increment * 0.5, egui::Color32::CYAN);
                } else {
                    // robot turns the air BLUE if it can't move
                    canvas_ui
                        .painter()
                        .circle_filled(robot_pos, increment * 0.8, egui::Color32::BLUE);
                }
            });
        });

        // call modified robot move, take each instruction as a seq, then display warehouse
        // on next loop
        // instead of trying to slow it down, instead we check whether enough time has elapsed
        // since the last warehouse update. if so, then we run it

        let delta = ctx.input(|i| i.time);
        if delta - self.delta > self.delay / 1000.0 {
            self.delta = delta;
            if self.running {
                if let Some(i) = self.instruction_queue.pop_front() {
                    self.move_robot(i);
                    self.iterations += 1;
                }
            }
        }
        ctx.request_repaint();

    }
}
