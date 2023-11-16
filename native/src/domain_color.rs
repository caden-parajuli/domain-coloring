use hsluv;
pub use num::complex::Complex;
use num::complex::ComplexFloat;
use std::f64::consts::PI as PI64;

// Only need this if you expose parsing to Dart
// #[frb(mirror(Complex))]
// #[repr(C)]
// pub struct _Complex<T> {
//     pub re: T,
//     pub im: T,
// }

struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(dead_code)]
impl Rgb {
    fn from_linear(r: f64, g: f64, b: f64) -> Self {
        Rgb {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
        }
    }
}

pub struct DCOptions {
    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
}

pub type ComplexFunction = dyn Fn(Complex<f32>) -> Complex<f32>;

#[allow(unused_variables)]
fn parse(fun_str: &str) -> Result<Box<ComplexFunction>, String> {
    // Parse the complex function
    Ok(Box::new(|z| z))
}

fn good_arg(z: Complex<f32>) -> f64 {
    if z.arg() >= 0.0 {
        (z.arg() as f64) / PI64 * 180.0
    } else {
        z.arg() as f64 * 180.0 / PI64 + 360.0
    }
}

fn color_bytes(fun_val: Complex<f32>) -> Rgb {
    //! returns RGB color corresponding to function value

    let lightness: f64 = (100.0 * fun_val.abs() / (fun_val.abs() + 1.0)) as f64;
    let saturation: f64 = 100.0;
    let hue: f64 = good_arg(fun_val);
    let linear = hsluv::hpluv_to_rgb(hue, saturation, lightness);
    Rgb::from_linear(linear.0, linear.1, linear.2)
}

pub fn color_bmp(width: usize, height: usize, fun_str: &str, options: DCOptions) -> Vec<u8> {
    // Parse the function
    let function = parse(fun_str).unwrap();

    let mut header = vec![
        b'B', b'M', 0, 0, 0, 0, // File size, to be updated later
        0, 0, 0, 0, // Reserved, 0
        0x36, 0, 0, 0, // Pixel buffer offset
        0x28, 0, 0, 0, // (DIB) Header size
    ];
    header.extend_from_slice(&(width as u32).to_le_bytes());
    header.extend_from_slice(&(height as u32).to_le_bytes());
    header.extend_from_slice(&[
        1, 0, // 1 Color plane
        24, 0, // 24 bits (3 bytes) per pixel
        0, 0, 0, 0, // No compression
        0, // Image size, ignored for uncompressed so we set to 0
        0, 0, 0, 0, // Horizontal pixels per meter, irrelevant
        0, 0, 0, 0, // Vertical pixels per meter, irrelevant
        0, 0, 0, 0, // Palette size, irrelevant
        0, 0, 0, 0, // Number of "important" colors
    ]);

    // BMP Header size
    let header_size = header.len();
    let buffer_size = 3 * height * (width + 1); // +1 for row padding?

    // Update header with file size
    let file_size = ((header_size + buffer_size) as u32).to_le_bytes();
    header[2] = file_size[0];
    header[3] = file_size[1];
    header[4] = file_size[2];
    header[5] = file_size[3];

    // BMP buffer
    let mut buffer: Vec<u8> = vec![0; header_size + buffer_size];

    // Set the header
    buffer[0..header_size].copy_from_slice(&header);

    // Set the pixels: Domain Coloring
    let mut x: f32;
    let mut y: f32;
    let x_step: f32 = (options.xmax - options.xmin) / (width as f32);
    let y_step: f32 = (options.ymax - options.xmin) / (height as f32);
    let mut rgb: Rgb;
    for y_px in 0..height {
        for x_px in 0..(width - 2) {
            x = options.xmin + x_px as f32 * x_step; // Not sure if this should use width - 1
            y = options.ymin + y_px as f32 * y_step; // Not sure if this should use height - 1
            rgb = color_bytes(function(Complex::new(x, y)));
            // println!("{:?}", rgb);
            // Consider get_unchecked with a debug assertion
            buffer[header_size + y_px * 3 * width + 3 * x_px] = rgb.b;
            buffer[header_size + y_px * 3 * width + 3 * x_px + 1] = rgb.g;
            buffer[header_size + y_px * 3 * width + 3 * x_px + 2] = rgb.r;
        }
    }
    buffer
}
