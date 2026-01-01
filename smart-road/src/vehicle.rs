use sdl2::{rect::Rect, pixels::Color, render::Canvas, video::Window};
use crate::intersection::{CENTER, ROAD_WIDTH, LANE_WIDTH, in_intersection};

#[derive(Clone, Copy, Debug)]
pub enum Direction { Up, Down, Left, Right }

#[derive(Clone, Copy, Debug,PartialEq)]
pub enum Route { Right, Straight, Left }

pub struct Vehicle {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub direction: Direction,
    pub route: Route,
}

impl Vehicle {
    
    pub fn new(x: f32, y: f32, direction: Direction, route: Route) -> Self {
        Self {
            x,
            y,
            direction,
            route,
            speed: 140.0,
        }
    }

    /// 🚦 STOP LINE CHECK
pub fn should_stop(&self) -> bool {
    // White divider distance from center
    const DIVIDER: f32 = 120.0;

    match self.direction {
        Direction::Up    => self.y <= CENTER as f32 + DIVIDER + 5.0,
        Direction::Down  => self.y >= CENTER as f32 - DIVIDER - 55.0, 
        Direction::Left  => self.x <= CENTER as f32 + DIVIDER + 5.0,
        Direction::Right => self.x >= CENTER as f32 - DIVIDER - 55.0,
    }
}



    /// 🔁 SMOOTH TURNING MOVEMENT
    pub fn apply_turn(&mut self, dt: f32) {
        let turn_speed = self.speed * 0.7 * dt;
        let lane_offset = LANE_WIDTH as f32;

        match (self.direction, self.route) {

            // From Bottom going Up
            (Direction::Up, Route::Right) => {
                self.x += turn_speed;
                if self.x >= CENTER as f32 + lane_offset { self.direction = Direction::Right; }
            }
            (Direction::Up, Route::Left) => {
                self.x -= turn_speed;
                if self.x <= CENTER as f32 - lane_offset { self.direction = Direction::Left; }
            }

            // From Top going Down
            (Direction::Down, Route::Right) => {
                self.x -= turn_speed;
                if self.x <= CENTER as f32 - lane_offset { self.direction = Direction::Left; }
            }
            (Direction::Down, Route::Left) => {
                self.x += turn_speed;
                if self.x >= CENTER as f32 + lane_offset { self.direction = Direction::Right; }
            }

            // From Left going Right
            (Direction::Left, Route::Right) => {
                self.y -= turn_speed;
                if self.y <= CENTER as f32 - lane_offset { self.direction = Direction::Up; }
            }
            (Direction::Left, Route::Left) => {
                self.y += turn_speed;
                if self.y >= CENTER as f32 + lane_offset { self.direction = Direction::Down; }
            }

            // From Right going Left
            (Direction::Right, Route::Right) => {
                self.y += turn_speed;
                if self.y >= CENTER as f32 + lane_offset { self.direction = Direction::Down; }
            }
            (Direction::Right, Route::Left) => {
                self.y -= turn_speed;
                if self.y <= CENTER as f32 - lane_offset { self.direction = Direction::Up; }
            }

            _ => {} // Straight -> nothing
        }
    }

    /// 🚗 CAR UPDATE LOOP
    pub fn update(&mut self, dt: f32, can_move: bool) {
        // 🚦 STOP if intersection busy
        if !can_move && self.should_stop() {
            // println!("Vehicle stopped at ({:.1}, {:.1}) dir={:?} route={:?}", self.x, self.y, self.direction, self.route);
            return;
        }

        // 🔁 Turn logic
        if in_intersection(self.x, self.y) && self.route != Route::Straight {
            self.apply_turn(dt);
        }

        // ➡️ Move based on direction
        match self.direction {
            Direction::Up    => self.y -= self.speed * dt,
            Direction::Down  => self.y += self.speed * dt,
            Direction::Left  => self.x -= self.speed * dt,
            Direction::Right => self.x += self.speed * dt,
        }
        
    }

    /// 🎨 DRAW
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 200, 0));
        let rect = Rect::new(self.x as i32, self.y as i32, 30, 50);
        canvas.fill_rect(rect).unwrap();
    }   
     pub fn is_out_of_bounds(&self) -> bool {
        self.x < -100.0 || self.x > 1000.0 || self.y < -100.0 || self.y > 1000.0
    }

    
}
