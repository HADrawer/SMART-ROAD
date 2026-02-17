use sdl2::{rect::Rect, render::Canvas, video::Window};
use sdl2::render::Texture;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{TILE_SIZE, GRID_W, GRID_H, MID_TILE, ROAD_HALF_TILES};
const INTERSECTION_MIN: i32 = MID_TILE - ROAD_HALF_TILES;
const INTERSECTION_MAX: i32 = MID_TILE + ROAD_HALF_TILES;

const SAFETY_DISTANCE: f32 = 70.0; 
const EMERGENCY_BRAKE_DISTANCE: f32 = 35.0; 
const MIN_CRAWL_SPEED: f32 = 20.0; 

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VelocityLevel {
    Slow = 0,
    Medium = 1,
    Fast = 2,
}

impl VelocityLevel {
    pub fn to_speed(&self) -> f32 {
        match self {
            VelocityLevel::Slow => 60.0,
            VelocityLevel::Medium => 120.0,
            VelocityLevel::Fast => 180.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Route {
    Right,
    Straight,
    Left,
}

#[derive(Clone)]
pub struct Vehicle {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub target_speed: f32, 
    pub velocity_level: VelocityLevel,
    pub path: Vec<(f32, f32)>,
    pub current_target: usize,
    pub car_id: usize,

    pub finished: bool, 
    pub length_multiplier: f32, 
    pub distance_traveled: f32,
    pub time_in_system: f32,
    pub entered_intersection: bool,
    pub intersection_entry_time: f32,
    pub intersection_exit_time: f32,
    pub width: u32,
    pub height: u32,
    
}

fn tile_center(tx: i32, ty: i32) -> (f32, f32) {
    (
        (tx * TILE_SIZE + TILE_SIZE / 2) as f32,
        (ty * TILE_SIZE + TILE_SIZE / 2) as f32,
    )
}

pub fn entry_lane_tile(dir: Direction, route: Route) -> i32 {
    match dir {
        Direction::Down => match route {
            Route::Left => MID_TILE - 3,
            Route::Straight => MID_TILE - 2,
            Route::Right => MID_TILE - 1,
        },
        Direction::Up => match route {
            Route::Left => MID_TILE + 1,
            Route::Straight => MID_TILE + 2,
            Route::Right => MID_TILE + 3,
        },
        Direction::Right => match route {
            Route::Left => MID_TILE + 1,
            Route::Straight => MID_TILE + 2,
            Route::Right => MID_TILE + 3,
        },
        Direction::Left => match route {
            Route::Left => MID_TILE - 3,
            Route::Straight => MID_TILE - 2,
            Route::Right => MID_TILE - 1,
        },
    }
}

fn exit_lane_tile(dir: Direction, route: Route) -> i32 {
    match dir {
        Direction::Down => match route {
            Route::Left => MID_TILE - 10,
            Route::Straight => MID_TILE + 2,
            Route::Right => MID_TILE + 12,
        },
        Direction::Left => match route {
            Route::Left => MID_TILE - 10,
            Route::Straight => MID_TILE - 2,
            Route::Right => MID_TILE + 10,
        },
        Direction::Up => match route {
            Route::Left => MID_TILE - 12,
            Route::Straight => MID_TILE - 2,
            Route::Right => MID_TILE + 10,
        },
        Direction::Right => match route {
            Route::Left => MID_TILE - 10,
            Route::Straight => MID_TILE + 2,
            Route::Right => MID_TILE + 10,
        },
    }
}

// Global vehicle ID counter
static mut VEHICLE_ID_COUNTER: usize = 0;

fn get_next_vehicle_id() -> usize {
    unsafe {
        VEHICLE_ID_COUNTER += 1;
        VEHICLE_ID_COUNTER
    }
}

impl Vehicle {
    pub fn new(direction: Direction, route: Route, car_id: usize) -> Self {
        let path = build_path(direction, route);
        let (x, y) = path[0];
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        // Start with medium velocity by default
        let velocity_level = VelocityLevel::Medium;
        let target_speed = velocity_level.to_speed();

        Self {
            id,
            x,
            y,
            speed: target_speed,
            target_speed,
            velocity_level,
            path,
            current_target: 1,
            car_id,
            finished: false,
            length_multiplier: 1.0, 
            distance_traveled: 0.0,
            time_in_system: 0.0,
            entered_intersection: false,
            intersection_entry_time: 0.0,
            intersection_exit_time: 0.0,
            width: 30,
            height: 50,
           
        }
    }
     pub fn has_priority_over(&self, other: &Vehicle) -> bool {
        self.id < other.id
    }
    pub fn is_in_intersection(&self) -> bool {
        let tile_x = (self.x / TILE_SIZE as f32) as i32;
        let tile_y = (self.y / TILE_SIZE as f32) as i32;
        
        tile_x >= INTERSECTION_MIN && tile_x <= INTERSECTION_MAX &&
        tile_y >= INTERSECTION_MIN && tile_y <= INTERSECTION_MAX
    }
    pub fn distance_to(&self, other: &Vehicle) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
    pub fn is_vehicle_ahead(&self, other: &Vehicle) -> bool {
        let lateral_threshold = 50.0; 
        let dir = self.facing_direction();
        let distance = self.distance_to(other);

        if distance > SAFETY_DISTANCE * 1.5 {
            return false;
        }
        
        match dir {
            Direction::Up => { other.y < self.y && (other.x - self.x).abs() < lateral_threshold
            }
            Direction::Down => {other.y > self.y && (other.x - self.x).abs() < lateral_threshold
            }
            Direction::Left => { other.x < self.x && (other.y - self.y).abs() < lateral_threshold
            }
            Direction::Right => {other.x > self.x && (other.y - self.y).abs() < lateral_threshold
            }
        }
    }

    pub fn set_velocity_level(&mut self, level: VelocityLevel) {
        self.velocity_level = level;
        self.target_speed = level.to_speed();
    }

    fn update_speed(&mut self, dt: f32) {
        let acceleration = 200.0; 
        let speed_diff = self.target_speed - self.speed;
        
        if speed_diff.abs() < acceleration * dt {
            self.speed = self.target_speed;
        } else if speed_diff > 0.0 {
            self.speed += acceleration * dt;
        } else {
            self.speed -= acceleration * dt;
        }
        self.speed = self.speed.max(0.0);
    }

   pub fn update(&mut self, dt: f32, other_vehicles: &[Vehicle]) {
    if self.current_target >= self.path.len() {
        return;
    }

    if let Some(&last_point) = self.path.last() {
        if (self.x - last_point.0).abs() < 5.0 && (self.y - last_point.1).abs() < 5.0 {
            self.finished = true;
            return;
        }
    }

    self.time_in_system += dt;

    let mut must_stop = false;
    let mut min_allowed_speed = self.velocity_level.to_speed();

    for other in other_vehicles {
        if self.id == other.id { continue; }

        let distance = self.distance_to(other);

      
        if self.is_vehicle_ahead(other) {
            let slow_factor = (distance / SAFETY_DISTANCE).max(0.2);
            min_allowed_speed = min_allowed_speed.min(other.speed * slow_factor);

            if distance < EMERGENCY_BRAKE_DISTANCE {
                must_stop = true;
            }
        }

      
        if self.is_approaching_intersection() && other.is_approaching_intersection() {
            if !self.is_vehicle_ahead(other) && !other.is_vehicle_ahead(self) {
                
                if self.id > other.id && !self.has_priority_over(other) && distance < SAFETY_DISTANCE {
                    must_stop = true;
                }
            }
        }
    }

    
    if must_stop {
        self.target_speed = 0.0;
    } else {
        self.target_speed = min_allowed_speed;
    }

    self.update_speed(dt);

    let (tx, ty) = self.path[self.current_target];
    let dx = tx - self.x;
    let dy = ty - self.y;
    let dist = (dx * dx + dy * dy).sqrt();

    if dist < self.speed * dt {
        self.current_target += 1;
        return;
    }

    let movement = self.speed * dt;
    let next_x = self.x + dx / dist * movement;
    let next_y = self.y + dy / dist * movement;

    let mut can_move = true;
    for other in other_vehicles {
        if self.id == other.id { continue; }

        let dx2 = other.x - next_x;
        let dy2 = other.y - next_y;
        let next_distance = (dx2 * dx2 + dy2 * dy2).sqrt();

        let min_gap = self.height as f32;

        if self.is_vehicle_ahead(other) && next_distance < min_gap {
            can_move = false;
            break;
        }
    }

    if can_move {
        self.x = next_x;
        self.y = next_y;
        self.distance_traveled += movement;
    } else {
        self.speed = 0.0;
    }

    self.target_speed = self.target_speed.min(self.velocity_level.to_speed());
}
    pub fn is_approaching_intersection(&self) -> bool {
        let tile_x = (self.x / TILE_SIZE as f32) as i32;
        let tile_y = (self.y / TILE_SIZE as f32) as i32;

        let buffer = 2;

        tile_x >= INTERSECTION_MIN - buffer &&
        tile_x <= INTERSECTION_MAX + buffer &&
        tile_y >= INTERSECTION_MIN - buffer &&
        tile_y <= INTERSECTION_MAX + buffer
    }

    pub fn draw(
    &self,
    canvas: &mut Canvas<Window>,
    textures: &HashMap<(usize, Direction), Texture>,
) {
    let dir = self.facing_direction();
    let texture = &textures[&(self.car_id, dir)];

    use sdl2::render::TextureQuery;
    let TextureQuery { width, height, .. } = texture.query();

    let scale = 0.5;
    let width_scaled = (width as f32 * scale) as u32;

    let height_scaled = (height as f32 * scale * self.length_multiplier) as u32;

    let dst = Rect::new(
        (self.x - width_scaled as f32 / 2.0) as i32,
        (self.y - height_scaled as f32 / 2.0) as i32,
        width_scaled,
        height_scaled,
    );

    canvas.copy(texture, None, dst).unwrap();
}


    pub fn facing_direction(&self) -> Direction {
        if self.current_target >= self.path.len() {
            return Direction::Up;
        }

        let (tx, ty) = self.path[self.current_target];
        let dx = tx - self.x;
        let dy = ty - self.y;

        if dx.abs() > dy.abs() {
            if dx > 0.0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else {
            if dy > 0.0 {
                Direction::Down
            } else {
                Direction::Up
            }
        }
    }

   pub fn is_out_of_bounds(&self) -> bool {
    self.x < -50.0
        || self.x > (GRID_W * TILE_SIZE + 50) as f32
        || self.y < -50.0
        || self.y > (GRID_H * TILE_SIZE + 50) as f32
}


    pub fn get_intersection_time(&self) -> f32 {
        if self.intersection_exit_time > 0.0 {
            self.intersection_exit_time - self.intersection_entry_time
        } else if self.entered_intersection {
            self.time_in_system - self.intersection_entry_time
        } else {
            0.0
        }
    }

    pub fn get_average_velocity(&self) -> f32 {
        if self.time_in_system > 0.0 {
            self.distance_traveled / self.time_in_system
        } else {
            0.0
        }
    }
}

pub fn build_path(dir: Direction, route: Route) -> Vec<(f32, f32)> {
    let entry = entry_lane_tile(dir, route);
    let exit = exit_lane_tile(dir, route);

    let mut tiles = Vec::new();

    match dir {
        Direction::Up => {
            tiles.push((entry, GRID_H + 1));
            tiles.push((entry, INTERSECTION_MAX));

            match route {
                Route::Straight => {
                    tiles.push((entry, -2));
                }
                Route::Left => {
                    tiles.push((entry, INTERSECTION_MAX - 4));
                    tiles.push((exit, INTERSECTION_MAX - 4));
                }
                Route::Right => {
                    tiles.push((entry, INTERSECTION_MAX));
                    tiles.push((exit, INTERSECTION_MAX));
                }
            }
        }
        Direction::Down => {
            tiles.push((entry, -2));
            tiles.push((entry, INTERSECTION_MIN));

            match route {
                Route::Straight => {
                    tiles.push((entry, GRID_H + 1));
                }
                Route::Left => {
                    tiles.push((entry, INTERSECTION_MIN));
                    tiles.push((exit, INTERSECTION_MIN));
                }
                Route::Right => {
                    tiles.push((entry, INTERSECTION_MIN + 4));
                    tiles.push((exit, INTERSECTION_MIN + 4));
                }
            }
        }
        Direction::Left => {
            tiles.push((GRID_W + 1, entry));
            tiles.push((INTERSECTION_MAX, entry));

            match route {
                Route::Straight => {
                    tiles.push((-2, entry));
                }
                Route::Left => {
                    tiles.push((INTERSECTION_MAX, entry));
                    tiles.push((INTERSECTION_MAX, exit));
                }
                Route::Right => {
                    tiles.push((INTERSECTION_MAX - 4, entry));
                    tiles.push((INTERSECTION_MAX - 4, exit));
                }
            }
        }
        Direction::Right => {
            tiles.push((-2, entry));
            tiles.push((INTERSECTION_MIN, entry));

            match route {
                Route::Straight => {
                    tiles.push((GRID_W + 1, entry));
                }
                Route::Left => {
                    tiles.push((INTERSECTION_MIN + 4, entry));
                    tiles.push((INTERSECTION_MIN + 4, exit));
                }
                Route::Right => {
                    tiles.push((INTERSECTION_MIN, entry));
                    tiles.push((INTERSECTION_MIN, exit));
                }
            }
        }
    }

    tiles.into_iter().map(|(x, y)| tile_center(x, y)).collect()
}