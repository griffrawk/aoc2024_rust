// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use aocutils::point::Point;
use eframe::egui::{self, Pos2, Rect, RichText};
use eframe::emath::Vec2;
use std::collections::{HashMap, VecDeque};
use std::{env, fs};

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
            Ok(Box::<Warehouse>::default())
        }),
    )
}

// the model, this would be the warehouse from the puzzle code
// todo change Point references to the egui Point probably
#[derive(Debug, Clone, Default)]
struct Robot {
    pos: Point<usize>,
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

// todo change Point references to the egui Point probably
#[derive(Debug, Clone)]
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
}

// fixme
impl Default for Warehouse {
    fn default() -> Self {
        // let path = env::current_dir().unwrap();
        // println!("The current directory is {}", path.display());
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
        }
    }
}

impl Warehouse {
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
            None => true,
        }
    }
}

impl eframe::App for Warehouse {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Top").show(ctx, |top_ui| {
            // todo space the panel out a bit, put counter in here and a slider to control speed
            if top_ui.button("Reset Warehouse").clicked() {
                self.reset_warehouse();
            }
            top_ui.horizontal(|ui|{
                ui.label(format!("Iterations: {}", self.iterations));
            })
        });
        egui::CentralPanel::default().show(ctx, |central_ui| {
            egui::Frame::canvas(central_ui.style()).show(central_ui, |canvas_ui| {
                
                let phys_pos = |pos: Point<usize>| -> (Pos2, f32) {
                    let canvas_dim = canvas_ui.max_rect();
                    // return centre position, increment (to base radii etc. on)
                    let increment = (canvas_dim.width() / self.max_x as f32)
                        .min(canvas_dim.height() / self.max_y as f32);
                    (
                        Pos2 {
                            x: increment * pos.x as f32 + canvas_dim.min.x + increment / 2.0,
                            y: increment * pos.y as f32 + canvas_dim.min.y + increment / 2.0,
                        },
                        increment,
                    )
                };
                
                // Walls and boxes
                for (pos, obstacle) in &self.locations {
                    let (canvas_pos, increment) = phys_pos(*pos);
                    match obstacle {
                        Obstacle::Wall => {
                            canvas_ui.painter().rect_filled(
                                egui::Rect::from_center_size(
                                    canvas_pos,
                                    Vec2 {
                                        // 2 point gap
                                        x: increment - 2.0,
                                        y: increment - 2.0,
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
                                        x: increment - 2.0,
                                        y: increment - 2.0,
                                    },
                                ),
                                egui::CornerRadius::default(),
                                egui::Color32::GREEN,
                            );
                        }
                    }
                }

                // Robot
                let (robot_pos, increment) = phys_pos(self.robot.pos);

                canvas_ui
                    .painter()
                    // 2 point gap
                    .circle_filled(robot_pos, increment / 2.0 - 2.0, egui::Color32::CYAN);
            });
        });

        // call modified robot move, take each instruction as a seq, then display warehouse
        // on next loop
        if let Some(i) = self.instruction_queue.pop_front() {
            self.move_robot(i);
            self.iterations += 1;
        }

        ctx.request_repaint();
    }
}
