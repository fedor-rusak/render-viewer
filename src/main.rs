use image::{imageops, Rgba, RgbaImage};
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
    Tile,
}

#[derive(Serialize, Deserialize)]
struct DrawCommand {
    geometry: Geometry,
    coords: Vec<[i32; 2]>,
    src: Option<String>,
    stroke_width: Option<f64>,
    color: Option<[i32; 4]>,
}

struct Canvas {
    image: RgbaImage,
}

impl Canvas {
    fn new(width: i32, height: i32) -> Self {
        Canvas {
            image: RgbaImage::new(width as u32, height as u32),
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
                    println!("Line\n  coords: {:?}", coords);
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
                    println!("Poly\n  coords: {:?}", coords);

                    poly_2d::draw_poly(&mut canvas, WIDTH, HEIGHT, &coords);
                } else if command.geometry == Geometry::Tile {
                    println!("Tile\n  coords: {:?}", coords);

                    let mut tile = match &command.src {
                        None => panic!("Tile has no src element!"),
                        Some(src) => {
                            println!("  src: {:?}", src);
                            image::open(src)?
                        }
                    };
                    let [x, y] = coords[0];

                    if x >= 0 && y >= 0 {
                        imageops::overlay(&mut canvas.image, &tile, x as u32, y as u32);
                    } else {
                        let (width, height) = tile.to_rgba8().dimensions();
                        if width as i32 + x <= 0 || height as i32 + y <= 0 {
                            panic!("Tile offset suggests tile render outside image!")
                        }

                        let (crop_x, crop_width) = if x < 0 {
                            (-x, width as i32 + x)
                        } else {
                            (0, width as i32)
                        };
                        let (crop_y, crop_height) = if y < 0 {
                            (-y, height as i32 + y)
                        } else {
                            (0, height as i32)
                        };
                        let cropped = imageops::crop(
                            &mut tile,
                            crop_x as u32,
                            crop_y as u32,
                            crop_width as u32,
                            crop_height as u32,
                        );

                        let pos_x = if x < 0 { 0 } else { x };
                        let pos_y = if y < 0 { 0 } else { y };
                        imageops::overlay(&mut canvas.image, &cropped, pos_x as u32, pos_y as u32);
                    }
                }
            }
        }

        canvas.image.save(IMAGE_FILE)?;
        println!("Rendered imaged!");
    }

    Ok(())
}
