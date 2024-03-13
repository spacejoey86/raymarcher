mod sdf3d;
mod vec3;

use std::{ops::Div, time::{Duration, Instant}};

use sdf3d::Sdf3d;
use vec3::Vec3;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point, rect::Rect};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const CAMERA_SIZE: f64 = 20.0;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let font = ttf_context.load_font("/usr/share/fonts/liberation/LiberationMono-Regular.ttf", 128)?;

    let window = video_subsystem
        .window("Ray marcher? I hardly know her!", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(100, 149, 237));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let my_sphere = Sdf3d::Sphere { radius: 2.0, colour: Color::GREEN };
    let my_cube = Sdf3d::Cuboid { half_size: Vec3::new(1.0, 2.0, 3.0), colours: [Color::RED; 6] };
    let coloured_cube = Sdf3d::Cuboid{half_size: Vec3::splat(2.0), colours: [Color::BLUE, Color::BLUE, Color::RED, Color::RED, Color::YELLOW, Color::YELLOW]};
    let inner_sdf = Box::new(coloured_cube);

    let mut frame_start_time = Instant::now();

    let mut t = 0;
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
        let ratio = size.1 as f64 / size.0 as f64;
        let sdf = Sdf3d::RotatedSdf{pitch: t as f64 / 10.0, yaw: t as f64 * 0.04, roll: 0.0, inner: inner_sdf.clone()};
        let points: Vec<(i32, i32, Color)> = (0..(size.0 as i32 * size.1 as i32)).into_par_iter().map(|n| -> (i32, i32, Color) {
            let x = n % size.0 as i32;
            let y = n / size.0 as i32;

            let pos = Vec3::new(
                x as f64 / size.0 as f64 * CAMERA_SIZE - (CAMERA_SIZE / 2.0),
                y as f64 / size.1 as f64 * CAMERA_SIZE * ratio - (CAMERA_SIZE * ratio * 0.5),
                -50.0,
            );
            let ray_dir = Vec3::new(0.0, 0.0, 1.0);

            if let Some((colour, collision_point)) = Sdf3d::sphere_trace(
                &sdf,
                pos,
                ray_dir,
                0.01,
            ) {
                let normal = Sdf3d::estimate_normal(&sdf, collision_point, 0.1);
                return (x, y, lighting(colour, ray_dir, normal));
                // canvas.set_draw_color();
            } else {
                return (x, y, sdl2::pixels::Color::RGB(100, 149, 237))
            }
        }).collect();

        for (x, y, colour) in points {
            canvas.set_draw_color(colour);
            canvas.draw_point(Point::new(x, y))?;
        }

        let current_time = Instant::now();
        let frame_rate = 1.0 / (current_time - frame_start_time).as_secs_f64();

        let surface = font
            .render(&format!("{:.1}", frame_rate))
            .blended(Color::BLACK)
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())?;
        canvas.copy(&texture, None, Rect::new(0, 0, 50, 50))?;
        frame_start_time = current_time;

        canvas.present();
        t = t + 1;
    }

    return Ok(());
}

fn lighting(object_colour: Color, ray_direction: Vec3, normal: Vec3) -> Color {
    let light_colour = Color::WHITE;
    let ambient = multiply_colour_float(multiply_colours(object_colour, light_colour), 1.0);
    let diffuse = multiply_colour_float(multiply_colours(object_colour, light_colour), normal.dot(&(-ray_direction)));

    return add_colour(ambient, diffuse);
}

fn multiply_colours(lhs: Color, rhs: Color) -> Color {
    return Color::RGB((lhs.r as f64 * rhs.r as f64 / u8::MAX as f64) as u8,
        (lhs.g as f64 * rhs.g as f64 / u8::MAX as f64) as u8,
        (lhs.b as f64 * rhs.b as f64 / u8::MAX as f64) as u8);
}

fn multiply_colour_float(lhs: Color, rhs: f64) -> Color {
    return Color::RGB((lhs.r as f64 * rhs) as u8,
        (lhs.g as f64 * rhs) as u8,
        (lhs.b as f64 * rhs) as u8);
}

fn add_colour(lhs: Color, rhs: Color) -> Color {
    return Color::RGB(((lhs.r as u16 + rhs.r as u16) / 2) as u8,
    ((lhs.g as u16 + rhs.g as u16) / 2) as u8,
    ((lhs.b as u16 + rhs.b as u16) / 2) as u8);
}
