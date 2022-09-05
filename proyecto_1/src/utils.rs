use anyhow::{anyhow, Result};
use image::GrayImage;

/// Transforms GrayImage into a Vec<u32> with ARGB format
pub fn gray_to_vec32(img: &GrayImage) -> Vec<u32> {
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
        .into_iter()
        .enumerate()
        .map(|(i, pixel)| {
            let y: f64 = (i / (width as usize)) as f64;
            let x: f64 = (i % (width as usize)) as f64;

            if (x > GRID_WIDTH && x % width_nth < GRID_WIDTH)
                || (y > GRID_WIDTH && y % height_nth < GRID_WIDTH)
            {
                return 0x00ff0000;
            };
            pixel
        })
        .collect())
}

/// Enumerates squares on an n*n grid from 1..n*n and returns which square the coords (x, y) are
/// inside of
pub fn check_grid(x: f64, y: f64, width: u32, height: u32, n: u32) -> u32 {
    let n64 = n as f64;
    let width_nth: f64 = width as f64 / n64;
    let height_nth: f64 = height as f64 / n64;

    let x: u32 = (x / width_nth).floor() as u32 + 1;
    let y: u32 = (y / height_nth).floor() as u32 * n;

    x + y
}
