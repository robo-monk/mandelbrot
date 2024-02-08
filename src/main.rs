extern crate num;
extern crate image;
use image::{ImageBuffer, Rgb, RgbImage};
use num::complex::Complex;
use num::complex::ComplexFloat;
use std::ops;
use palette::{Hsl, Srgb, FromColor};


// #[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug)]
// struct Complex<T> {
//     Real: T,
//     Imaginary: T,
// }

// impl<T: Copy> Complex<T> {
//     pub fn new(imaginary: T, real: T) -> Complex<T>{
//         Complex {
//             Real: real,
//             Imaginary: imaginary
//         }
//     }

//     pub fn abs(self) -> Complex<T> {
//         Complex {
//             Real: self.re.abs(),
//             Imaginary: self.im.abs(),
//         }
//     }
// }

// impl<T: Copy + std::ops::Add<Output = T>> ops::Add<Complex<T>> for Complex<T> {
//     type Output = Complex<T>;

//     fn add(self, another: Complex<T>) -> Complex<T> {
//         Complex {
//             Imaginary: another.im + self.im,
//             Real: another.re + self.re,
//         }
//     }
// }

// impl<T: Copy + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Sub<Output = T>> ops::Mul<Complex<T>> for Complex<T> {
//     type Output = Complex<T>;

//     fn mul(self, another: Complex<T>) -> Complex<T> {
//         let real = (self.re * another.re) - (self.im * another.im);
//         let im = (self.re * another.im) + (self.im * another.re);

//         Complex {
//             Imaginary: im,
//             Real: real,
//         }
//     }
// }

// impl <T: Copy + std::ops::Sub<Output = T>> ops::Sub<Complex<T>> for Complex<T> {
//     type Output = Complex<T>;

//     fn sub(self, another: Complex<T>) -> Complex<T> {
//         Complex {
//             Imaginary: self.im - another.im,
//             Real: self.re - another.re,
//         }
//     }
// }

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

    const MAX_TURBULANCE: f64 = 1000.0;

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

// fn is_in_mandelbrot_set(z: Complex<f64>) -> bool {
//    let b = run_mandelbrot(z, 20);
//    b.re > 10.0 || b.re < -10.0 || b.im > 10.0 || b.im < -10.0
// }

// fn map_value(value: u32, start1: u32, stop1: u32, start2: u32, stop2: u32) -> u32 {
//     (value - start1) / (stop1 - start1) * (stop2 - start2) + start2
// }
fn assigned_color(num: u32) -> Rgb<u8> {
    // Map the number to a hue value (0-360 degrees)
    let hue = (num % 360) as f32;

    // Define fixed saturation and lightness values for a pastel color scheme
    let saturation = 0.5; // 50% for a soft color
    let lightness = 0.8; // 80% for high brightness

    // Create an HSL color
    let hsl_color = Hsl::new(hue, saturation, lightness);
    let srgb = Srgb::from_color(hsl_color);
    println!("srgb is {:?}", srgb);
    Rgb([(srgb.red*255.0) as u8, (srgb.green*255.0) as u8, (srgb.blue*255.0) as u8])
}

fn map_value(value: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    (value - start1) / (stop1 - start1) * (stop2 - start2) + start2
}

fn main() {

    let MIN_X = -2.0;
    let MIN_Y = -2.0;

    let MAX_X = 1.0;
    let MAX_Y = 2.0;

    let mut x = MIN_X;
    let mut y = MIN_Y;

    const X_STEP: f64 = 0.01;
    const Y_STEP: f64 = 0.01;

    let width = (MAX_X - MIN_X) / X_STEP;
    let height = (MAX_Y - MIN_Y) / Y_STEP;

    println!("image width is {}", width);
    println!("image height is {}", height);

    let mut img = RgbImage::new(width as u32, height as u32);

    while y < MAX_Y {
        while x < MAX_X {
            let z = Complex::new(x, y);

            let r = run_mandelbrot(z, 20);
            // println!("iterations: {} | acc: {} | ?: {}", r.iterations, r.acceleration, r.is_in_set);

            // if r.is_in_set {
                let img_x = map_value(x, MIN_X, MAX_X, 0.0, width) as u32;
                let img_y = map_value(y, MIN_Y, MAX_Y, 0.0, height) as u32;
                let color = assigned_color(r.acceleration as u32);
                // println!("color {:?}", color);
                img.put_pixel(img_x, img_y, color);
            // }
            x += X_STEP;
        }
        x = MIN_X;
        y += Y_STEP;
    }
    
    img.save("./out.png");
}
