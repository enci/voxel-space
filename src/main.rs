extern crate minifb;

use image::io::Reader as ImageReader;
use image::GenericImageView;
use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

struct Point {
    x: f32,
    y: f32,
}

fn draw_vertical_line(
    x: u32,
    ytop: u32,
    ybottom: u32,
    col: u32,
    stride: u32,
    buffer: &mut Vec<u32>,
) {
    for y in ytop..ybottom {
        let i = x + y * stride;
        let i = i as usize;
        buffer[i] = col;
    }
}

fn render(
    p: Point,
    phi: f32,
    height: u32,
    horizon: u32,
    scale_height: u32,
    distance: u32,
    screen_width: u32,
    screen_height: u32,
    heightmap: &mut image::DynamicImage,
    colormap: &mut image::DynamicImage,
    buffer: &mut Vec<u32>,
) {
    // precalculate viewing angle parameters
    let sinphi = f32::sin(phi);
    let cosphi = f32::cos(phi);

    // Draw from back to the front (high z coordinate to low z coordinate)
    let mut z = distance as i32;
    let end = 1 as i32;
    let step = -2 as i32;

    while z >= end
    {
        let zf = z as f32;
        // Find line on map. This calculation corresponds to a field of view of 90°
        let mut pleft = Point {
            x: (-cosphi * zf - sinphi * zf) + p.x,
            y: (sinphi * zf - cosphi * zf) + p.y,
        };
        let pright = Point {
            x: (cosphi * zf - sinphi * zf) + p.x,
            y: (-sinphi * zf - cosphi * zf) + p.y,
        };

        // segment the line
        let dx = (pright.x - pleft.x) / screen_width as f32;
        let dy = (pright.y - pleft.y) / screen_width as f32;

        // Raster line and draw a vertical line for each segment
        for i in 0..screen_width {
            let mut height_on_screen = height as i32;
            let h = heightmap.get_pixel(pleft.x as u32, pleft.y as u32);
            let h = h[1] as i32;
            height_on_screen -= h;
            height_on_screen *= scale_height as i32;
            height_on_screen /= z;
            height_on_screen += horizon as i32;

            let color = colormap.get_pixel(pleft.x as u32, pleft.y as u32);
            let color = convert_to_u32(color);
            
            if height_on_screen < 0 {
                height_on_screen = 0;
            }

            draw_vertical_line(
                i,
                height_on_screen as u32,
                screen_height,
                color,
                screen_width,
                buffer,
            );

            pleft.x += dx;
            pleft.y += dy;
        }

        z += step;
    }
}

fn clear(color: u32, buffer: &mut Vec<u32>) {
    for i in buffer.iter_mut() {
        *i = color;
    }
}

fn convert_to_u32(pixel: image::Rgba<u8>) -> u32 {
    ((pixel[3] as u32) << 24)
        | ((pixel[0] as u32) << 16)
        | ((pixel[1] as u32) << 8)
        | (pixel[2] as u32)
}

fn main() {
    let screen_width = 360;
    let screen_height = 240;

    let heightmap = ImageReader::open("assets/D1.png");
    let heightmap = heightmap.unwrap_or_else(|e| panic!("{}", e)).decode();
    let heightmap = heightmap.unwrap_or_else(|e| panic!("{}", e));
    let mut height_map = heightmap;

    let colormap = ImageReader::open("assets/C1W.png");
    let colormap = colormap.unwrap_or_else(|e| panic!("{}", e)).decode();
    let colormap = colormap.unwrap_or_else(|e| panic!("{}", e));
    let mut color_map = colormap;

    // The dimensions method returns the images width and height.
    //println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    //println!("{:?}", img.color());

    //let (width, height) = img.dimensions();
    //let width = width as usize;
    //let height = height as usize;

    let mut buffer: Vec<u32> = vec![0; screen_width * screen_height];
    let mut window = Window::new(
        "Window",
        screen_width,
        screen_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_secs_f64(1.0 / 1000.0)));
    // //window.limit_update_rate(Some(std::time::Duration::from_secs(0)));

    let mut last_frame = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        clear(0x0035FF, &mut buffer);
        render(
            Point { x: 0.0, y: 0.0 }, // Point
            0.0,                      // Angle
            50,                        // Height
            120,                      // Horizon
            120,                      // Scale height
            600,                      // Distance
            screen_width as u32,
            screen_height as u32,
            &mut height_map,
            &mut color_map,
            &mut buffer,
        );

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, screen_width, screen_height)
            .unwrap();

        let current_frame = Instant::now();
        let delta_time = current_frame - last_frame;
        last_frame = current_frame;
        let fps = 1.0 / delta_time.as_secs_f64();
        let title = format!("Framebuffer - FPS {}", fps);
        window.set_title(&title);
    }
}
