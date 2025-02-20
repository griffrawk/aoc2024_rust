// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::{HashMap, VecDeque};
use std::{env, fs};
use std::time::Duration;
use eframe::egui::{self, RichText};
use aocutils::point::Point;

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
    robot: Robot,
    locations: HashMap<Point<usize>, Obstacle>,
    instructions: Vec<Direction>,
    instruction_queue: VecDeque<Direction>,
}

// fixme
impl Default for Warehouse {
    fn default() -> Self {
        // let path = env::current_dir().unwrap();
        // println!("The current directory is {}", path.display());
        let file = "aoc2024/src/bin/day15/day15_test.txt";
        let mut robot: Robot = Default::default();
        let mut locations: HashMap<Point<usize>, Obstacle> = HashMap::new();
        let mut instructions = Vec::new();
        let contents = fs::read_to_string(file).expect("Can't read the file");
        let mut map = true;
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
        let mut instruction_queue = VecDeque::from(instructions.clone());

        Warehouse {
            robot,
            locations,
            instructions,
            instruction_queue,
        }
    }
}

impl Warehouse {
    fn reset_instructions(&mut self) {
        self.instruction_queue.drain(0..);
        self.instruction_queue = VecDeque::from(self.instructions.clone());

        // todo also has to reset the warehouse and robot position!
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
                self.reset_instructions();
            }
        });
        egui::CentralPanel::default().show(ctx, |central_ui| {
            egui::Frame::canvas(central_ui.style()).show(central_ui, |canvas_ui| {
                // todo replace with wall, box, and robot drawing
                //  needs some scaling code for logical array size vs logical screen size,
                //  and provide some gaps between boxes, eg 18x18 on a 20x20 grid
                canvas_ui.painter().rect_filled(
                    egui::Rect::from_x_y_ranges(20.0..=50.0, 10.0..=50.0),
                    egui::CornerRadius::default(),
                    egui::Color32::GREEN,
                );

                // Robot
                canvas_ui.painter().circle_filled(
                    egui::Pos2::new((self.robot.pos.x * 50) as f32, (self.robot.pos.y * 50) as f32),
                    20.0,
                    egui::Color32::BLUE);
            });

        });

        // call modified robot move, take each instruction as a seq, then display warehouse
        // on next loop
        if let Some(i) = self.instruction_queue.pop_front() {
            self.move_robot(i);
        }

        ctx.request_repaint_after(Duration::from_nanos(500000));
    }
}