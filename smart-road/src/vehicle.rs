use sdl2::{rect::Rect, render::Canvas, video::Window};
use sdl2::render::Texture;
use std::collections::HashMap;

// 🔑 Import grid constants from main.rs (crate root)
use crate::{TILE_SIZE, GRID_W, GRID_H, MID_TILE, ROAD_HALF_TILES};
const INTERSECTION_MIN: i32 = MID_TILE - ROAD_HALF_TILES;
const INTERSECTION_MAX: i32 = MID_TILE + ROAD_HALF_TILES ;

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
            Route::Left     => MID_TILE - 10, // SOUTH
            Route::Straight => MID_TILE - 2, // WEST
            Route::Right    => MID_TILE + 10, // NORTH
        },

        // FROM SOUTH → exiting E / N / W
        Direction::Up => match route {
            Route::Left     => MID_TILE - 12, // EAST
            Route::Straight => MID_TILE - 2, // NORTH
            Route::Right    => MID_TILE + 10, // WEST
        },

        // FROM WEST → exiting N / E / S
        Direction::Right => match route {
            Route::Left     => MID_TILE - 10, // NORTH
            Route::Straight => MID_TILE + 2, // EAST
            Route::Right    => MID_TILE + 10, // SOUTH
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
    // 1️⃣ spawn outside
    tiles.push((entry, GRID_H + 1));

    // 2️⃣ enter intersection vertically
    tiles.push((entry, INTERSECTION_MAX));

    match route {
        // 3️⃣ straight continues vertically
        Route::Straight => {
            tiles.push((entry, -2));
        }

        // 4️⃣ LEFT TURN: vertical → horizontal → exit
        Route::Left => {
            // go UP to turning row
            tiles.push((entry, INTERSECTION_MAX-4));

            // then go LEFT into west exit lanes
            tiles.push((exit, INTERSECTION_MAX-4));
        }

        // 5️⃣ RIGHT TURN: vertical → horizontal → exit
        Route::Right => {
            // go UP to turning row
            tiles.push((entry, INTERSECTION_MAX));

            // then go RIGHT into east exit lanes
            tiles.push((exit, INTERSECTION_MAX));
        }
    }
}
       Direction::Down => {
    // 1️⃣ spawn above screen
    tiles.push((entry, -2));

    // 2️⃣ enter intersection vertically
    tiles.push((entry, INTERSECTION_MIN));

    match route {
        // 3️⃣ straight continues down
        Route::Straight => {
            tiles.push((entry, GRID_H + 1));
        }

        // 4️⃣ LEFT TURN → go RIGHT (east)
        Route::Left => {
            // go DOWN to turning row
            tiles.push((entry, INTERSECTION_MIN));

            // then go RIGHT into east exit lanes
            tiles.push((exit, INTERSECTION_MIN));
        }

        // 5️⃣ RIGHT TURN → go LEFT (west)
        Route::Right => {
            // go DOWN to turning row
            tiles.push((entry, INTERSECTION_MIN+4));

            // then go LEFT into west exit lanes
            tiles.push((exit, INTERSECTION_MIN+4));
        }
    }
}

       Direction::Left => {
    // 1️⃣ spawn right of screen
    tiles.push((GRID_W + 1, entry));

    // 2️⃣ enter intersection horizontally
    tiles.push((INTERSECTION_MAX, entry));

    match route {
        // 3️⃣ straight continues left
        Route::Straight => {
            tiles.push((-2, entry));
        }

        // 4️⃣ LEFT TURN → go DOWN (south)
        Route::Left => {
            // go LEFT to turning column
            tiles.push((INTERSECTION_MAX, entry));

            // then go DOWN into south exit lanes
            tiles.push((INTERSECTION_MAX, exit));
        }

        // 5️⃣ RIGHT TURN → go UP (north)
        Route::Right => {
            // go LEFT to turning column
            tiles.push((INTERSECTION_MAX-4, entry));

            // then go UP into north exit lanes
            tiles.push((INTERSECTION_MAX-4, exit));
        }
    }
}
        Direction::Right => {
    // 1️⃣ spawn left of screen
    tiles.push((-2, entry));

    // 2️⃣ enter intersection horizontally
    tiles.push((INTERSECTION_MIN, entry));

    match route {
        // 3️⃣ straight continues right
        Route::Straight => {
            tiles.push((GRID_W + 1, entry));
        }

        // 4️⃣ LEFT TURN → go UP (north)
        Route::Left => {
            // go RIGHT to turning column
            tiles.push((INTERSECTION_MIN+4, entry));

            // then go UP into north exit lanes
            tiles.push((INTERSECTION_MIN+4, exit));
        }

        // 5️⃣ RIGHT TURN → go DOWN (south)
        Route::Right => {
            // go RIGHT to turning column
            tiles.push((INTERSECTION_MIN, entry));

            // then go DOWN into south exit lanes
            tiles.push((INTERSECTION_MIN, exit));
        }
    }
}
    }

    tiles.into_iter().map(|(x, y)| tile_center(x, y)).collect()
}
