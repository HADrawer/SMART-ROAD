use sdl2::{rect::Rect, render::Canvas, video::Window};
use sdl2::render::Texture;
use std::collections::HashMap;

// 🔑 Import grid constants from main.rs (crate root)
use crate::{TILE_SIZE, GRID_W, GRID_H, MID_TILE, ROAD_HALF_TILES};
const INTERSECTION_MIN: i32 = MID_TILE - ROAD_HALF_TILES;
const INTERSECTION_MAX: i32 = MID_TILE + ROAD_HALF_TILES;

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

pub struct Vehicle {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub path: Vec<(f32, f32)>,
    pub current_target: usize,
    pub car_id: usize,
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
        // NORTH → cars moving DOWN
        Direction::Down => match route {
            Route::Left => MID_TILE - 3,
            Route::Straight => MID_TILE - 2,
            Route::Right => MID_TILE - 1,
        },

        // SOUTH → cars moving UP
        Direction::Up => match route {
            Route::Left => MID_TILE + 1,
            Route::Straight => MID_TILE + 2,
            Route::Right => MID_TILE + 3,
        },

        // WEST → cars moving RIGHT
        Direction::Right => match route {
            
             Route::Left => MID_TILE + 1,
            Route::Straight => MID_TILE + 2,
            Route::Right => MID_TILE + 3,
        },

        // EAST → cars moving LEFT
        Direction::Left => match route {
            Route::Left => MID_TILE - 3,
            Route::Straight => MID_TILE - 2,
            Route::Right => MID_TILE - 1,
        },
    }
}

fn exit_lane_tile(dir: Direction, route: Route) -> i32 {
    match dir {
        // FROM NORTH → exiting W / S / E
        Direction::Down => match route {
            Route::Left     => MID_TILE - 10, // WEST exit lanes
            Route::Straight => MID_TILE + 2, // SOUTH exit lanes
            Route::Right    => MID_TILE + 12, // EAST exit lanes
        },

        // FROM EAST → exiting S / W / N
        Direction::Left => match route {
            Route::Left     => MID_TILE + 2, // SOUTH
            Route::Straight => MID_TILE - 2, // WEST
            Route::Right    => MID_TILE - 2, // NORTH
        },

        // FROM SOUTH → exiting E / N / W
        Direction::Up => match route {
            Route::Left     => MID_TILE + 2, // EAST
            Route::Straight => MID_TILE - 2, // NORTH
            Route::Right    => MID_TILE - 2, // WEST
        },

        // FROM WEST → exiting N / E / S
        Direction::Right => match route {
            Route::Left     => MID_TILE - 2, // NORTH
            Route::Straight => MID_TILE + 2, // EAST
            Route::Right    => MID_TILE + 2, // SOUTH
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

        Self {
            x,
            y,
            speed: 140.0,
            path,
            current_target: 1,
            car_id,
        }
    }

    /// Move vehicle along tile-based path
    pub fn update(&mut self, dt: f32) {
        if self.current_target >= self.path.len() {
            return;
        }

        let (tx, ty) = self.path[self.current_target];
        let dx = tx - self.x;
        let dy = ty - self.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < self.speed * dt {
            self.current_target += 1;
            return;
        }

        self.x += dx / dist * self.speed * dt;
        self.y += dy / dist * self.speed * dt;
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
}

// =======================================================
// 🧭 TILE-BASED PATH GENERATION
// =======================================================

pub fn build_path(dir: Direction, route: Route) -> Vec<(f32, f32)> {
    
    let entry = entry_lane_tile(dir, route);
    let exit = exit_lane_tile(dir, route);

    //     Direction::Up => match route {
    //         Route::Left => MID_TILE + 1,
    //         Route::Straight => entry,
    //         Route::Right => MID_TILE + 3,
    //     },

    //     Direction::Down => match route {
    //         Route::Left => MID_TILE - 3,
    //         Route::Straight => entry,
    //         Route::Right => MID_TILE - 1,
    //     },

    //     Direction::Left => match route {
    //         Route::Left => MID_TILE - 3,
    //         Route::Straight => entry,
    //         Route::Right => MID_TILE - 1,
    //     },

    //     Direction::Right => match route {
    //         Route::Left => MID_TILE + 1,
    //         Route::Straight => entry,
    //         Route::Right => MID_TILE + 3,
    //     },
    // };

    let mut tiles = Vec::new();

    match dir {
        Direction::Up => {
            tiles.push((entry, GRID_H + 1));
            tiles.push((entry, INTERSECTION_MAX));
            match route {
                Route::Straight => tiles.push((entry, -2)),
                Route::Left => tiles.push((exit, INTERSECTION_MAX)),
                Route::Right => tiles.push((exit, INTERSECTION_MAX)),
            }
        }

        Direction::Down => {
            tiles.push((entry, -2));
            tiles.push((entry, INTERSECTION_MIN));
            match route {
                Route::Straight => tiles.push((entry, GRID_H + 1)),
                Route::Left => tiles.push((exit, INTERSECTION_MIN)),
                Route::Right => tiles.push((exit, INTERSECTION_MIN)),
            }
        }

        Direction::Left => {
            tiles.push((GRID_W + 1, entry));
            tiles.push((INTERSECTION_MIN, entry));
            match route {
                Route::Straight => tiles.push((-2, entry)),
                Route::Left => tiles.push((INTERSECTION_MIN, exit)),
                Route::Right => tiles.push((INTERSECTION_MIN, exit)),
            }
        }

        Direction::Right => {
            tiles.push((-2, entry));
            tiles.push((INTERSECTION_MIN, entry));
            match route {
                Route::Straight => tiles.push((GRID_W + 1, entry)),
                Route::Left => tiles.push((INTERSECTION_MIN, exit)),
                Route::Right => tiles.push((INTERSECTION_MIN, exit)),
            }
        }
    }

    tiles.into_iter().map(|(x, y)| tile_center(x, y)).collect()
}
