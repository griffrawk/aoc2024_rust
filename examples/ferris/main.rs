use std::env;
use eframe::egui::{self, Color32, RichText};
use aoc2024::day_15::day15::Warehouse;

fn main() -> Result<(), eframe::Error> {
    fred();
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 320.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

// the model
struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Andrew".to_owned(),
            age: 59,
        }
    }
}

impl eframe::App for MyApp {
    // this is run in a loop 60fps or thererabouts
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                // coloured labels need to be rich text
                let name_label = ui.label(RichText::new("Your name: ").color(Color32::RED));
                // input field
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            // slider
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            // this acts as the definer for a button, and its action
            let increment_button = ui.button("Increment");

            if increment_button.clicked() {
                self.age += 1;
            }
            if ui.button("Decrement").clicked() {
                self.age -= 1;
            }
            // another label
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // an image
            ui.image(egui::include_image!(
                "ferris.png"
            ));
        });
    }
}

fn fred() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    let warehouse = Warehouse::new("aoc2024/src/day_15/day15_basic.txt");
    dbg!(warehouse);
}