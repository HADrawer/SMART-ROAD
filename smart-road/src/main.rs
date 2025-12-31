use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::time::{Duration, Instant};
use rand::Rng;

mod intersection;
mod vehicle;

use intersection::*;
use vehicle::{Vehicle, Direction, Route};

/// Each route maps to lane index:
/// -2 = Right turn lane
/// -1 = Right-straight lane
///  0 = Center straight lane
/// +1 = Left-straight lane
/// +2 = Left turn lane
fn lane_for_route(route: Route, dir: Direction) -> i32 {
    match (dir, route) {
        // FROM BOTTOM (moving UP)
        (Direction::Up, Route::Right)   => -2,
        (Direction::Up, Route::Straight)=> -1,
        (Direction::Up, Route::Left)    =>  1,

        // FROM TOP (moving DOWN)
        (Direction::Down, Route::Right)   =>  2,
        (Direction::Down, Route::Straight)=>  1,
        (Direction::Down, Route::Left)    => -1,

        // FROM RIGHT SIDE (moving LEFT)
        (Direction::Left, Route::Right)   =>  1,
        (Direction::Left, Route::Straight)=>  0,
        (Direction::Left, Route::Left)    => -1,

        // FROM LEFT SIDE (moving RIGHT)
        (Direction::Right, Route::Right)   => -1,
        (Direction::Right, Route::Straight)=>  0,
        (Direction::Right, Route::Left)    =>  1,
    }
}

/// =========================================================

fn main() {
    let mut busy = false;
    let mut vehicles: Vec<Vehicle> = vec![];

    let sdl = sdl2::init().unwrap();
    let window = sdl.video().unwrap()
        .window("Smart Intersection", 900, 900)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut events = sdl.event_pump().unwrap();

    let routes = [Route::Right, Route::Straight, Route::Left];
    let mut last_frame = Instant::now();

    'run: loop {
        let dt = last_frame.elapsed().as_secs_f32();
        last_frame = Instant::now();
        let mut rng = rand::thread_rng();

        // INPUT ---------
        for evt in events.poll_iter() {
            match evt {
                Event::Quit{..} |
                Event::KeyDown{ keycode: Some(Keycode::Escape), .. } => break 'run,

                // BOTTOM → UP
                Event::KeyDown{ keycode: Some(Keycode::Up), repeat: false, .. } => {
                    let r = routes[rng.gen_range(0..3)];
                    let lane = lane_for_route(r, Direction::Up);
                    let x = CENTER + (lane * LANE_WIDTH) + LANE_WIDTH/2 + 50;
                    vehicles.push(Vehicle::new(x as f32, 900.0, Direction::Up, r));
                    println!("⬆️ UP     | lane {} | spawn @ {:.0},900 | {:?}", lane, x, r);
                }

                // TOP → DOWN
                Event::KeyDown{ keycode: Some(Keycode::Down), repeat: false, .. } => {
                    let r = routes[rng.gen_range(0..3)];
                    let lane = lane_for_route(r, Direction::Down);
                    let x = CENTER + (lane * LANE_WIDTH) + LANE_WIDTH/2 - 110;
                    vehicles.push(Vehicle::new(x as f32, 0.0, Direction::Down, r));
                    println!("⬇️ DOWN   | lane {} | spawn @ {:.0},0 | {:?}", lane, x, r);
                }

                // RIGHT → LEFT
                Event::KeyDown{ keycode: Some(Keycode::Left), repeat: false, .. } => {
                    let r = routes[rng.gen_range(0..3)];
                    let lane = lane_for_route(r, Direction::Left);
                    let y = CENTER + (lane * LANE_WIDTH) + LANE_WIDTH/2 - 100;
                    vehicles.push(Vehicle::new(900.0, y as f32, Direction::Left, r));
                    println!("⬅️ LEFT   | lane {} | spawn @ 900,{:.0} | {:?}", lane, y, r);
                }

                // LEFT → RIGHT
                Event::KeyDown{ keycode: Some(Keycode::Right), repeat: false, .. } => {
                    let r = routes[rng.gen_range(0..3)];
                    let lane = lane_for_route(r, Direction::Right);
                    let y = CENTER + (lane * LANE_WIDTH) + LANE_WIDTH/2;
                    vehicles.push(Vehicle::new(0.0, y as f32, Direction::Right, r));
                    println!("➡️ RIGHT  | lane {} | spawn @ 0,{:.0} | {:?}", lane, y, r);
                }

                _ => {}
            }
        }

        // UPDATE ---------
        for v in &mut vehicles {
            if v.should_stop() && busy {
                v.update(dt, false);
            } else {
                busy = true;
                v.update(dt, true);
            }

            // FREE INTERSECTION WHEN CAR LEAVES SCREEN
            if v.x < -100.0 || v.x > 1000.0 || v.y < -100.0 || v.y > 1000.0 {
                busy = false;
            }
        }

        // RENDER ---------
        canvas.set_draw_color(Color::RGB(20,20,20));
        canvas.clear();
        draw(&mut canvas);

        for v in &mut vehicles {
            v.draw(&mut canvas);
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}
