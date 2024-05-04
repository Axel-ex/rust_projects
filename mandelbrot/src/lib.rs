use crossbeam;
use image::{png::PNGEncoder, ColorType};
use num::Complex;
use std::io::Write;
use std::{fs::File, str::FromStr};

pub struct Mandelbrot {
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    pixels: Vec<u8>,
    file_name: String,
}

impl Mandelbrot {
    pub fn new() -> Mandelbrot {
        Mandelbrot {
            bounds: (0, 0),
            upper_left: Complex { re: 0., im: 0. },
            lower_right: Complex { re: 0., im: 0. },
            pixels: vec![0; 0],
            file_name: String::new(),
        }
    }

    pub fn parse_arguments(&mut self, args: Vec<String>) {
        if args.len() != 5 {
            writeln!(
                std::io::stderr(),
                "Usage: mandelbrot <file> <pixels> <upper_left> <lower_right>"
            )
            .unwrap();
            writeln!(
                std::io::stderr(),
                "Example: {} mandelbrot.png 1000x750 -1.20,0.35 -1,0.2",
                args[0]
            )
            .unwrap();
            std::process::exit(1);
        }

        self.bounds = Mandelbrot::parse_pair(&args[2], 'x').expect("Error parsing image dimension");
        self.upper_left = Mandelbrot::parse_complex_pair(&args[3])
            .expect("Error parsing upper left corner point");
        self.lower_right = Mandelbrot::parse_complex_pair(&args[4])
            .expect("Error parsing lower rigt corner point");
        self.pixels = vec![0; self.bounds.0 * self.bounds.1];
        self.file_name = String::from(&args[1]);
    }

    pub fn compute(&mut self) {
        let threads = 8;
        let rows_per_band = self.bounds.1 / threads + 1;

        {
            // divide the pixel buffer into non overlapping slices of size nb of row * length of a row
            let bands: Vec<&mut [u8]> = self
                .pixels
                .chunks_mut(rows_per_band * self.bounds.0)
                .collect();
            crossbeam::scope(|spawner| {
                for (i, band) in bands.into_iter().enumerate() {
                    let top = rows_per_band * i;
                    let height = band.len() / self.bounds.0;
                    let band_bounds = (self.bounds.0, height);
                    let band_upper_left = Mandelbrot::pixel_to_point(
                        self.bounds,
                        (0, top),
                        self.upper_left,
                        self.lower_right,
                    );
                    let band_lower_right = Mandelbrot::pixel_to_point(
                        self.bounds,
                        (self.bounds.0, top + height),
                        self.upper_left,
                        self.lower_right,
                    );
                    spawner.spawn(move || {
                        Mandelbrot::render(band, band_bounds, band_upper_left, band_lower_right);
                    });
                }
            });
        }
    }

    pub fn write_image(&self) -> Result<(), std::io::Error> {
        let output = File::create(&self.file_name)?;

        let encoder = PNGEncoder::new(output);
        encoder.encode(
            &self.pixels,
            self.bounds.0 as u32,
            self.bounds.1 as u32,
            ColorType::Gray(8),
        )?;

        Ok(())
    }

    /// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
    /// iterations to decide.
    ///
    /// If `c` is not a member, return `Some(i)`, where `i` is the number of
    /// iterations it took for `c` to leave the circle of radius two centered on the
    /// origin. If `c` seems to be a member (more precisely, if we reached the
    /// iteration limit without being able to prove that `c` is not a member),
    /// return `None`.
    fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
        let mut z = Complex { re: 0.0, im: 0.0 };
        for i in 0..limit {
            z = z * z + c;
            if z.norm_sqr() > 4.0 {
                return Some(i);
            }
        }
        None
    }

    fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)>
    where
        T::Err: std::fmt::Debug,
    {
        match s.find(separator) {
            Some(index) => {
                let (left, right) = s.split_at(index);
                match (left.parse(), right[1..].parse()) {
                    (Ok(l), Ok(r)) => Some((l, r)),
                    _ => None,
                }
            }
            None => None,
        }
    }

    fn parse_complex_pair(s: &str) -> Option<Complex<f64>> {
        match Mandelbrot::parse_pair(s, ',') {
            Some((re, im)) => Some(Complex { re, im }),
            None => None,
        }
    }

    fn pixel_to_point(
        bounds: (usize, usize),
        pixel: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
    ) -> Complex<f64> {
        let (width, height) = (
            lower_right.re - upper_left.re,
            upper_left.im - lower_right.im,
        );

        Complex {
            re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
            im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
        }
    }

    fn render(
        pixels: &mut [u8],
        bound: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
    ) {
        assert!(pixels.len() == bound.0 * bound.1);

        for row in 0..bound.1 {
            for column in 0..bound.0 {
                let point =
                    Mandelbrot::pixel_to_point(bound, (column, row), upper_left, lower_right);
                pixels[row * bound.0 + column] = match Mandelbrot::escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8,
                };
            }
        }
    }
}
