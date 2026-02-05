// src/stats.rs

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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

pub fn show_stats_window(stats: &Stats, events: &mut sdl2::EventPump) {
    // First print to console as backup
    print_stats_to_console(stats);

    // Create SDL window for stats
    match create_stats_gui(stats, events) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to create stats window: {}", e);
            eprintln!("Stats printed to console above.");
        }
    }
}

fn create_stats_gui(stats: &Stats, events: &mut sdl2::EventPump) -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    let window = video
        .window("📊 Simulation Statistics", 600, 700)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    'stats_loop: loop {
        // Handle events
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape | Keycode::Return | Keycode::Space),
                    ..
                } => break 'stats_loop,
                _ => {}
            }
        }

        // Draw background
        canvas.set_draw_color(Color::RGB(30, 30, 40));
        canvas.clear();

        // Draw title background
        canvas.set_draw_color(Color::RGB(50, 100, 150));
        canvas.fill_rect(Rect::new(0, 0, 600, 60)).ok();

        // Draw sections
        draw_stats_content(&mut canvas, stats);

        // Draw footer
        canvas.set_draw_color(Color::RGB(40, 40, 50));
        canvas.fill_rect(Rect::new(0, 660, 600, 40)).ok();

        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}

fn draw_stats_content(canvas: &mut Canvas<Window>, stats: &Stats) {
    let line_height = 35;
    let mut y = 80;

    // Runtime section
    canvas.set_draw_color(Color::RGB(70, 130, 180));
    canvas.fill_rect(Rect::new(20, y, 560, line_height as u32)).ok();
    y += line_height + 10;

    // Directions section
    canvas.set_draw_color(Color::RGB(60, 60, 80));
    canvas.fill_rect(Rect::new(20, y, 560, 30)).ok();
    y += 40;

    // Direction bars
    let max_direction = stats.up.max(stats.down).max(stats.left).max(stats.right).max(1);
    
    // Up
    draw_stat_bar(canvas, 40, y, "Up (S→N)", stats.up, max_direction, Color::RGB(100, 200, 100));
    y += line_height;
    
    // Down
    draw_stat_bar(canvas, 40, y, "Down (N→S)", stats.down, max_direction, Color::RGB(100, 150, 200));
    y += line_height;
    
    // Right
    draw_stat_bar(canvas, 40, y, "Right (W→E)", stats.right, max_direction, Color::RGB(200, 150, 100));
    y += line_height;
    
    // Left
    draw_stat_bar(canvas, 40, y, "Left (E→W)", stats.left, max_direction, Color::RGB(200, 100, 150));
    y += line_height + 15;

    // Route Types section
    canvas.set_draw_color(Color::RGB(60, 60, 80));
    canvas.fill_rect(Rect::new(20, y, 560, 30)).ok();
    y += 40;

    let max_route = stats.left_turn.max(stats.straight).max(stats.right_turn).max(1);
    
    // Right turns
    draw_stat_bar(canvas, 40, y, "Right Turns", stats.right_turn, max_route, Color::RGB(150, 200, 150));
    y += line_height;
    
    // Straight
    draw_stat_bar(canvas, 40, y, "Straight", stats.straight, max_route, Color::RGB(150, 150, 200));
    y += line_height;
    
    // Left turns
    draw_stat_bar(canvas, 40, y, "Left Turns", stats.left_turn, max_route, Color::RGB(200, 150, 150));
    y += line_height + 15;

    // Physics section
    canvas.set_draw_color(Color::RGB(60, 60, 80));
    canvas.fill_rect(Rect::new(20, y, 560, 30)).ok();
    y += 40;

    // Total vehicles
    canvas.set_draw_color(Color::RGB(80, 80, 100));
    canvas.fill_rect(Rect::new(40, y, 520, 30)).ok();
    y += 40;

    // Distance
    canvas.set_draw_color(Color::RGB(80, 80, 100));
    canvas.fill_rect(Rect::new(40, y, 520, 30)).ok();
    y += 40;

    // Avg distance per vehicle
    if stats.total_vehicles > 0 {
        canvas.set_draw_color(Color::RGB(80, 80, 100));
        canvas.fill_rect(Rect::new(40, y, 520, 30)).ok();
        y += 40;
    }

    // Intersection time
    if stats.avg_intersection_time > 0.0 {
        canvas.set_draw_color(Color::RGB(80, 80, 100));
        canvas.fill_rect(Rect::new(40, y, 520, 30)).ok();
        y += 40;
    }

    // Collisions avoided
    canvas.set_draw_color(Color::RGB(80, 80, 100));
    canvas.fill_rect(Rect::new(40, y, 520, 30)).ok();
}

fn draw_stat_bar(
    canvas: &mut Canvas<Window>,
    x: i32,
    y: i32,
    _label: &str,
    value: u32,
    max_value: u32,
    color: Color,
) {
    // Background bar
    canvas.set_draw_color(Color::RGB(50, 50, 60));
    canvas.fill_rect(Rect::new(x, y, 520, 25)).ok();

    // Value bar
    let bar_width = if max_value > 0 {
        ((value as f32 / max_value as f32) * 400.0) as u32
    } else {
        0
    };
    
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(x + 5, y + 3, bar_width.max(1), 19)).ok();

    // Border
    canvas.set_draw_color(Color::RGB(100, 100, 120));
    canvas.draw_rect(Rect::new(x, y, 520, 25)).ok();
}

fn print_stats_to_console(stats: &Stats) {
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
    println!("Press ESC, Enter, or Space to close the stats window");
    println!("=====================================\n");
}
