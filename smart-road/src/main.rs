use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::time::{Duration, Instant};
use rand::Rng;

mod intersection;
mod vehicle;

use intersection::*;
use vehicle::{Vehicle, Direction, Route};

fn main() {
    let mut busy = false;
    let mut vehicles: Vec<Vehicle> = vec![];

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("Smart Intersection", 900, 900).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut events = sdl.event_pump().unwrap();

    let mut last = Instant::now();
    let routes = [Route::Right, Route::Straight, Route::Left];

    'run: loop {
        let dt = last.elapsed().as_secs_f32();
        last = Instant::now();
        let mut rng = rand::thread_rng();

        // ------- EVENT INPUT -------
        for ev in events.poll_iter() {
            match ev {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'run,

                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    let r = routes[rng.gen_range(0..3)];
                    let lane = match r { Route::Left => -LANE_WIDTH, Route::Straight => 0, Route::Right => LANE_WIDTH };
                    vehicles.push(Vehicle::new((CENTER + lane) as f32, 900.0, Direction::Up, r));
                }

                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    let r = routes[rng.gen_range(0..3)];
                    let lane = match r { Route::Left => LANE_WIDTH, Route::Straight => 0, Route::Right => -LANE_WIDTH };
                    vehicles.push(Vehicle::new((CENTER + lane) as f32, 0.0, Direction::Down, r));
                }

                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    let r = routes[rng.gen_range(0..3)];
                    let lane = match r { Route::Left => LANE_WIDTH, Route::Straight => 0, Route::Right => -LANE_WIDTH };
                    vehicles.push(Vehicle::new(900.0, (CENTER + lane) as f32, Direction::Left, r));
                }

                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    let r = routes[rng.gen_range(0..3)];
                    let lane = match r { Route::Left => -LANE_WIDTH, Route::Straight => 0, Route::Right => LANE_WIDTH };
                    vehicles.push(Vehicle::new(0.0, (CENTER + lane) as f32, Direction::Right, r));
                }

                _ => {}
            }
        }

        // ------- UPDATE VEHICLES -------
        for v in &mut vehicles {
            if v.should_stop() && busy {
                v.update(dt, false);
            } else {
                busy = true;
                v.update(dt, true);
            }

            if v.x < -100.0 || v.x > 1000.0 || v.y < -100.0 || v.y > 1000.0 {
                busy = false;
            }
        }

        // ------- RENDER -------
        canvas.set_draw_color(Color::RGB(20, 20, 20));
        canvas.clear();
        draw(&mut canvas);

        for v in &vehicles {
            v.draw(&mut canvas);
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}
