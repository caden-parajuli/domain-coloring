use crate::domain_color;
use crate::domain_color::DCOptions;
use std::io::Write;

use tempfile::Builder;

#[test]
fn color_bmp_test() {
    let mut img_file = Builder::new()
        .suffix(".bmp")
        .tempfile()
        .expect("Tempfile could not be created");

    let bmp = domain_color::color_bmp(
        1000,
        1000,
        "z",
        DCOptions {
            xmin: -5.0,
            xmax: 5.0,
            ymin: -5.0,
            ymax: 5.0,
        },
    );

    img_file
        .write_all(&bmp)
        .expect("Could not write image data to file");

    println!("{:?}", img_file.path());
    img_file.keep().expect("Could not save file");
}
