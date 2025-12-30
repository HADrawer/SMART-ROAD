use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

// ==== CONSTANTS ====
pub const CENTER: i32 = 450;
pub const ROAD_WIDTH: i32 = 180;
pub const LANE_WIDTH: i32 = ROAD_WIDTH / 3;

// Intersection zone for turning
pub const TURN_MIN: f32 = (CENTER - 50) as f32;
pub const TURN_MAX: f32 = (CENTER + 50) as f32;

pub fn draw(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(45, 45, 45));

    // Vertical Road
    canvas.fill_rect(Rect::new(CENTER - ROAD_WIDTH / 2, 0, ROAD_WIDTH as u32, 900)).unwrap();

    // Horizontal Road
    canvas.fill_rect(Rect::new(0, CENTER - ROAD_WIDTH / 2, 900, ROAD_WIDTH as u32)).unwrap();

    // ---- Lane stripes ----
    canvas.set_draw_color(Color::RGB(200, 200, 200));

    // Vertical dashed lines
    for x_offset in [-LANE_WIDTH, 0, LANE_WIDTH] {
        for y in (0..900).step_by(60) {
            canvas.fill_rect(Rect::new(CENTER + x_offset - 2, y, 4, 30)).unwrap();
        }
    }

    // Horizontal dashed lines
    for y_offset in [-LANE_WIDTH, 0, LANE_WIDTH] {
        for x in (0..900).step_by(60) {
            canvas.fill_rect(Rect::new(x, CENTER + y_offset - 2, 30, 4)).unwrap();
        }
    }
}

// Check if car is in center intersection area
pub fn in_intersection(x: f32, y: f32) -> bool {
    x > TURN_MIN && x < TURN_MAX && y > TURN_MIN && y < TURN_MAX
}
