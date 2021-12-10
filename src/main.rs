extern crate minifb;

use image::io::Reader as ImageReader;
use image::GenericImageView;
use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

fn convert_to_u32(pixel: image::Rgba<u8>) -> u32 {
    ((pixel[3] as u32) << 24)
        | ((pixel[0] as u32) << 16)
        | ((pixel[1] as u32) << 8)
        | (pixel[2] as u32)
}

fn main() {
    let img = ImageReader::open("assets/crab-galapagos-islands.jpg");
    let img = img.unwrap_or_else(|e| panic!("{}", e)).decode();
    let img = img.unwrap_or_else(|e| panic!("{}", e));

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    let (width, height) = img.dimensions();
    let width = width as usize;
    let height = height as usize;

    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut window =
        Window::new("Window", width, height, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_secs_f64(1.0 / 1000.0)));
    // //window.limit_update_rate(Some(std::time::Duration::from_secs(0)));

    let mut last_frame = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for x in 0..width {
            for y in 0..height {
                let xi = x as u32;
                let yi = y as u32;
                let pixel = img.get_pixel(xi, yi);
                let pixel = convert_to_u32(pixel);
                let i = x + y * width;
                buffer[i] = pixel;
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, width, height).unwrap();

        let current_frame = Instant::now();
        let delta_time = current_frame - last_frame;
        last_frame = current_frame;
        let fps = 1.0 / delta_time.as_secs_f64();
        let title = format!("Ruster - FPS {}", fps);
        window.set_title(&title);
    }
}
