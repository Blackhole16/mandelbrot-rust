extern crate num;
extern crate image;

use num::complex::Complex;
use image::ImageBuffer;
use std::fs::File;

fn main() {
    const ITERATIONS: i32 = 10000;

    const RSTART: f64 = -2f64;
    const REND: f64 = 2f64;
    const ISTART: f64 = -1.5;
    const IEND: f64 = 1.5;

    const WIDTH: i32 = 1920;
    const HEIGHT: i32 = 1080;

    const RD: f64 = (REND - RSTART) / WIDTH as f64;
    const ID: f64 = (IEND - ISTART) / HEIGHT as f64;

    let mut arr = [[ITERATIONS; WIDTH as usize]; HEIGHT as usize];
    let zero = Complex::new(0.0, 0.0);

    for y in 0..arr.len() {
        for x in 0..arr[0].len() {
            let c = Complex::new((x as f64)*RD + RSTART, (y as f64)*ID + ISTART);
            let mut z = zero;
            for i in 0..ITERATIONS {
                z = z*z + c;
                if z.norm_sqr() >= 2500f64 {
                    arr[y as usize][x as usize] = i;
                    break;
                }
            }
        }
    }
    let imgbuf = ImageBuffer::from_fn(WIDTH as u32, HEIGHT as u32, |x,y| {
        let i = arr[y as usize][x as usize] as f64;
        let id = i / ITERATIONS as f64;
        let ir = id.powf(0.01);
        let ic = ir * 255f64;
        image::Luma([ic as u8]) 
    });

    // Save the image as “fractal.png”
    let mut file = File::create("fractal.png").unwrap();

    // We must indicate the image’s color type and what format to save as
    image::ImageLuma8(imgbuf).save(&mut file, image::PNG);
}
