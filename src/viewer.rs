use fltk::{app, frame::*, image::PngImage, window::*};
use std::error::Error;
use std::fs;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn run_viewer(width: i32, height: i32, image_file: &'static str) -> Result<(), Box<dyn Error + '_>> {
    let app = app::App::default();
    let mut window_instance = Window::default()
        .with_size(width + 10, height + 25) //on Mac borders mess things up
        .with_label("Viewer")
        .center_screen();
    let mut frame = Frame::default().size_of(&window_instance);

    let image = PngImage::load("output.png")?;

    frame.set_image(Some(image));

    // To remove an image
    // frame.set_image(None::<PngImage>);

    window_instance.end();
    window_instance.make_resizable(true);
    window_instance.show();

    let mut modified = fs::metadata("output.png")?.modified()?;
    app::add_idle(move || {
        let execution_time = Instant::now();

        let metadata = match fs::metadata(image_file) {
            Ok(it) => it,
            Err(err) => panic!(err),
        };

        let new_modified = match metadata.modified() {
            Ok(it) => it,
            Err(err) => panic!(err),
        };
        if modified != new_modified {
            modified = new_modified;
            println!("{:?}", modified);

            let new_image = match PngImage::load(image_file) {
                Ok(it) => it,
                Err(err) => panic!(err),
            };
            frame.set_image(Some(new_image));
            window_instance.redraw();
        }

        let elapsed = execution_time.elapsed();

        let millis = 1000.0 / 60.0 - (elapsed.as_millis() as f64);
        sleep(Duration::from_millis(millis as u64));
    });
    app.run()?;

    Ok(())
}
