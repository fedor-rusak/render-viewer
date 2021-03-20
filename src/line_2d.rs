use image::{Rgba, RgbaImage};
use std::cmp;

pub fn plot_line<F: FnMut(i32, i32)>(x0: i32, y0: i32, x1: i32, y1: i32, callback: &mut F) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = (y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy; /* error value e_xy */

    let mut x = x0;
    let mut y = y0;
    loop {
        callback(x, y);

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 >= -dy {
            err -= dy;
            x += sx;
        }

        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

//fot anti-aliasing. Which means to make lines appear smooooth on pixel grid.
fn get_alpha(value: i32, ed: f64, wd: f64) -> u8 {
    cmp::max(
        0,
        255 - (255.0 * (value.abs() as f64 / ed - wd + 1.0)) as u8,
    )
}

// some cool stuff!! http://members.chello.at/~easyfilter/bresenham.html
pub fn plot_line_wide(
    image: &mut RgbaImage,
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    width: f64,
) {
    let delta_x = (end_x - start_x).abs();
    let step_x = if start_x < end_x { 1 } else { -1 };
    let delta_y = (end_y - start_y).abs();
    let step_y = if start_y < end_y { 1 } else { -1 };
    let mut err = delta_x - delta_y;
    let (mut temp_err, mut temp_x, mut temp_y): (i32, i32, i32);
    let half_width = (width + 1.0) / 2.0;
    let (mut x, mut y) = (start_x, start_y);
    let diagonal = if delta_x + delta_y == 0 {
        1.0
    } else {
        ((delta_x * delta_x + delta_y * delta_y) as f64).sqrt()
    };

    loop {
        let alpha = get_alpha(err - delta_x + delta_y, diagonal, half_width);
        image[(x as u32, y as u32)] = Rgba([0, 255, 255, alpha]);

        temp_err = err;
        temp_x = x; //can be modifidiagonal in vertical pixels step!
        if 2 * temp_err >= -delta_x {
            /* vertical pixels step */
            temp_err += delta_y;
            temp_y = y;
            while (temp_err as f64) < diagonal * half_width
                && (end_y != temp_y || delta_x > delta_y)
            {
                temp_err += delta_x;
                temp_y += step_y;
                let alpha = get_alpha(temp_err, diagonal, half_width);
                image[(x as u32, temp_y as u32)] = Rgba([0, 255, 255, alpha]);
            }
            if x == end_x {
                break;
            }
            temp_err = err;
            err -= delta_y;
            x += step_x;
        }

        if 2 * temp_err <= delta_y {
            /* horizontal pixels step */
            temp_err = delta_x - temp_err;
            while (temp_err as f64) < diagonal * half_width
                && (end_x != temp_x || delta_x < delta_y)
            {
                temp_err += delta_y;
                temp_x += step_x;
                let alpha = get_alpha(temp_err, diagonal, half_width);
                image[(temp_x as u32, y as u32)] = Rgba([0, 255, 255, alpha]);
            }
            if y == end_y {
                break;
            }
            err += delta_x;
            y += step_y;
        }
    }
}
