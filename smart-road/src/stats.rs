// src/stats.rs

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
    println!("📏 Total Distance Traveled: {:.2} m", stats.total_distance / 10.0); // assuming 10px = 1m
    if stats.total_vehicles > 0 {
        println!("📊 Avg Distance per Vehicle: {:.2} m", 
            (stats.total_distance / stats.total_vehicles as f32) / 10.0);
    }
    if stats.avg_intersection_time > 0.0 {
        println!("⏱️  Avg Intersection Time: {:.2} s", stats.avg_intersection_time);
    }
    println!("🛡️  Collisions Avoided: {}", stats.collision_avoided);
    
    println!("=====================================\n");
}