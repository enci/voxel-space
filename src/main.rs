extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::time::{Instant};
use rand::prelude::*;

const WIDTH: usize = 360;
const HEIGHT: usize = 240;


fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_secs_f64(1.0 / 1000.0)));
    //window.limit_update_rate(Some(std::time::Duration::from_secs(0)));

    let mut last_frame = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            let val : u32 = random();
            *i = val;
            //*i = 0xFFFFFFFF;
        }        

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        let current_frame = Instant::now(); 
        let delta_time = current_frame - last_frame;
        last_frame = current_frame;
        let fps = 1.0 / delta_time.as_secs_f64();
        let title = format!("Ruster - FPS {}", fps);    
        window.set_title(&title);
    }
}
