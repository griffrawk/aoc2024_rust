use crate::day16_graph::{Graph, Node};
use plotters::backend::BitMapBackend;
use plotters::coord::types::RangedCoordi32;
use plotters::prelude::full_palette::GREY;
use plotters::prelude::*;

const OUTPUT_FILENAME: &str = "src/bin/day16/output/day16_gen";

impl Graph {
    pub fn visual_plot(&mut self, last: bool) -> Result<(), Box<dyn std::error::Error>> {
        let out = format!("{}_{:06}{}", OUTPUT_FILENAME, self.plot_sequence, ".png");
        let root_area = BitMapBackend::new(&out, (1024, 1024)).into_drawing_area();

        root_area.fill(&WHITE).unwrap();
        let end_x = self.xrange.end;
        let end_y = self.yrange.end;
        let root_area =
            root_area.apply_coord_spec(Cartesian2d::<RangedCoordi32, RangedCoordi32>::new(
                0..end_x,
                0..end_y,
                (0..1024, 0..1024),
            ));

        let block_side = 1024 / self.yrange.end + 1;
        let node_block = |x: i32, y: i32, max_cost: i32, node: Node| {
            return EmptyElement::at((x, y))
                + Rectangle::new(
                    [(0, 0), (block_side, block_side)],
                    ShapeStyle::from(&MandelbrotHSL::get_color_normalized(
                        node.g_cost as f64,
                        0.0,
                        max_cost as f64,
                    ))
                    .filled(),
                );
        };
        let block = |x: i32, y: i32, c: RGBColor| {
            return EmptyElement::at((x, y))
                + Rectangle::new(
                    [(0, 0), (block_side, block_side)],
                    ShapeStyle::from(&c).filled(),
                );
        };

        for pos in self.walls.clone() {
            root_area.draw(&block(pos.x, pos.y, GREY))?;
        }

        // todo revisit this for animation maybe. not convinced the calc is correct
        //  as different to end cost as below, when found at the last frame
        // let end_cost = self.node_list[&self.end].cost;
        // let max_cost = 1 + 10 *
        //     self.node_list
        //     .values()
        //     .fold(0, |acc, node| {
        //         if node.cost < usize::MAX {
        //             max(acc, node.cost)
        //         } else {
        //             0
        //         }
        //     });
        let max_cost = 107512;

        // dbg!(max_cost);

        for (pos, node) in self.node_list.clone() {
            if node.g_cost < i32::MAX || node.seen {
                root_area.draw(&node_block(pos.x, pos.y, max_cost, node))?;
            }
        }

        if last {
            for pos in self.show_path() {
                root_area.draw(&block(pos.x, pos.y, BLACK))?;
            }
        }
        root_area.draw(&block(self.start.x, self.start.y, RED))?;
        root_area.draw(&block(self.end.x, self.end.y, GREEN))?;

        root_area.present()?;
        Ok(())
    }

    pub fn a_star_visual_plot(
        &mut self,
        max_cost: i32,
        last: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let out = format!("{}_{:06}{}", OUTPUT_FILENAME, self.plot_sequence, ".png");
        let root_area = BitMapBackend::new(&out, (1024, 1024)).into_drawing_area();

        root_area.fill(&WHITE).unwrap();
        let end_x = self.xrange.end;
        let end_y = self.yrange.end;
        let root_area =
            root_area.apply_coord_spec(Cartesian2d::<RangedCoordi32, RangedCoordi32>::new(
                0..end_x,
                0..end_y,
                (0..1024, 0..1024),
            ));

        let block_side = 1024 / self.yrange.end + 1;
        let node_block = |x: i32, y: i32, max_cost: i32, node: Node, last: bool| {
            let mut bg_colour =
                MandelbrotHSL::get_color_normalized(node.f_est_cost as f64, 0.0, max_cost as f64);
            let mut fg_colour = BLACK;
            if last {
                bg_colour = HSLColor(0.2, 0.1, 0.3);
                fg_colour = WHITE;
            }
            EmptyElement::at((x, y))
                + Rectangle::new(
                    [(0, 0), (block_side, block_side)],
                    ShapeStyle::from(&bg_colour).filled(),
                )
                + Text::new(
                    format!("{}", node.f_est_cost),
                    (10, 10),
                    ("sans-serif", 30).into_font().color(&fg_colour),
                )
        };

        let block = |x: i32, y: i32, c: RGBColor| {
            return EmptyElement::at((x, y))
                + Rectangle::new(
                    [(0, 0), (block_side, block_side)],
                    ShapeStyle::from(&c).filled(),
                );
        };

        for pos in self.walls.clone() {
            root_area.draw(&block(pos.x, pos.y, GREY))?;
        }

        for (pos, node) in self.node_list.clone() {
            if node.g_cost < i32::MAX {
                root_area.draw(&node_block(pos.x, pos.y, max_cost, node, false))?;
            }
        }

        if last {
            for pos in self.show_path() {
                let node = self.node_list[&pos].clone();
                root_area.draw(&node_block(pos.x, pos.y, max_cost, node, last))?;
            }
        }
        root_area.draw(&block(self.start.x, self.start.y, RED))?;
        root_area.draw(&block(self.end.x, self.end.y, GREEN))?;

        root_area.present()?;
        Ok(())
    }
}
