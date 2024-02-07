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

impl<T: Copy + std::ops::Mul<Output = T>> ops::Mul<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    fn mul(self, another: Complex<T>) -> Complex<T> {
        Complex {
            Imaginary: another.Imaginary * self.Imaginary,
            Real: another.Real * self.Real,
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

fn main() {
    println!("hello");
    let a = Complex::new(0.2, 0.3);
    println!("{:?}", run_mandelbrot(a, 100));
}
