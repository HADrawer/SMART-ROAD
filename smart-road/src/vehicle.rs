use sdl2::{rect::Rect, render::Canvas, video::Window};
use sdl2::render::Texture;
use std::collections::HashMap;

// 🔒 Import grid constants from main.rs (crate root)
use crate::{TILE_SIZE, GRID_W, GRID_H, MID_TILE, ROAD_HALF_TILES};
const INTERSECTION_MIN: i32 = MID_TILE - ROAD_HALF_TILES;
const INTERSECTION_MAX: i32 = MID_TILE + ROAD_HALF_TILES;

// 🚦 Safety distance in pixels
const SAFETY_DISTANCE: f32 = 80.0;

// 🎯 Velocity levels for traffic control
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
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub target_speed: f32, // Desired speed based on velocity level
    pub velocity_level: VelocityLevel,
    pub path: Vec<(f32, f32)>,
    pub current_target: usize,
    pub car_id: usize,
    
    // 📊 Physics tracking
    pub distance_traveled: f32,
    pub time_in_system: f32,
    pub entered_intersection: bool,
    pub intersection_entry_time: f32,
    pub intersection_exit_time: f32,
}

// =======================================================
// 🔧 TILE HELPERS
// =======================================================

/// Convert tile coords → pixel center
fn tile_center(tx: i32, ty: i32) -> (f32, f32) {
    (
        (tx * TILE_SIZE + TILE_SIZE / 2) as f32,
        (ty * TILE_SIZE + TILE_SIZE / 2) as f32,
    )
}

