use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};
mod intersection;
mod vehicle;

use intersection::draw as draw_intersection;
use vehicle::Vehicle;




fn main() {
    let car = Vehicle { x: 435.0, y: 800.0 };
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let window = video
        .window("Smart Intersection", 900, 900)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut event_pump = sdl.event_pump().unwrap();

    let mut last_frame = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            car.draw(&mut canvas);

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let now = Instant::now();
        let _dt = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;

        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        draw_intersection(&mut canvas);

        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }
}
