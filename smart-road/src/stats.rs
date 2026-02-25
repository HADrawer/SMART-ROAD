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

    // ⚠️ Near miss tracking
    pub near_misses: u32,
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
            near_misses: 0,
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
    print_stats_to_console(stats);

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
        .window("📊 Simulation Statistics", 700, 700)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    'stats_loop: loop {
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

        canvas.set_draw_color(Color::RGB(20, 20, 30));
        canvas.clear();

        draw_text_stats(&mut canvas, stats);

        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}

fn draw_text_stats(canvas: &mut Canvas<Window>, stats: &Stats) {
    let mut y;
    let line_height = 30;

    // Title bar
    canvas.set_draw_color(Color::RGB(70, 130, 180));
    canvas.fill_rect(Rect::new(0, 0, 700, 50)).ok();
    y = 70;

    draw_simple_line(canvas, 50, y, "SIMULATION STATISTICS", "", Color::RGB(255, 255, 255));
    y += 40;

    // Runtime
    let runtime_str = format!("{:.2}", stats.runtime);
    draw_simple_line(canvas, 50, y, "Runtime:", &runtime_str, Color::RGB(255, 255, 100));
    draw_simple_line(canvas, 400, y, "seconds", "", Color::RGB(200, 200, 200));
    y += line_height + 10;

    // Directions
    draw_simple_line(canvas, 50, y, "DIRECTIONS", "", Color::RGB(150, 200, 255));
    y += line_height;
    draw_simple_line(canvas, 70, y, "Up (South->North):", &format!("{}", stats.up), Color::RGB(100, 255, 100));
    y += line_height;
    draw_simple_line(canvas, 70, y, "Down (North->South):", &format!("{}", stats.down), Color::RGB(100, 200, 255));
    y += line_height;
    draw_simple_line(canvas, 70, y, "Right (West->East):", &format!("{}", stats.right), Color::RGB(255, 200, 100));
    y += line_height;
    draw_simple_line(canvas, 70, y, "Left (East->West):", &format!("{}", stats.left), Color::RGB(255, 150, 200));
    y += line_height + 10;

    // Routes
    draw_simple_line(canvas, 50, y, "ROUTE TYPES", "", Color::RGB(150, 200, 255));
    y += line_height;
    draw_simple_line(canvas, 70, y, "Right Turns:", &format!("{}", stats.right_turn), Color::RGB(150, 255, 150));
    y += line_height;
    draw_simple_line(canvas, 70, y, "Straight:", &format!("{}", stats.straight), Color::RGB(150, 200, 255));
    y += line_height;
    draw_simple_line(canvas, 70, y, "Left Turns:", &format!("{}", stats.left_turn), Color::RGB(255, 200, 150));
    y += line_height + 10;

    // Physics
    draw_simple_line(canvas, 50, y, "PHYSICS DATA", "", Color::RGB(150, 200, 255));
    y += line_height;
    draw_simple_line(canvas, 70, y, "Total Vehicles:", &format!("{}", stats.total_vehicles), Color::RGB(180, 180, 255));
    y += line_height;

    let total_dist = format!("{:.2}", stats.total_distance / 10.0);
    draw_simple_line(canvas, 70, y, "Total Distance:", &total_dist, Color::RGB(255, 200, 100));
    draw_simple_line(canvas, 400, y, "m", "", Color::RGB(200, 200, 200));
    y += line_height;

    if stats.total_vehicles > 0 {
        let avg_dist = format!("{:.2}", (stats.total_distance / stats.total_vehicles as f32) / 10.0);
        draw_simple_line(canvas, 70, y, "Avg Distance/Vehicle:", &avg_dist, Color::RGB(255, 200, 100));
        draw_simple_line(canvas, 450, y, "m", "", Color::RGB(200, 200, 200));
        y += line_height;
    }

    if stats.avg_intersection_time > 0.0 {
        let avg_time = format!("{:.2}", stats.avg_intersection_time);
        draw_simple_line(canvas, 70, y, "Avg Intersection Time:", &avg_time, Color::RGB(255, 200, 100));
        draw_simple_line(canvas, 450, y, "s", "", Color::RGB(200, 200, 200));
        y += line_height;
    }

    draw_simple_line(canvas, 70, y, "Collisions Avoided:", &format!("{}", stats.collision_avoided), Color::RGB(100, 255, 100));
    y += line_height + 10;

    // ⚠️ Near Misses section
    draw_simple_line(canvas, 50, y, "SAFETY", "", Color::RGB(150, 200, 255));
    y += line_height;

    let near_miss_color = if stats.near_misses == 0 {
        Color::RGB(100, 255, 100) // green if none
    } else {
        Color::RGB(255, 100, 100) // red if any occurred
    };
    draw_simple_line(canvas, 70, y, "Near Misses:", &format!("{}", stats.near_misses), near_miss_color);

    // Footer
    canvas.set_draw_color(Color::RGB(50, 50, 60));
    canvas.fill_rect(Rect::new(0, 670, 700, 30)).ok();
    draw_simple_line(canvas, 150, 678, "Press ESC/Enter/Space to close", "", Color::RGB(150, 150, 150));
}

