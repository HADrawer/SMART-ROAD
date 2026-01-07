use sdl2::{event::Event, keyboard::Keycode};
use std::time::{Duration, Instant};
use rand::Rng;
use rand::seq::SliceRandom; 
use rand::prelude::IndexedRandom; // Add this import
use sdl2::image::{InitFlag, LoadTexture};
use std::path::PathBuf;
use std::collections::HashMap;
use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use crate::config::{TILE_SIZE, GRID_W, GRID_H, MID_TILE};


// const PAVEMENT_WIDTH: i32 = 40;
// const WINDOW_SIZE: i32 = 900;
// const ROAD_COLOR: Color = Color::RGB(60, 60, 60);


mod intersection;
mod vehicle;
mod stats;
mod config;
pub const ROAD_HALF_TILES: i32 = 3;
use stats::{Stats, show_stats_window};
use intersection::*;
use vehicle::{Vehicle, Direction, Route };
#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Grass,
    Pavement,
    VerticalRoad,
    HorizontalRoad,
    Intersection,
}
type CarTextures<'a> = HashMap<(usize, Direction), Texture<'a>>;

/// Prevent stacking on spawn


fn build_map() -> [[Tile; GRID_W as usize]; GRID_H as usize] {
    let mut map = [[Tile::Grass; GRID_W as usize]; GRID_H as usize];

    let mid = GRID_W / 2;
    let road_half: i32 = 3; // 3 tiles per direction (6 lanes total later)

    for y in 0..GRID_H {
        for x in 0..GRID_W {
            // Vertical road
           // Vertical road
// Vertical road (3 left lanes + 3 right lanes)
if (x >= mid - 3 && x <= mid - 1) || (x >= mid + 1 && x <= mid + 3) {
    map[y as usize][x as usize] = Tile::VerticalRoad;
}

// Horizontal road (3 top lanes + 3 bottom lanes)
if (y >= mid - 3 && y <= mid - 1) || (y >= mid + 1 && y <= mid + 3) {
    map[y as usize][x as usize] = Tile::HorizontalRoad;
}

            // Intersection
            if (x >= mid - road_half && x <= mid + road_half)
                && (y >= mid - road_half && y <= mid + road_half)
            {
                map[y as usize][x as usize] = Tile::Intersection;
            }

            // Pavement ring
            if map[y as usize][x as usize] == Tile::Grass {
                if (x >= mid - road_half - 1 && x <= mid + road_half + 1)
                    || (y >= mid - road_half - 1 && y <= mid + road_half + 1)
                {
                    map[y as usize][x as usize] = Tile::Pavement;
                }
            }
        }
    }

    map
}

/// Unified spawner - only spawns in entry lanes
// src/main.rs
fn entry_lane_tile(dir: Direction, route: Route) -> i32 {
    match dir {
        Direction::Up => match route {
            Route::Left => MID_TILE + 1,
            Route::Straight => MID_TILE + 2,
            Route::Right => MID_TILE + 3,
        },

        Direction::Down => match route {
            Route::Left => MID_TILE - 3,
            Route::Straight => MID_TILE - 2,
            Route::Right => MID_TILE - 1,
        },

        Direction::Left => match route {
            
            Route::Left => MID_TILE - 3,
            Route::Straight => MID_TILE - 2,
            Route::Right => MID_TILE - 1,
        },

        Direction::Right => match route {
           Route::Left => MID_TILE + 1,
            Route::Straight => MID_TILE + 2,
            Route::Right => MID_TILE + 3,
        },
    }
}
fn spawn_vehicle(vehicles: &mut Vec<Vehicle>, stats: &mut Stats, r: Route, dir: Direction) {
   let lane_tile = entry_lane_tile(dir, r);

    let (x, y): (f32, f32) = match dir {
    Direction::Up => (
        (lane_tile * TILE_SIZE + TILE_SIZE / 2) as f32,
        (GRID_H * TILE_SIZE + 50) as f32,
    ),
    Direction::Down => (
        (lane_tile * TILE_SIZE + TILE_SIZE / 2) as f32,
        -50.0,
    ),
    Direction::Left => (
        (GRID_W * TILE_SIZE + 50) as f32,
        (lane_tile * TILE_SIZE + TILE_SIZE / 2) as f32,
    ),
    Direction::Right => (
        -50.0,
        (lane_tile * TILE_SIZE + TILE_SIZE / 2) as f32,
    ),
};

    let car_id = rand::rng().random_range(1..=4);
    let mut vehicle = Vehicle::new(dir, r, car_id);

    if !vehicle.path.is_empty() {
        vehicle.path[0] = (x, y);
        vehicle.x = x;
        vehicle.y = y;
    }

    vehicles.push(vehicle);

    stats.total_vehicles += 1;
}

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
// let pavement = texture_creator
//     .load_texture(
//         PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//             .join("assets")
//             .join("roads")
//             .join("pavement.png")
//     )
//     .unwrap();

// let vertical_road = texture_creator
//     .load_texture(
//         PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//             .join("assets")
//             .join("roads")
//             .join("vertical-road.png")
//     )
//     .unwrap();

// let horizontal_road = texture_creator
//     .load_texture(
//         PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//             .join("assets")
//             .join("roads")
//             .join("horizontal-road.png")
//     )
//     .unwrap();

// let intersection = texture_creator
//     .load_texture(
//         PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//             .join("assets")
//             .join("roads")
//             .join("intersection.png")
//     )
//     .unwrap();

let grass_tex = texture_creator
    .load_texture("assets/grass.png")
    .unwrap();

let pavement_tex = texture_creator
    .load_texture("assets/roads/pavement.png")
    .unwrap();

let verti_road_tex = texture_creator
    .load_texture("assets/roads/vertical-road.png")
    .unwrap();

let hori_road_tex = texture_creator
    .load_texture("assets/roads/horizontal-road.png")
    .unwrap();

let intersection_tex = texture_creator
    .load_texture("assets/roads/intersection.png")
    .unwrap();

     let map = build_map();

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
            .join("assets")
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

       


// ================= RENDER =================
// ===== GRID BACKGROUND =====
for y in 0..GRID_H {
    for x in 0..GRID_W {
        let tile = map[y as usize][x as usize];

        let tex = match tile {
            Tile::Grass => &grass_tex,
            Tile::Pavement => &pavement_tex,
            Tile::VerticalRoad => &verti_road_tex,
            Tile::HorizontalRoad => &hori_road_tex,
            Tile::Intersection => &intersection_tex,
        };

        canvas.copy(
            tex,
            None,
            Rect::new(
                x * TILE_SIZE,
                y * TILE_SIZE,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        ).unwrap();
    }
}




// 4️⃣ Cars (top layer)
for v in &vehicles {
    v.draw(&mut canvas, &car_textures);
}

canvas.present();




        std::thread::sleep(Duration::from_millis(16));
    }

    println!("\n📊 Simulation finished.");
    show_stats_window(&stats);
}