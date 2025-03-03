// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, RichText};

fn main() -> Result<(), eframe::Error> {
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
        egui::TopBottomPanel::top("wibble").show(ctx, |foo| {
            foo.label("A canvas");
            egui::Frame::canvas(foo.style()).show(foo, |ui| {
                ui.painter().rect_filled(
                    egui::Rect::from_x_y_ranges(10.0..=50.0, 10.0..=50.0),
                    egui::CornerRadius::default(),
                    egui::Color32::from_rgb(190, 120, 200),
                );
            });
        });
        egui::TopBottomPanel::bottom("wobble").show(ctx, |foo| {
            foo.add(egui::Slider::new(&mut self.age, 0..=300).text("fo"));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                // coloured labels need to be rich text
                let name_label = ui.label(RichText::new("Your name: ").color(egui::Color32::RED));
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
            ui.image(egui::include_image!("ferris.png"));
        });
    }
}
