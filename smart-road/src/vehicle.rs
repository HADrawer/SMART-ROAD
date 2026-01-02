use sdl2::{rect::Rect, pixels::Color, render::Canvas, video::Window};
use crate::intersection::{CENTER, ROAD_WIDTH, LANE_WIDTH, HALF_ROAD};

#[derive(Clone, Copy, Debug)]
pub enum Direction { Up, Down, Left, Right }

#[derive(Clone, Copy, Debug,PartialEq)]
pub enum Route { Right, Straight, Left }

pub struct Vehicle {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub path: Vec<(f32, f32)>,
    pub current_target: usize,
}

impl Vehicle {
    
    pub fn new(direction: Direction, route: Route) -> Self {
    let path = build_path(direction, route);
    let (x, y) = path[0];

    Self {
        x,
        y,
        speed: 140.0,
        path,
        current_target: 1,
    }
}





    

    /// 🚗 CAR UPDATE LOOP
    pub fn update(&mut self, dt: f32) {
        
        if self.current_target >= self.path.len() {
            return;
        }
        let (tx, ty) = self.path[self.current_target];

        let dx = tx - self.x;
        let dy = ty - self.y;
        let dist = (dx*dx + dy*dy).sqrt();

       if dist < self.speed * dt {
        self.current_target += 1;
        return;
    }

    self.x += dx / dist * self.speed * dt;
    self.y += dy / dist * self.speed * dt;
    

        
        
    }
   

    /// 🎨 DRAW
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 200, 0));
        let rect = Rect::new(self.x as i32 - 15, self.y as i32 - 25, 30, 50);
        canvas.fill_rect(rect).unwrap();
    }   
     pub fn is_out_of_bounds(&self) -> bool {
        self.x < -100.0 || self.x > 1000.0 || self.y < -100.0 || self.y > 1000.0
    }
    

}

pub fn build_path(dir: Direction, route: Route) -> Vec<(f32, f32)> {
    let c = CENTER as f32;
    let road_half = ROAD_WIDTH as f32 / 2.0;
    let lane = LANE_WIDTH as f32;

    match (dir, route) {

        // ================= LEFT → =================
        (Direction::Left, Route::Straight) => {
            let y = c + road_half - lane * 1.5;
            vec![(900.0, y), (-100.0, y)]
        }

        (Direction::Left, Route::Right) => {
            let y = c + road_half - lane * 0.5;
            vec![
                (900.0, y),
                (c + 80.0, y),
                (c, c + 80.0),
                (c, 1000.0),
            ]
        }

        (Direction::Left, Route::Left) => {
            let y = c + road_half - lane * 2.5;
            vec![
                (900.0, y),
                (c + 80.0, y),
                (c + 60.0, c),
                (c + 60.0, -100.0),
            ]
        }

        // ================= RIGHT ← =================
        (Direction::Right, Route::Straight) => {
            let y = c - road_half + lane * 1.5;
            vec![(-100.0, y), (1000.0, y)]
        }

        (Direction::Right, Route::Right) => {
            let y = c - road_half + lane * 0.5;
            vec![
                (-100.0, y),
                (c - 80.0, y),
                (c, c - 80.0),
                (c, -100.0),
            ]
        }

        (Direction::Right, Route::Left) => {
            let y = c - road_half + lane * 2.5;
            vec![
                (-100.0, y),
                (c - 80.0, y),
                (c - 60.0, c),
                (c - 60.0, 1000.0),
            ]
        }

        // ================= UP ↑ =================
        (Direction::Up, Route::Straight) => {
            let x = c - road_half + lane * 1.5;
            vec![(x, 900.0), (x, -100.0)]
        }

        (Direction::Up, Route::Right) => {
            let x = c - road_half + lane * 0.5;
            vec![
                (x, 900.0),
                (x, c + 80.0),
                (c + 80.0, c),
                (1000.0, c),
            ]
        }

        (Direction::Up, Route::Left) => {
            let x = c - road_half + lane * 2.5;
            vec![
                (x, 900.0),
                (x, c + 80.0),
                (c, c + 60.0),
                (-100.0, c + 60.0),
            ]
        }

        // ================= DOWN ↓ =================
        (Direction::Down, Route::Straight) => {
            let x = c + road_half - lane * 1.5;
            vec![(x, -100.0), (x, 1000.0)]
        }

        (Direction::Down, Route::Right) => {
            let x = c + road_half - lane * 0.5;
            vec![
                (x, -100.0),
                (x, c - 80.0),
                (c - 80.0, c),
                (-100.0, c),
            ]
        }

        (Direction::Down, Route::Left) => {
            let x = c + road_half - lane * 2.5;
            vec![
                (x, -100.0),
                (x, c - 80.0),
                (c, c - 60.0),
                (1000.0, c - 60.0),
            ]
        }
    }
}


fn lane_center_from_offset(offset: f32) -> f32 {
    CENTER as f32 + offset
}
