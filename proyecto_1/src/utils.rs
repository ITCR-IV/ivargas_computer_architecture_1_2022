use anyhow::{anyhow, Result};
use image::{GrayImage, ImageBuffer, Pixel};

/// Transforms GrayImage into a Vec<u32> with ARGB format
pub fn gray_to_vec32(img: GrayImage) -> Vec<u32> {
    img.as_raw()
        .iter()
        .map(|luma| {
            let luma = *luma as u32;
            luma | luma << 8 | luma << 16
        })
        .collect()
}

const GRID_WIDTH: f64 = 3.0;

/// Draws an n*n red grid on top of the image represented by "buffer" in an ARGB format
pub fn draw_grid(buffer: Vec<u32>, width: u32, height: u32, n: u32) -> Result<Vec<u32>> {
    if (width * height) as usize != buffer.len() {
        return Err(anyhow!("width*height must match buffer's size"));
    }

    let n = n as f64;
    let width_nth: f64 = width as f64 / n;
    let height_nth: f64 = height as f64 / n;
    Ok(buffer
        .iter()
        .enumerate()
        .map(|(i, pixel)| {
            let y: f64 = (i / (width as usize)) as f64;
            let x: f64 = (i % (width as usize)) as f64;

            if (x > GRID_WIDTH && x % width_nth < GRID_WIDTH)
                || (y > GRID_WIDTH && y % height_nth < GRID_WIDTH)
            {
                return 0x00ff0000;
            };
            *pixel
        })
        .collect())
}

/// Enumerates squares on an n*n grid from 0..n*n-1 and returns which square the coords (x, y) are
/// inside of
pub fn check_grid(x: f64, y: f64, width: u32, height: u32, n: u32) -> u32 {
    let n64 = n as f64;
    let width_nth: f64 = width as f64 / n64;
    let height_nth: f64 = height as f64 / n64;

    let x: u32 = (x / width_nth).floor() as u32;
    let y: u32 = (y / height_nth).floor() as u32 * n;

    x + y
}

/// Crops (by cloning) the rectangle of `img` that goes from x1->x2 and y1->y2
pub fn cut_image(img: &GrayImage, x1: u32, y1: u32, x2: u32, y2: u32) -> GrayImage {
    let cut_pixels: Vec<u8> = img
        .enumerate_pixels()
        .filter_map(|(x, y, pixel)| {
            // Range has to be exclusive on the "[xy]2" or else when creating the ImageBuffer it'll
            // crash if squares on right-most or down-most edges are selected
            if (x >= x1 && x < x2) && (y >= y1 && y < y2) {
                Some(pixel.channels()[0])
            } else {
                None
            }
        })
        .collect();

    ImageBuffer::from_vec(x2 - x1, y2 - y1, cut_pixels).unwrap()
}
