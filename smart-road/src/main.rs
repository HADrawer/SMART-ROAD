use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::time::{Duration, Instant};
use rand::Rng;
use rand::seq::SliceRandom; 
use rand::prelude::IndexedRandom; // Add this import
use sdl2::image::{InitFlag, LoadTexture};
use std::path::PathBuf;
use std::collections::HashMap;
use sdl2::render::Texture;

mod intersection;
mod vehicle;
mod stats;

use stats::{Stats, show_stats_window};
use intersection::*;
use vehicle::{Vehicle, Direction, Route };

type CarTextures<'a> = HashMap<(usize, Direction), Texture<'a>>;

/// Prevent stacking on spawn


/// Unified spawner
/// Unified spawner
/// Unified spawner - only spawns in entry lanes
// src/main.rs
fn spawn_vehicle(vehicles: &mut Vec<Vehicle>, stats: &mut Stats, r: Route, dir: Direction) {
    let center = CENTER as f32;
    let half_road = ROAD_WIDTH as f32 / 2.0;
    let lane_width = LANE_WIDTH as f32;
    
    // Calculate lane index (0-2) based on direction and route
    let lane_index = match (dir, r) {
        (Direction::Left, Route::Left) => 2,
        (Direction::Left, Route::Straight) => 1,
        (Direction::Left, Route::Right) => 0,
        (Direction::Right, Route::Left) => 2,
        (Direction::Right, Route::Straight) => 1,
        (Direction::Right, Route::Right) => 0,
        (Direction::Up, Route::Left) => 2,
        (Direction::Up, Route::Straight) => 1,
        (Direction::Up, Route::Right) => 0,
        (Direction::Down, Route::Left) => 2,
        (Direction::Down, Route::Straight) => 1,
        (Direction::Down, Route::Right) => 0,
    };
    
    // Calculate exact spawn position using lane geometry
    let (x, y) = match dir {
        Direction::Right => {
            // From right side, lane offset is Y coordinate
            let lane_y = center + half_road - (lane_index as f32 + 0.5) * lane_width;
            (-100.0, lane_y)
        },
        Direction::Left => {
            // From left side, lane offset is Y coordinate
            let lane_y = center - half_road + (lane_index as f32 + 0.5) * lane_width;
            (900.0, lane_y)
        },
        Direction::Up => {
            // From bottom, lane offset is X coordinate
            let lane_x = center + half_road - (lane_index as f32 + 0.5) * lane_width;
            (lane_x, 900.0)
        },
        Direction::Down => {
            // From top, lane offset is X coordinate
            let lane_x = center - half_road + (lane_index as f32 + 0.5) * lane_width;
            (lane_x, -100.0)
        },
    };

    

    println!("🚗 {:?} from {:?} at ({:.0},{:.0})", r, dir, x, y);
    
    // Create vehicle with exact position
let car_id = rand::rng().random_range(1..=4);
let mut vehicle = Vehicle::new(dir, r, car_id);    
    // Ensure vehicle starts at calculated position
    if !vehicle.path.is_empty() {
        vehicle.path[0] = (x, y);
        vehicle.x = x;
        vehicle.y = y;
    }
    
    vehicles.push(vehicle);

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

 // === SDL INIT ===
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    
    sdl2::image::init(InitFlag::PNG).unwrap();
    let window = video
        .window("Smart Intersection", 900, 900)
        .position_centered()
        .build()
        .unwrap();

  
  let mut canvas = window.into_canvas().present_vsync().build().unwrap();
let texture_creator = canvas.texture_creator();
// === ADDED ===
let mut car_textures: CarTextures = HashMap::new();

for car_id in 1..=4 {
    for dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let filename = format!(
            "car{}-{}.png",
            car_id,
            match dir {
                Direction::Up => "up",
                Direction::Down => "down",
                Direction::Left => "left",
                Direction::Right => "right",
            }
        );

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assests")
            .join(&filename);

        let texture = texture_creator
            .load_texture(&path)
            .expect("Failed to load car texture");

        car_textures.insert((car_id, dir), texture);
    }
}



    let mut events = sdl.event_pump().unwrap();

    let routes = [Route::Right, Route::Straight, Route::Left];
    let mut rng = rand::rng();

    let mut last_frame = Instant::now();
    let mut last_spawn = Instant::now();
    let mut auto_spawn = false;

    'run: loop {
        let dt = last_frame.elapsed().as_secs_f32();
        last_frame = Instant::now();
        stats.runtime += dt;
    
    let mut intersection_busy = false;
     
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
                    spawn_vehicle(&mut vehicles, &mut stats, routes[rng.random_range(0..3)], Direction::Up); // Use random_range
                }
                Event::KeyDown{ keycode: Some(Keycode::Down), repeat: false, .. } => {
                    spawn_vehicle(&mut vehicles, &mut stats, routes[rng.random_range(0..3)], Direction::Down);
                }
                Event::KeyDown{ keycode: Some(Keycode::Right), repeat: false, .. } => {
                    spawn_vehicle(&mut vehicles, &mut stats, routes[rng.random_range(0..3)], Direction::Right);
                }
                Event::KeyDown{ keycode: Some(Keycode::Left), repeat: false, .. } => {
                    spawn_vehicle(&mut vehicles, &mut stats, routes[rng.random_range(0..3)], Direction::Left);
                }

                _ => {}
            }
        }

        // AUTO SPAWN -------------------------
        if auto_spawn && last_spawn.elapsed().as_secs_f32() > 0.7 {
            let r = *routes.choose(&mut rng).unwrap();
            let d = *[Direction::Up, Direction::Down, Direction::Left, Direction::Right]
                .choose(&mut rng).unwrap();
            
            spawn_vehicle(&mut vehicles, &mut stats, r, d);
            last_spawn = Instant::now();
        }

        for v in &mut vehicles {
            v.update(dt);
        }

        // RENDER -----------------------------
        canvas.set_draw_color(Color::RGB(20,20,20));
        canvas.clear();
        draw(&mut canvas);

        for v in &vehicles {
v.draw(&mut canvas, &car_textures);
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    println!("\n📊 Simulation finished.");
    show_stats_window(&stats);
}