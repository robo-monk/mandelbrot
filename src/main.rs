extern crate num;
extern crate image;
use image::{ImageBuffer, Rgb, RgbImage};
use num::complex::Complex;
use num::complex::ComplexFloat;
use std::ops;
use palette::{Hsl, Srgb, FromColor};

fn mandelbrot(x: Complex<f64>, z: Complex<f64>) -> Complex<f64>{
    x*x + z
}

struct MandelbrotResult {
    is_in_set: bool, 
    iterations: u32,
    turbulance: f64,
    acceleration: f64,
    sum: Complex<f64>
}

fn run_mandelbrot(z: Complex<f64>, max_iterations: u32) -> MandelbrotResult {
    let mut sum = Complex::new(0.0, 0.0);
    let mut turbulance = 0.0;
    let mut last_mand_value = Complex::new(0.0, 0.0);
    let mut is_in_set = true;

    let mut i: u32 = 0;

    const MAX_TURBULANCE: f64 = 500.0;

    while i < max_iterations {
        let new_mand_value = mandelbrot(last_mand_value, z);
        turbulance = turbulance + (last_mand_value - new_mand_value).re.abs();
        last_mand_value = new_mand_value;
        sum = sum + last_mand_value;

        if turbulance > MAX_TURBULANCE {
            is_in_set = false;
            break;
        }
        i += 1;
    }

    MandelbrotResult {
        is_in_set,
        iterations: i,
        turbulance,
        acceleration: turbulance / (i as f64),
        sum,
    }
}

fn assigned_color(num: u32) -> Rgb<u8> {
    // Map the number to a hue value (0-360 degrees)
    let hue = (num % 360) as f32;

    // Define fixed saturation and lightness values for a pastel color scheme
    let saturation = 0.5; // 50% for a soft color
    let lightness = 0.3; // 80% for high brightness

    // Create an HSL color
    let hsl_color = Hsl::new(hue, saturation, lightness);
    let srgb = Srgb::from_color(hsl_color);
    // println!("srgb is {:?}", srgb);
    Rgb([(srgb.red*255.0) as u8, (srgb.green*255.0) as u8, (srgb.blue*255.0) as u8])
}

fn map_value(value: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    (value - start1) / (stop1 - start1) * (stop2 - start2) + start2
}

fn render_img(width: u32, height: u32, zoom: f64, focal_point: Complex<f64>, filepath: &str) {
    let mut img = RgbImage::new(width, height);

    let aspect_ratio = width as f64 / height as f64;

    let min_x = focal_point.re - (aspect_ratio / zoom);
    let max_x = focal_point.re + (aspect_ratio / zoom);

    let min_y = focal_point.im - (1.0 / zoom);
    let max_y = focal_point.im + (1.0 / zoom);


    println!("Rendering img with focal point {:?} [x_range: ({}, {}), y_range: ({}, {})]", focal_point, min_x, max_x, min_y, max_y);

    let mut x = min_x;
    let mut y = min_y;


    for y in 0..height {
        let progress = (y as f32 / height as f32) * 100.0;

        if (progress % 5.0).abs() <= 0.001 {
            println!("{}%", progress.round());
        }

        for x in 0..width {
            let mapped_x = map_value(x.into(), 0.0, width.into(), min_x, max_x);
            let mapped_y = map_value(y.into(), 0.0, height.into(), min_y, max_y);

            let z = Complex::new(mapped_x, mapped_y);
            let r = run_mandelbrot(z, 300);
            // println!("[{:?}] iterations: {} | acc: {} | ?: {}", z, r.iterations, r.acceleration, r.is_in_set);

            let color = assigned_color((1.0*r.acceleration).round() as u32);
            img.put_pixel(x, y, color);
        }
    }

    img.save(filepath);
}

fn main() {
    let mut zoom = 1.0;
    let mut i = 0;

    let mut frame = 0;
    let end_frame = 2000;

    while frame < end_frame {
        let filepath = format!("./out/out_{}.png", i); // Generate unique file name

        render_img(3024, 1964, zoom, Complex::new(-1.45555, 0.0), &filepath);
        zoom *= 1.5;
        i += 1;

        frame += 1;
    }
}
