use image::{ImageBuffer, Rgba, RgbaImage};
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::{env, fs};

mod common_2d;
mod line_2d;
mod poly_2d;
mod viewer;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;
const IMAGE_FILE: &str = "output.png";

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Geometry {
    Line,
    Poly,
}

#[derive(Serialize, Deserialize)]
struct DrawCommand {
    geometry: Geometry,
    coords: Vec<[i32; 2]>,
    stroke_width: Option<f64>,
    color: Option<[i32; 4]>,
}

struct Canvas {
    image: RgbaImage,
}

impl Canvas {
    fn new(width: i32, height: i32) -> Self {
        Canvas {
            image: ImageBuffer::new(width as u32, height as u32),
        }
    }
}

impl common_2d::PutPixel for Canvas {
    fn put_pixel(&mut self, x: i32, y: i32) {
        self.image[(x as u32, y as u32)] = Rgba([0, 255, 255, 255]);
    }

    fn put_pixel_alpha(&mut self, x: i32, y: i32, alpha: u8) {
        self.image[(x as u32, y as u32)] = Rgba([0, 255, 255, alpha]);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(argument) = args.get(1) {
        if argument == "viewer" {
            viewer::run_viewer(WIDTH, HEIGHT, IMAGE_FILE)?;
        }
    } else {
        let mut canvas = Canvas::new(WIDTH, HEIGHT);

        let data_input = fs::read_to_string("commands.json")?;
        let commands: Vec<DrawCommand> = serde_json::from_str(&data_input)?;

        {
            for command in commands.iter() {
                let coords = &command.coords;

                if command.geometry == Geometry::Line {
                    println!("Line coords: {:?}", coords);
                    let start = coords[0];
                    let end = coords[1];

                    match command.stroke_width {
                        None => line_2d::plot_line(&mut canvas, start[0], start[1], end[0], end[1]),
                        Some(stroke_width) => {
                            line_2d::plot_line_wide(
                                &mut canvas,
                                start[0],
                                start[1],
                                end[0],
                                end[1],
                                stroke_width,
                            );
                        }
                    }
                } else if command.geometry == Geometry::Poly {
                    println!("Poly coords: {:?}", coords);

                    poly_2d::draw_poly(&mut canvas, WIDTH, HEIGHT, &coords);
                }
            }
        }

        canvas.image.save(IMAGE_FILE)?;
        println!("Rendered imaged!");
    }

    Ok(())
}
