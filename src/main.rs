mod cuboid;
mod direction;
mod sdf3d;
mod sphere;
mod vec3;

use sdf3d::Sdf3d;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point};
use sphere::Sphere;
use vec3::Vec3;

const CAMERA_SIZE: f64 = 20.0;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Ray marcher? I hardly know her!", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(100, 149, 237));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let sdf = Sphere::new(2.0, Color::GREEN);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let size = canvas.output_size()?;
        let ratio = (size.1 as f64 / size.0 as f64);
        for x in 0..size.0 as i32 {
            for y in 0..size.1 as i32 {
                let pos = Vec3::new(
                    x as f64 / size.0 as f64 * CAMERA_SIZE - (CAMERA_SIZE / 2.0),
                    y as f64 / size.1 as f64 * CAMERA_SIZE * ratio - (CAMERA_SIZE * ratio * 0.5),
                    -50.0,
                );
                // println!("({}, {}) is pos {}", x, y, pos);
                canvas.set_draw_color(<dyn Sdf3d>::sphere_trace(
                    &sdf,
                    pos,
                    Vec3::new(0.0, 0.0, 1.0),
                    0.01,
                ));

                canvas.draw_point(Point::new(x, y))?
            }
        }
        canvas.present();
    }

    return Ok(());
}
