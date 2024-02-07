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
   b.Real > 50.0 || b.Real < -50.0 || b.Imaginary > 50.0 || b.Imaginary < -50.0
//    b.Real.is_infinite() || b.Imaginary.is_infinite();
}

fn main() {
    println!("hello");
    let a = Complex::new(0.02, 0.3);
    println!("{:?}", run_mandelbrot(a, 100));

    let mut x = -2.0;
    let mut y = -2.0;
    let STEP = 0.01;

    while y < 2.0 {
        while x < 2.0 {
            let z = Complex::new(y, x);
            // print!("{:?}", run_mandelbrot(z, 100));
            if is_in_mandelbrot_set(z) {
                print!("o");
            } else {
                print!("-");
            }
            // print!("{:?}", is_in_mandelbrot_set(z));
            x += STEP;
        }
        x = -2.0;
        y += STEP;
        println!("  !")
    }


    // for x in MIN_XY.Real..MAX_XY.Real {
    //     for y in MIN_XY.Imaginary..MAX_XY.Imaginary {
    //         // current_z.Imaginary += INCR;
    //         let z = Complex::new(y, x);
    //         print!("{:?}", run_mandelbrot(z, 100));
    //     }
    //     println!("          #")
    // }
    // let ROWS = (RANGE_X) * RESOLUTION;
    // let COLS = (RANGE_Y) * RESOLUTION;

}
