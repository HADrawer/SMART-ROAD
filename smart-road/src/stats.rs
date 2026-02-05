// src/stats.rs

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Debug)]
pub struct Stats {
    pub total_vehicles: u32,
    pub up: u32,
    pub down: u32,
    pub left: u32,
    pub right: u32,
    pub left_turn: u32,
    pub straight: u32,
    pub right_turn: u32,
    pub runtime: f32,
    
    // 📊 Physics tracking
    pub total_distance: f32,
    pub avg_intersection_time: f32,
    pub collision_avoided: u32,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            total_vehicles: 0,
            up: 0,
            down: 0,
            left: 0,
            right: 0,
            left_turn: 0,
            straight: 0,
            right_turn: 0,
            runtime: 0.0,
            total_distance: 0.0,
            avg_intersection_time: 0.0,
            collision_avoided: 0,
        }
    }

    /// Update statistics from current vehicle state
    pub fn update_from_vehicles(&mut self, vehicles: &[crate::vehicle::Vehicle]) {
        // Calculate total distance from all vehicles
        self.total_distance = vehicles.iter()
            .map(|v| v.distance_traveled)
            .sum();

        // Calculate average intersection time
        let intersection_times: Vec<f32> = vehicles.iter()
            .filter_map(|v| {
                let time = v.get_intersection_time();
                if time > 0.0 { Some(time) } else { None }
            })
            .collect();

        if !intersection_times.is_empty() {
            self.avg_intersection_time = intersection_times.iter().sum::<f32>() 
                / intersection_times.len() as f32;
        }
    }
}

/// Display statistics in an overlay on the existing canvas
pub fn show_stats_overlay(canvas: &mut Canvas<Window>, stats: &Stats) {
    // Semi-transparent background
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 220));
    canvas.fill_rect(Rect::new(50, 50, 800, 800)).unwrap();
    
    // Border
    canvas.set_draw_color(Color::RGB(100, 150, 200));
    for i in 0..3 {
        canvas.draw_rect(Rect::new(50 + i, 50 + i, 800 - i as u32 * 2, 800 - i as u32 * 2)).ok();
    }

    let mut y = 80;
    let line_height = 35;

    // Title area
    canvas.set_draw_color(Color::RGB(50, 100, 150));
    canvas.fill_rect(Rect::new(60, y, 780, 50)).unwrap();
    y += 70;

    // Runtime
    canvas.set_draw_color(Color::RGB(70, 130, 180));
    canvas.fill_rect(Rect::new(70, y, 760, line_height as u32)).unwrap();
    y += line_height + 15;

    // Directions header
    canvas.set_draw_color(Color::RGB(60, 60, 80));
    canvas.fill_rect(Rect::new(70, y, 760, 30)).unwrap();
    y += 40;

    // Direction bars
    let max_direction = stats.up.max(stats.down).max(stats.left).max(stats.right).max(1);
    
    draw_stat_bar(canvas, 90, y, stats.up, max_direction, Color::RGB(100, 200, 100));
    y += line_height;
    
    draw_stat_bar(canvas, 90, y, stats.down, max_direction, Color::RGB(100, 150, 200));
    y += line_height;
    
    draw_stat_bar(canvas, 90, y, stats.right, max_direction, Color::RGB(200, 150, 100));
    y += line_height;
    
    draw_stat_bar(canvas, 90, y, stats.left, max_direction, Color::RGB(200, 100, 150));
    y += line_height + 15;

    // Routes header
    canvas.set_draw_color(Color::RGB(60, 60, 80));
    canvas.fill_rect(Rect::new(70, y, 760, 30)).unwrap();
    y += 40;

    let max_route = stats.left_turn.max(stats.straight).max(stats.right_turn).max(1);
    
    draw_stat_bar(canvas, 90, y, stats.right_turn, max_route, Color::RGB(150, 200, 150));
    y += line_height;
    
    draw_stat_bar(canvas, 90, y, stats.straight, max_route, Color::RGB(150, 150, 200));
    y += line_height;
    
    draw_stat_bar(canvas, 90, y, stats.left_turn, max_route, Color::RGB(200, 150, 150));
    y += line_height + 15;

    // Physics header
    canvas.set_draw_color(Color::RGB(60, 60, 80));
    canvas.fill_rect(Rect::new(70, y, 760, 30)).unwrap();
    y += 40;

    // Physics stats (just boxes for now)
    for _ in 0..5 {
        canvas.set_draw_color(Color::RGB(80, 80, 100));
        canvas.fill_rect(Rect::new(90, y, 740, 30)).unwrap();
        y += 40;
    }

    // Footer with instruction
    canvas.set_draw_color(Color::RGB(50, 100, 150));
    canvas.fill_rect(Rect::new(60, 800, 780, 40)).unwrap();
}

fn draw_stat_bar(
    canvas: &mut Canvas<Window>,
    x: i32,
    y: i32,
    value: u32,
    max_value: u32,
    color: Color,
) {
    // Background
    canvas.set_draw_color(Color::RGB(50, 50, 60));
    canvas.fill_rect(Rect::new(x, y, 740, 25)).unwrap();

    // Value bar
    let bar_width = if max_value > 0 {
        ((value as f32 / max_value as f32) * 650.0) as u32
    } else {
        0
    };
    
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(x + 5, y + 3, bar_width.max(1), 19)).unwrap();

    // Border
    canvas.set_draw_color(Color::RGB(100, 100, 120));
    canvas.draw_rect(Rect::new(x, y, 740, 25)).ok();
}

pub fn show_stats_window(stats: &Stats) {
    println!("\n=====================================");
    println!("📊  FINAL SIMULATION STATISTICS");
    println!("=====================================");
    println!("🕒 Runtime: {:.2} seconds", stats.runtime);

    println!("\n🚗 Directions:");
    println!("⬆️  Up (South→North)     : {}", stats.up);
    println!("⬇️  Down (North→South)   : {}", stats.down);
    println!("➡️  Right (West→East)     : {}", stats.right);
    println!("⬅️  Left (East→West)      : {}", stats.left);

    println!("\n🛣️  Route Types:");
    println!("↩️ Right Turns : {}", stats.right_turn);
    println!("⬆️ Straight    : {}", stats.straight);
    println!("⬅️ Left Turns  : {}", stats.left_turn);

    println!("\n🚗 Total Vehicles: {}", stats.total_vehicles);
    
    println!("\n⚡ Physics Data:");
    println!("📏 Total Distance Traveled: {:.2} m", stats.total_distance / 10.0);
    if stats.total_vehicles > 0 {
        println!("📊 Avg Distance per Vehicle: {:.2} m", 
            (stats.total_distance / stats.total_vehicles as f32) / 10.0);
    }
    if stats.avg_intersection_time > 0.0 {
        println!("⏱️  Avg Intersection Time: {:.2} s", stats.avg_intersection_time);
    }
    println!("🛡️  Collisions Avoided: {}", stats.collision_avoided);
    
    println!("=====================================");
    println!("Statistics displayed on screen - press any key to continue");
    println!("=====================================\n");
}