fn entry_lane_tile(dir: Direction, route: Route) -> i32 {
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

// =======================================================
// 🚗 VEHICLE IMPLEMENTATION
// =======================================================

impl Vehicle {
    pub fn new(direction: Direction, route: Route, car_id: usize) -> Self {
        let path = build_path(direction, route);
        let (x, y) = path[0];
        
        // Start with medium velocity by default
        let velocity_level = VelocityLevel::Medium;
        let target_speed = velocity_level.to_speed();

        Self {
            x,
            y,
            speed: target_speed,
            target_speed,
            velocity_level,
            path,
            current_target: 1,
            car_id,
            distance_traveled: 0.0,
            time_in_system: 0.0,
            entered_intersection: false,
            intersection_entry_time: 0.0,
            intersection_exit_time: 0.0,
        }
    }

    /// Check if vehicle is in the intersection zone
    pub fn is_in_intersection(&self) -> bool {
        let tile_x = (self.x / TILE_SIZE as f32) as i32;
        let tile_y = (self.y / TILE_SIZE as f32) as i32;
        
        tile_x >= INTERSECTION_MIN && tile_x <= INTERSECTION_MAX &&
        tile_y >= INTERSECTION_MIN && tile_y <= INTERSECTION_MAX
    }

    /// Calculate distance to another vehicle
    pub fn distance_to(&self, other: &Vehicle) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Check if another vehicle is ahead on the path
    pub fn is_vehicle_ahead(&self, other: &Vehicle) -> bool {
        // Check if vehicles are on similar paths (within same lane corridor)
        let lateral_threshold = 60.0; // pixels
        
        let dir = self.facing_direction();
        match dir {
            Direction::Up => {
                // Other vehicle is ahead if it's above (smaller y) and in same lane
                other.y < self.y && (other.x - self.x).abs() < lateral_threshold
            }
            Direction::Down => {
                // Other vehicle is ahead if it's below (larger y) and in same lane
                other.y > self.y && (other.x - self.x).abs() < lateral_threshold
            }
            Direction::Left => {
                // Other vehicle is ahead if it's to the left (smaller x) and in same lane
                other.x < self.x && (other.y - self.y).abs() < lateral_threshold
            }
            Direction::Right => {
                // Other vehicle is ahead if it's to the right (larger x) and in same lane
                other.x > self.x && (other.y - self.y).abs() < lateral_threshold
            }
        }
    }

    /// Set velocity level for traffic control
    pub fn set_velocity_level(&mut self, level: VelocityLevel) {
        self.velocity_level = level;
        self.target_speed = level.to_speed();
    }

    /// Update speed smoothly (acceleration/deceleration)
    fn update_speed(&mut self, dt: f32) {
        let acceleration = 100.0; // pixels/s²
        let speed_diff = self.target_speed - self.speed;
        
        if speed_diff.abs() < acceleration * dt {
            self.speed = self.target_speed;
        } else if speed_diff > 0.0 {
            self.speed += acceleration * dt;
        } else {
            self.speed -= acceleration * dt;
        }
        
        // Ensure speed doesn't go negative
        self.speed = self.speed.max(0.0);
    }

    /// Move vehicle along tile-based path with collision avoidance
    pub fn update(&mut self, dt: f32, other_vehicles: &[Vehicle]) {
        if self.current_target >= self.path.len() {
            return;
        }

        // 📊 Track time in system
        self.time_in_system += dt;

        // 🚦 Check for vehicles ahead and adjust speed
        let mut should_slow_down = false;
        for other in other_vehicles {
            if std::ptr::eq(self, other) {
                continue; // Skip self
            }
            
            if self.is_vehicle_ahead(other) {
                let distance = self.distance_to(other);
                if distance < SAFETY_DISTANCE {
                    should_slow_down = true;
                    // Emergency brake if too close
                    if distance < SAFETY_DISTANCE * 0.5 {
                        self.target_speed = 0.0;
                    } else {
                        // Slow down proportionally
                        self.target_speed = VelocityLevel::Slow.to_speed();
                    }
                    break;
                }
            }
        }

        // Resume normal speed if no obstacles
        if !should_slow_down {
            self.target_speed = self.velocity_level.to_speed();
        }

        // 🎯 Update speed smoothly
        self.update_speed(dt);

        // 📍 Track intersection entry/exit
        let was_in_intersection = self.entered_intersection;
        let is_in_intersection = self.is_in_intersection();
        
        if !was_in_intersection && is_in_intersection {
            self.entered_intersection = true;
            self.intersection_entry_time = self.time_in_system;
        } else if was_in_intersection && !is_in_intersection && self.intersection_exit_time == 0.0 {
            self.intersection_exit_time = self.time_in_system;
        }

        // 🚗 Move along path
        let (tx, ty) = self.path[self.current_target];
        let dx = tx - self.x;
        let dy = ty - self.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < self.speed * dt {
            self.current_target += 1;
            return;
        }

        let movement = self.speed * dt;
        self.x += dx / dist * movement;
        self.y += dy / dist * movement;
        
        // 📊 Track distance
        self.distance_traveled += movement;
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
        let w = (width as f32 * scale) as u32;
        let h = (height as f32 * scale) as u32;

        let dst = Rect::new(
            (self.x - w as f32 / 2.0) as i32,
            (self.y - h as f32 / 2.0) as i32,
            w,
            h,
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
        self.x < -200.0 || self.x > (GRID_W * TILE_SIZE + 200) as f32
            || self.y < -200.0 || self.y > (GRID_H * TILE_SIZE + 200) as f32
    }

    /// Get intersection traversal time
    pub fn get_intersection_time(&self) -> f32 {
        if self.intersection_exit_time > 0.0 {
            self.intersection_exit_time - self.intersection_entry_time
        } else if self.entered_intersection {
            self.time_in_system - self.intersection_entry_time
        } else {
            0.0
        }
    }

    /// Get average velocity
    pub fn get_average_velocity(&self) -> f32 {
        if self.time_in_system > 0.0 {
            self.distance_traveled / self.time_in_system
        } else {
            0.0
        }
    }
}

// =======================================================
// 🧭 TILE-BASED PATH GENERATION
// =======================================================

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