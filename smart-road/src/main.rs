use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::time::{Duration, Instant};
use rand::Rng;

mod intersection;
mod vehicle;
mod stats;

use stats::{Stats, show_stats_window};
use intersection::*;
use vehicle::{Vehicle, Direction, Route};

/// -2 = Far Right Turn, -1 = Right-Straight, 0 = Straight,
/// +1 = Left-Straight, +2 = Far Left Turn
fn lane_for_route(route: Route, dir: Direction) -> i32 {
    match (dir, route) {
        (Direction::Up,    Route::Right)   => -2,
        (Direction::Up,    Route::Straight)=> -1,
        (Direction::Up,    Route::Left)    =>  1,

        (Direction::Down,  Route::Right)   =>  2,
        (Direction::Down,  Route::Straight)=>  1,
        (Direction::Down,  Route::Left)    => -1,

        (Direction::Left,  Route::Right)   =>  1,
        (Direction::Left,  Route::Straight)=>  0,
        (Direction::Left,  Route::Left)    => -1,

        (Direction::Right, Route::Right)   => -1,
        (Direction::Right, Route::Straight)=>  0,
        (Direction::Right, Route::Left)    =>  1,
    }
}

/// Prevent stacking on spawn
fn is_spawn_blocked(vehicles: &[Vehicle], x: f32, y: f32) -> bool {
    vehicles
        .iter()
        .any(|v| (v.x - x).abs() < 40.0 && (v.y - y).abs() < 40.0)
}

/// Unified spawner
fn spawn_vehicle(vehicles: &mut Vec<Vehicle>, stats: &mut Stats, r: Route, dir: Direction) {
    let lane = lane_for_route(r, dir);
    let offset = lane * LANE_WIDTH + LANE_WIDTH/2;

    let (x, y) = match dir {
        Direction::Up => ((CENTER + offset) as f32, 900.0),
        Direction::Down => ((CENTER + offset) as f32, 0.0),
        Direction::Left => (900.0, (CENTER + offset) as f32),
        Direction::Right => (0.0, (CENTER + offset) as f32),
    };

    if is_spawn_blocked(vehicles, x, y) {
        println!("🚫 Spawn blocked — lane occupied!");
        return;
    }

    println!("🚗 {:?} from {:?} at ({:.0},{:.0})", r, dir, x, y);
    vehicles.push(Vehicle::new(x, y, dir, r));

    // 📊 Stats update
    stats.total_vehicles += 1;
    match dir {
        Direction::Up => stats.up += 1,
        Direction::Down => stats.down += 1,
        Direction::Left => stats.left += 1,
        Direction::Right => stats.right += 1,
    }
    match r {
        Route::Left => stats.left_turn += 1,
        Route::Straight => stats.straight += 1,
        Route::Right => stats.right_turn += 1,
    }
}

//// =========================================================

fn main() {

    let mut vehicles: Vec<Vehicle> = vec![];
    let mut stats = Stats::new();

    let sdl = sdl2::init().unwrap();
    let window = sdl.video().unwrap()
        .window("Smart Intersection", 900, 900)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut events = sdl.event_pump().unwrap();

    let routes = [Route::Right, Route::Straight, Route::Left];
    let mut rng = rand::thread_rng();

    let mut last_frame = Instant::now();
    let mut last_spawn = Instant::now();
    let mut auto_spawn = false;
    let mut intersection_busy = false;

    'run: loop {
        let dt = last_frame.elapsed().as_secs_f32();
        last_frame = Instant::now();
        stats.runtime += dt;

        // INPUT ------------------------------
        for evt in events.poll_iter() {
            match evt {
                Event::Quit{..} => break 'run,

                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    show_stats_window(&stats);
                    break 'run;
                }

                Event::KeyDown { keycode: Some(Keycode::R), repeat: false, .. } => {
                    auto_spawn = !auto_spawn;
                    println!("🔁 Auto-spawn {}", if auto_spawn {"ON"} else {"OFF"});
                }

                Event::KeyDown{ keycode: Some(Keycode::Up), repeat: false, .. } => {
                    spawn_vehicle(&mut vehicles, &mut stats, routes[rng.gen_range(0..3)], Direction::Up);
                }
                Event::KeyDown{ keycode: Some(Keycode::Down), repeat: false, .. } => {
                    spawn_vehicle(&mut vehicles, &mut stats, routes[rng.gen_range(0..3)], Direction::Down);
                }
                Event::KeyDown{ keycode: Some(Keycode::Left), repeat: false, .. } => {
                    spawn_vehicle(&mut vehicles, &mut stats, routes[rng.gen_range(0..3)], Direction::Left);
                }
                Event::KeyDown{ keycode: Some(Keycode::Right), repeat: false, .. } => {
                    spawn_vehicle(&mut vehicles, &mut stats, routes[rng.gen_range(0..3)], Direction::Right);
                }

                _ => {}
            }
        }

        // AUTO SPAWN -------------------------
        if auto_spawn && last_spawn.elapsed().as_secs_f32() > 0.7 {
            let r = routes[rng.gen_range(0..3)];
            let d = [Direction::Up, Direction::Down, Direction::Left, Direction::Right][rng.gen_range(0..4)];
            spawn_vehicle(&mut vehicles, &mut stats, r, d);
            last_spawn = Instant::now();
        }

        // UPDATE -----------------------------
        for v in &mut vehicles {
            if v.should_stop() && intersection_busy {
                v.update(dt, false);
            } else {
                intersection_busy = true;
                v.update(dt, true);
            }

            if v.is_out_of_bounds() {
                intersection_busy = false;
            }
        }

        // RENDER -----------------------------
        canvas.set_draw_color(Color::RGB(20,20,20));
        canvas.clear();
        draw(&mut canvas);

        for v in &vehicles {
            v.draw(&mut canvas);
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    println!("\n📊 Simulation finished.");
    show_stats_window(&stats);
}
