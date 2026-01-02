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
    let center = CENTER as f32;
    let road_half = ROAD_WIDTH as f32 / 2.0;
    let lane_center = LANE_WIDTH as f32 / 2.0;
    
    match (dir, route) {
        // From LEFT (right side) - entry lanes
        (Direction::Right, Route::Left) => vec![
            // Start in left turn lane
            (900.0, center + road_half - LANE_WIDTH as f32 * 2.5),  
            // Approach intersection in a straight line
            (center + 200.0, center + road_half - LANE_WIDTH as f32 * 2.5),
            // Turn left
            (center + 60.0, center),
            // Exit going up
            (center + 60.0, -100.0),
        ],
        (Direction::Right, Route::Straight) => vec![
           // Start in straight lane
            (-100.0, center + road_half - LANE_WIDTH as f32 * 1.5),  
            // Go straight through intersection
            (1000.0, center + road_half - LANE_WIDTH as f32 * 1.5),
        ],
        (Direction::Right, Route::Right) => vec![
            // Start in right turn lane
            (900.0, center + road_half - LANE_WIDTH as f32 * 0.5),  
            // Approach intersection in a straight line
            (center + 200.0, center + road_half - LANE_WIDTH as f32 * 0.5),
            // Turn right
            (center, center + 60.0),
            // Exit going down
            (center, 1000.0),
        ],

        // From RIGHT (left side) - entry lanes
        (Direction::Left, Route::Left) => vec![
            // Start in left turn lane
            (-100.0, center - road_half + LANE_WIDTH as f32 * 2.5),  
            // Approach intersection in a straight line
            (center - 200.0, center - road_half + LANE_WIDTH as f32 * 2.5),
            // Turn left
            (center - 60.0, center),
            // Exit going down
            (center - 60.0, 1000.0),
        ],
        (Direction::Left, Route::Straight) => vec![
            // Start in straight lane
            (900.0, center - road_half + LANE_WIDTH as f32 * 1.5),  
            // Go straight through intersection
            (-100.0, center - road_half + LANE_WIDTH as f32 * 1.5),
           
        ],
        (Direction::Left, Route::Right) => vec![
            // Start in right turn lane
            (-100.0, center - road_half + LANE_WIDTH as f32 * 0.5),  
            // Approach intersection in a straight line
            (center - 200.0, center - road_half + LANE_WIDTH as f32 * 0.5),
            // Turn right
            (center, center - 60.0),
            // Exit going up
            (center, -100.0),
        ],

        // From UP (bottom side) - entry lanes
        (Direction::Up, Route::Left) => vec![
            // Start in left turn lane
            (center - road_half + LANE_WIDTH as f32 * 2.5, 900.0),  
            // Approach intersection in a straight line
            (center - road_half + LANE_WIDTH as f32 * 2.5, center + 200.0),
            // Turn left
            (center, center - 60.0),
            // Exit going left
            (-100.0, center - 60.0),
        ],
        (Direction::Up, Route::Straight) => vec![
            // Start in straight lane
            (center - road_half + LANE_WIDTH as f32 * 1.5, 900.0),  
            // Go straight through intersection
            (center - road_half + LANE_WIDTH as f32 * 1.5, -100.0),
        ],
        (Direction::Up, Route::Right) => vec![
            // Start in right turn lane
            (center - road_half + LANE_WIDTH as f32 * 0.5, 900.0),  
            // Approach intersection in a straight line
            (center - road_half + LANE_WIDTH as f32 * 0.5, center + 200.0),
            // Turn right
            (center - 60.0, center),
            // Exit going right
            (1000.0, center - 60.0),
        ],

        // From DOWN (top side) - entry lanes
        (Direction::Down, Route::Left) => vec![
            // Start in left turn lane
            (center + road_half - LANE_WIDTH as f32 * 2.5, -100.0),  
            // Approach intersection in a straight line
            (center + road_half - LANE_WIDTH as f32 * 2.5, center - 200.0),
            // Turn left
            (center, center + 60.0),
            // Exit going right
            (1000.0, center + 60.0),
        ],
        (Direction::Down, Route::Straight) => vec![
            // Start in straight lane
            (center + road_half - LANE_WIDTH as f32 * 1.5, -100.0),  
            // Go straight through intersection
            (center + road_half - LANE_WIDTH as f32 * 1.5, 1000.0),
        ],
        (Direction::Down, Route::Right) => vec![
            // Start in right turn lane
            (center + road_half - LANE_WIDTH as f32 * 0.5, -100.0),  
            // Approach intersection in a straight line
            (center + road_half - LANE_WIDTH as f32 * 0.5, center - 200.0),
            // Turn right
            (center + 60.0, center),
            // Exit going left
            (-100.0, center + 60.0),
        ],
    }
}

fn lane_center_from_offset(offset: f32) -> f32 {
    CENTER as f32 + offset
}
