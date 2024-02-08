extern crate image;

use image::{ImageBuffer, Rgb, RgbImage};
use std::ops;

#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug)]
struct Complex<T> {
    Real: T,
    Imaginary: T,
}

impl<T: Copy> Complex<T> {
    pub fn new(imaginary: T, real: T) -> Complex<T>{
        Complex {
            Real: real,
            Imaginary: imaginary
        }
    }
}

impl<T: Copy + std::ops::Add<Output = T>> ops::Add<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    fn add(self, another: Complex<T>) -> Complex<T> {
        Complex {
            Imaginary: another.Imaginary + self.Imaginary,
            Real: another.Real + self.Real,
        }
    }
}

impl<T: Copy + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Sub<Output = T>> ops::Mul<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    fn mul(self, another: Complex<T>) -> Complex<T> {
        let real = (self.Real * another.Real) - (self.Imaginary * another.Imaginary);
        let im = (self.Real * another.Imaginary) + (self.Imaginary * another.Real);

        Complex {
            Imaginary: im,
            Real: real,
        }
    }
}

impl <T: Copy + std::ops::Sub<Output = T>> ops::Sub<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    fn sub(self, another: Complex<T>) -> Complex<T> {
        Complex {
            Imaginary: self.Imaginary - another.Imaginary,
            Real: self.Real - another.Real,
        }
    }
}


fn mandelbrot(x: Complex<f64>, z: Complex<f64>) -> Complex<f64>{
    x*x + z
}

fn run_mandelbrot(z: Complex<f64>, iterations: u8) -> Complex<f64> {
    let mut sum = Complex::new(0.0, 0.0);
    let mut last_mand_value = Complex::new(0.0, 0.0);
    for i in 0..iterations {
        last_mand_value = mandelbrot(last_mand_value, z);
        sum = sum + last_mand_value;
    }
    sum
}

fn is_in_mandelbrot_set(z: Complex<f64>) -> bool {
   let b = run_mandelbrot(z, 255);
   b.Real > 10.0 || b.Real < -10.0 || b.Imaginary > 10.0 || b.Imaginary < -10.0
//    b.Real.is_infinite() || b.Imaginary.is_infinite();
}

// fn map_value(value: u32, start1: u32, stop1: u32, start2: u32, stop2: u32) -> u32 {
//     (value - start1) / (stop1 - start1) * (stop2 - start2) + start2
// }
fn map_value(value: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    (value - start1) / (stop1 - start1) * (stop2 - start2) + start2
}


fn main() {
    // let MIN_X = -2.0;
    // let MIN_Y = -2.0;

    // let MAX_X = 2.0;
    // let MAX_Y = 2.0;

    let MIN_X = -2.0;
    let MIN_Y = -2.0;

    let MAX_X = 2.0;
    let MAX_Y = 2.0;

    let mut x = MIN_X;
    let mut y = MIN_Y;

    const X_STEP: f64 = 0.005;
    const Y_STEP: f64 = 0.02;

    let width = (MAX_X - MIN_X) / X_STEP;
    let height = (MAX_Y - MIN_Y) / Y_STEP;

    println!("image width is {}", width);
    println!("image height is {}", height);

    // let mut img = ImageBuffer::new(width as u32, height as u32);
    let mut img = RgbImage::new(width as u32, height as u32);
    
    
    while y < MAX_Y {
        while x < MAX_X {
            let z = Complex::new(y, x);

            if is_in_mandelbrot_set(z) {
                // print!("x");
                let img_x = map_value(x, MIN_X, MAX_X, 0.0, width) as u32;
                let img_y = map_value(y, MIN_Y, MAX_Y, 0.0, height) as u32;
                println!("{} {}", img_x, img_y);
                img.put_pixel(img_x, img_y, Rgb([255, 255, 255]));
                // img.put_pixel(map_value(x, 0, width, MIN_X, MAX_X), Rgb([255, 255, 255]));
            } else {
                // print!(" ");
                // img.put_pixel(map_value(x, 0, width, MIN_X, MAX_X), Rgb([0, 0, 0]));
            }
            x += X_STEP;
        }
        x = MIN_X;
        y += Y_STEP;
        // println!("");
    }
    
    img.save("./out.png");
    
}
