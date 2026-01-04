use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

// ===== ROAD CONFIG =====
pub const LANE_WIDTH: i32 = 40;
pub const LANES_PER_SIDE: i32 = 3;           // Entry lanes per direction
pub const TOTAL_LANES: i32 = LANES_PER_SIDE * 2; // 6 lanes total per road
pub const ROAD_WIDTH: i32 = LANE_WIDTH * TOTAL_LANES;
pub const CENTER: i32 = 450;
pub const HALF_ROAD: i32 = ROAD_WIDTH / 2;



pub fn draw(canvas: &mut Canvas<Window>) {
    // === Step 1: Entry + Exit Roads ===
    canvas.set_draw_color(Color::RGB(45, 45, 45)); // road asphalt

    // Vertical entry (bottom)
   // Vertical entry (bottom)
canvas.fill_rect(Rect::new(
    CENTER - ROAD_WIDTH / 2,
    CENTER,
    ROAD_WIDTH as u32,
    450u32,                 // ⬅️ u32
)).unwrap();

// Vertical entry (top)
canvas.fill_rect(Rect::new(
    CENTER - ROAD_WIDTH / 2,
    0,
    ROAD_WIDTH as u32,
    (CENTER - 120).max(0) as u32, // safe u32
)).unwrap();

// Horizontal entry (left)
canvas.fill_rect(Rect::new(
    0,
    CENTER - ROAD_WIDTH / 2,
    (CENTER - 120).max(0) as u32,
    ROAD_WIDTH as u32,
)).unwrap();

// Horizontal entry (right)
canvas.fill_rect(Rect::new(
    CENTER,
    CENTER - ROAD_WIDTH / 2,
    450u32,
    ROAD_WIDTH as u32,
)).unwrap();


    // === Step 2: Intersection block (clean, no lines crossing middle) ===
    canvas.set_draw_color(Color::RGB(120, 40, 40)); 
    canvas.fill_rect(Rect::new(
        CENTER - 120,
        CENTER - 120,
        240,
        240,
    )).unwrap();

    // Draw lane dividers after base
    draw_lane_dividers(canvas);
}

fn draw_lane_dividers(canvas: &mut Canvas<Window>) {

    let dash = 25;
    let gap = 25;

    // ===== VERTICAL LANE DIVIDERS =====
    for i in 0..TOTAL_LANES {
        let x = CENTER - ROAD_WIDTH / 2 + (LANE_WIDTH * i);

        // 🚧 Middle bold divider between IN ↔ OUT
        if i == LANES_PER_SIDE {
            canvas.set_draw_color(Color::RGB(255, 255, 255)); // bold white
            canvas.fill_rect(Rect::new(x - 2, 0, 4, 900)).ok();
            continue;
        }

        // normal dashed lane lines
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        for y in (0..900).step_by((dash + gap) as usize) {
            canvas.fill_rect(Rect::new(x, y, 2, dash)).ok();
        }
    }

    // ===== HORIZONTAL LANE DIVIDERS =====
    for i in 0..TOTAL_LANES {
        let y = CENTER - ROAD_WIDTH / 2 + (LANE_WIDTH * i);

        // 🚧 Middle bold divider between IN ↔ OUT
        if i == LANES_PER_SIDE {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.fill_rect(Rect::new(0, y - 2, 900, 4)).ok();
            continue;
        }

        // normal dashed lane lines
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        for x in (0..900).step_by((dash + gap) as usize) {
            canvas.fill_rect(Rect::new(x, y, dash, 2)).ok();
        }
    }
}

// Used by Vehicles to detect center region
pub fn in_intersection(x: f32, y: f32) -> bool {
    x > (CENTER - ROAD_WIDTH) as f32 &&
    x < (CENTER + ROAD_WIDTH) as f32 &&
    y > (CENTER - ROAD_WIDTH) as f32 &&
    y < (CENTER + ROAD_WIDTH) as f32
}
