use image::{ImageBuffer, Rgba, RgbaImage};
use std::env;
use std::error::Error;

mod line_2d;
mod poly_2d;
mod viewer;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;
const IMAGE_FILE: &str = "output.png";

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(argument) = args.get(1) {
        if argument == "viewer" {
            viewer::run_viewer(WIDTH, HEIGHT, IMAGE_FILE)?;
        }
    } else {
        let mut image: RgbaImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
        {
            let mut callback = |x: i32, y: i32| {
                if x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT {
                    image[(x as u32, y as u32)] = Rgba([0, 255, 255, 255]);
                }

                return;
            };
            line_2d::plot_line(10, 10, 50, 100, &mut callback);
        }
        line_2d::plot_line_wide(&mut image, 10 + 100, 10, 50 + 300, 20, 3.0);

        {
            let mut callback = |x_start: i32, x_end: i32, y: i32| {
                for x in x_start..x_end {
                    if x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT {
                        image[(x as u32, y as u32)] = Rgba([0, 255, 255, 255]);
                    }
                }
                return;
            };
            let coords = vec![[300, 200], [300, 400], [400, 300]];

            poly_2d::draw_poly(WIDTH, HEIGHT, &coords, &mut callback);
        }

        image.save(IMAGE_FILE)?;
        println!("Rendered imaged!");
    }

    Ok(())
}
