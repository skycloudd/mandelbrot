use num::complex::Complex;
use num::complex::ComplexFloat;

fn main() {
    let image_width = 10000;
    let x_left = -2.00;
    let x_right = 0.5;
    let y_bottom = -1.12;
    let y_top = 1.12;
    let image_height = ((y_top - y_bottom) / (x_right - x_left) * image_width as f32) as i32;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        for i in 0..image_width {
            let re = (x_right - x_left) * i as f32 / image_width as f32 + x_left;
            let im = (y_top - y_bottom) * j as f32 / image_height as f32 + y_bottom;

            let c = Complex::new(re, im);

            let colour = compute_colour(c);

            println!("{} {} {}", colour.0, colour.1, colour.2);
        }

        eprint!("{:.3}%\r", j as f32 / (image_height - 1) as f32 * 100.0);
    }
    println!();
}

fn compute_colour(c: Complex<f32>) -> (u8, u8, u8) {
    const MAX_ITERS: i32 = 1000;
    colour_for(compute_iterations(c, MAX_ITERS), MAX_ITERS)
}

fn compute_iterations(c: Complex<f32>, max_iters: i32) -> i32 {
    let mut i = 0;
    let mut z: Complex<f32> = Complex::new(0.0, 0.0);

    while z.abs() <= 2.0 && i < max_iters {
        z = z * z + c;
        i += 1;
    }

    i
}

fn colour_for(i: i32, max_iters: i32) -> (u8, u8, u8) {
    let first = (max_iters as f32 * 0.04) as i32;
    let second = (max_iters as f32 * 0.1) as i32;

    if i < first {
        let blue = i as f32 / first as f32 * 255.0;
        (0, 0, blue as u8)
    } else if i < second {
        let green = i as f32 / second as f32 * 255.0;
        (0, green as u8, 255)
    } else if i < max_iters {
        let red = i as f32 / max_iters as f32 * 255.0;
        (red as u8, 255, 255)
    } else {
        (255, 255, 255)
    }
}