fn draw_simple_line(canvas: &mut Canvas<Window>, x: i32, y: i32, label: &str, value: &str, color: Color) {
    for (i, ch) in label.chars().enumerate() {
        draw_pixel_char(canvas, x + (i as i32 * 8), y, ch, Color::RGB(200, 200, 200));
    }
    
    if !value.is_empty() {
        let value_x = x + 300;
        for (i, ch) in value.chars().enumerate() {
            draw_pixel_char(canvas, value_x + (i as i32 * 10), y, ch, color);
        }
    }
}

fn draw_pixel_char(canvas: &mut Canvas<Window>, x: i32, y: i32, ch: char, color: Color) {
    canvas.set_draw_color(color);
    
    let pattern = get_char_pattern(ch);
    
    for (row, bits) in pattern.iter().enumerate() {
        for col in 0..5 {
            if (bits >> (4 - col)) & 1 == 1 {
                canvas.fill_rect(Rect::new(
                    x + col * 1,
                    y + row as i32 * 2,
                    1,
                    2,
                )).ok();
            }
        }
    }
}

fn get_char_pattern(ch: char) -> Vec<u8> {
    match ch {
        'A' => vec![0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'B' => vec![0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
        'C' => vec![0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110],
        'D' => vec![0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
        'E' => vec![0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
        'F' => vec![0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
        'G' => vec![0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110],
        'H' => vec![0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'I' => vec![0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111],
        'L' => vec![0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        'M' => vec![0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
        'N' => vec![0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
        'O' => vec![0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'P' => vec![0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
        'R' => vec![0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
        'S' => vec![0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
        'T' => vec![0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        'U' => vec![0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'V' => vec![0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
        'Y' => vec![0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        
        'a' => vec![0b00000, 0b00000, 0b01110, 0b00001, 0b01111, 0b10001, 0b01111],
        'c' => vec![0b00000, 0b00000, 0b01110, 0b10000, 0b10000, 0b10001, 0b01110],
        'd' => vec![0b00001, 0b00001, 0b01111, 0b10001, 0b10001, 0b10001, 0b01111],
        'e' => vec![0b00000, 0b00000, 0b01110, 0b10001, 0b11111, 0b10000, 0b01110],
        'g' => vec![0b00000, 0b01111, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110],
        'h' => vec![0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b10001, 0b10001],
        'i' => vec![0b00100, 0b00000, 0b01100, 0b00100, 0b00100, 0b00100, 0b01110],
        'l' => vec![0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        'm' => vec![0b00000, 0b00000, 0b11010, 0b10101, 0b10101, 0b10001, 0b10001],
        'n' => vec![0b00000, 0b00000, 0b11110, 0b10001, 0b10001, 0b10001, 0b10001],
        'o' => vec![0b00000, 0b00000, 0b01110, 0b10001, 0b10001, 0b10001, 0b01110],
        'p' => vec![0b00000, 0b00000, 0b11110, 0b10001, 0b11110, 0b10000, 0b10000],
        'f' => vec![0b00110, 0b01001, 0b01000, 0b11110, 0b01000, 0b01000, 0b01000],
        'r' => vec![0b00000, 0b00000, 0b10110, 0b11001, 0b10000, 0b10000, 0b10000],
        's' => vec![0b00000, 0b00000, 0b01111, 0b10000, 0b01110, 0b00001, 0b11110],
        't' => vec![0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00100, 0b00011],
        'u' => vec![0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b10011, 0b01101],
        'v' => vec![0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
        'w' => vec![0b00000, 0b00000, 0b10001, 0b10001, 0b10101, 0b11011, 0b10001],
        
        '0' => vec![0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110],
        '1' => vec![0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        '2' => vec![0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111],
        '3' => vec![0b11111, 0b00010, 0b00100, 0b00010, 0b00001, 0b10001, 0b01110],
        '4' => vec![0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
        '5' => vec![0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110],
        '6' => vec![0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
        '7' => vec![0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
        '8' => vec![0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
        '9' => vec![0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100],
        
        ' ' => vec![0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
        ':' => vec![0b00000, 0b00100, 0b00000, 0b00000, 0b00000, 0b00100, 0b00000],
        '.' => vec![0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b00000],
        '/' => vec![0b00001, 0b00010, 0b00010, 0b00100, 0b01000, 0b01000, 0b10000],
        '-' => vec![0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
        '>' => vec![0b01000, 0b00100, 0b00010, 0b00001, 0b00010, 0b00100, 0b01000],
        '(' => vec![0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010],
        ')' => vec![0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000],
        
        _ => vec![0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    }
}

fn print_stats_to_console(stats: &Stats) {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║           📊 FINAL SIMULATION STATISTICS 📊              ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║ 🕒 Runtime: {:<45.2} seconds ║", stats.runtime);
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║                    🚗 DIRECTIONS                          ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║ ⬆️  Up (South→North)     : {:<30} ║", stats.up);
    println!("║ ⬇️  Down (North→South)   : {:<30} ║", stats.down);
    println!("║ ➡️  Right (West→East)     : {:<30} ║", stats.right);
    println!("║ ⬅️  Left (East→West)      : {:<30} ║", stats.left);
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║                   🛣️  ROUTE TYPES                         ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║ ↩️ Right Turns : {:<40} ║", stats.right_turn);
    println!("║ ⬆️ Straight    : {:<40} ║", stats.straight);
    println!("║ ↪️ Left Turns  : {:<40} ║", stats.left_turn);
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║ 🚗 Total Vehicles: {:<39} ║", stats.total_vehicles);
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║                    ⚡ PHYSICS DATA                        ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║ 📏 Total Distance Traveled: {:<26.2} m ║", stats.total_distance / 10.0);
    if stats.total_vehicles > 0 {
        println!("║ 📊 Avg Distance per Vehicle: {:<24.2} m ║", 
            (stats.total_distance / stats.total_vehicles as f32) / 10.0);
    }
    if stats.avg_intersection_time > 0.0 {
        println!("║ ⏱️  Avg Intersection Time: {:<26.2} s ║", stats.avg_intersection_time);
    }
    println!("║ 🛡️  Collisions Avoided: {:<32} ║", stats.collision_avoided);
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║                    ⚠️  SAFETY                             ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║ ⚠️  Near Misses: {:<40} ║", stats.near_misses);
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!("\n✨ Visual statistics window will open momentarily...");
    println!("   Press ESC, Enter, or Space to close the window\n");
}