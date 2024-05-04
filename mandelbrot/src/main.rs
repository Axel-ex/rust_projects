use mandelbrot::Mandelbrot;

#[cfg(test)]
mod tests;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut mandelbrot = Mandelbrot::new();
    mandelbrot.parse_arguments(args);
    mandelbrot.compute();
    match mandelbrot.write_image() {
        Ok(_) => (),
        Err(e) => {
            println!("{e}");
        }
    }
}
