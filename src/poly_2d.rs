/*
 * Found through:
 *
 * https://stackoverflow.com/questions/1341399/rasterizing-a-2d-polygon
 *
 * Changed by me to fit my use case.
 *
 * https://gitlab.com/ideasman42/rust-simple-examples/blob/master/draw_poly_2d_callback/src/main.rs
 *
 * Based on this (probably?):
 *
 * https://alienryderflex.com/polygon_fill/
 *
 * This is a simple, self contained example of how to fill a polygon on a grid (raster image),
 * using the following command:
 *
 * This code is an example of:
 *
 * - Using a callback.
 * - Vector of tuples.
 * - Vector manipulation.
 */

// Used to store x intersections for the current y axis ('pixel_y')
struct NodeX {
    span_y_index: usize,
    // 'x' pixel value for the current 'pixel_y'.
    x: i32,
}

/**
 *
 * - callback: Takes the x, y coords and x-span (x_end is not inclusive),
 *   note that `x_end` will always be greater than `x`.
 *
 */
pub fn draw_poly<F: FnMut(i32, i32, i32)>(
    xmax: i32,
    ymax: i32,
    coords: &Vec<[i32; 2]>,
    callback: &mut F,
) {
    /* Originally by Darel Rex Finley, 2007.
     * Optimized by Campbell Barton, 2016 to keep sorted intersections. */

    /*
     * Note: all the index lookups here could be made unsafe
     * (as in, we know they won't fail).
     */

    let (xmin, ymin) = (0, 0);
    // only because we use this with int values frequently, avoids casting every time.
    let coords_len: i32 = coords.len() as i32;

    let mut span_y: Vec<[i32; 2]> = Vec::with_capacity(coords.len());

    {
        let mut i_prev: i32 = coords_len - 1;
        let mut i_curr: i32 = 0;
        let mut co_prev = &coords[i_prev as usize];
        for co_curr in coords {
            if co_prev[1] != co_curr[1] {
                // Any segments entirely above or below the area of interest can be skipped.
                if (std::cmp::min(co_prev[1], co_curr[1]) >= ymax)
                    || (std::cmp::max(co_prev[1], co_curr[1]) < ymin)
                {
                    continue;
                }

                span_y.push(if co_prev[1] < co_curr[1] {
                    [i_prev, i_curr]
                } else {
                    [i_curr, i_prev]
                });
            }
            i_prev = i_curr;
            i_curr += 1;
            co_prev = co_curr;
        }
    }

    // sort edge-segments on y, then x axis
    span_y.sort_by(|a, b| {
        let co_a = &coords[a[0] as usize];
        let co_b = &coords[b[0] as usize];
        let mut ord = co_a[1].cmp(&co_b[1]);
        if ord == std::cmp::Ordering::Equal {
            ord = co_a[0].cmp(&co_b[0]);
        }
        if ord == std::cmp::Ordering::Equal {
            // co_a & co_b are identical, use the line closest to the x-min
            let co = co_a; // could be co_b too.
            let co_a = &coords[a[1] as usize];
            let co_b = &coords[b[1] as usize];
            ord = 0.cmp(
                &(((co_b[0] - co[0]) * (co_a[1] - co[1]))
                    - ((co_a[0] - co[0]) * (co_b[1] - co[1]))),
            );
        }
        ord
    });

    let mut node_x: Vec<NodeX> = Vec::with_capacity(coords.len() + 1);
    let mut span_y_index: usize = 0;

    if span_y.len() != 0 && coords[span_y[0][0] as usize][1] < ymin {
        while (span_y_index < span_y.len()) && (coords[span_y[span_y_index][0] as usize][1] < ymin)
        {
            assert!(
                coords[span_y[span_y_index][0] as usize][1]
                    < coords[span_y[span_y_index][1] as usize][1]
            );
            if coords[span_y[span_y_index][1] as usize][1] >= ymin {
                node_x.push(NodeX {
                    span_y_index: span_y_index,
                    x: -1,
                });
            }
            span_y_index += 1;
        }
    }

    // Loop through the rows of the image.
    for pixel_y in ymin..ymax {
        let mut is_sorted = true;
        let mut do_remove = false;
        {
            let mut x_ix_prev = i32::min_value();
            for n in &mut node_x {
                let s = &span_y[n.span_y_index];
                let co_prev = &coords[s[0] as usize];
                let co_curr = &coords[s[1] as usize];

                assert!(co_prev[1] < pixel_y && co_curr[1] >= pixel_y);
                let x = (co_prev[0] - co_curr[0]) as f64;
                let y = (co_prev[1] - co_curr[1]) as f64;
                let y_px = (pixel_y - co_curr[1]) as f64;
                let x_ix = ((co_curr[0] as f64) + ((y_px / y) * x)).round() as i32;
                n.x = x_ix;

                if is_sorted && (x_ix_prev > x_ix) {
                    is_sorted = false;
                }
                if do_remove == false && co_curr[1] == pixel_y {
                    do_remove = true;
                }
                x_ix_prev = x_ix;
            }
        }
        // Theres no reason this will ever be larger
        assert!(node_x.len() <= coords.len() + 1);

        // Sort the nodes, via a simple "bubble" sort.
        if is_sorted == false {
            let node_x_end = node_x.len() - 1;
            let mut i: usize = 0;
            while i < node_x_end {
                if node_x[i].x > node_x[i + 1].x {
                    node_x.swap(i, i + 1);
                    if i != 0 {
                        i -= 1;
                    }
                } else {
                    i += 1;
                }
            }
        }

        // Fill the pixels between node pairs.
        {
            let mut i = 0;
            while i < node_x.len() {
                let mut x_src = node_x[i].x;
                let mut x_dst = node_x[i + 1].x;

                if x_src >= xmax {
                    break;
                }

                if x_dst > xmin {
                    if x_src < xmin {
                        x_src = xmin;
                    }
                    if x_dst > xmax {
                        x_dst = xmax;
                    }

                    // Single call per x-span.
                    if x_src < x_dst {
                        callback(x_src - xmin, x_dst - xmin, pixel_y - ymin);
                    }
                }
                i += 2;
            }
        }

        // Clear finalized nodes in one pass, only when needed
        // (avoids excessive array-resizing).
        if do_remove {
            let mut i_dst: usize = 0;
            for i_src in 0..node_x.len() {
                let s = &span_y[node_x[i_src].span_y_index];
                let co = &coords[s[1] as usize];
                if co[1] != pixel_y {
                    if i_dst != i_src {
                        // x is initialized for the next pixel_y (no need to adjust here)
                        node_x[i_dst].span_y_index = node_x[i_src].span_y_index;
                    }
                    i_dst += 1;
                }
            }
            node_x.truncate(i_dst);
        }

        // Scan for new events
        {
            while span_y_index < span_y.len()
                && coords[span_y[span_y_index][0] as usize][1] == pixel_y
            {
                // Note, node_x these are just added at the end,
                // not ideal but sorting once will resolve.

                // x is initialized for the next pixel_y
                node_x.push(NodeX {
                    span_y_index: span_y_index,
                    x: -1,
                });
                span_y_index += 1;
            }
        }
    }
}
