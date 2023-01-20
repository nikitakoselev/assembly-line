// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PER_HOUR_SPEED_1: f64 = 221f64;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut result: f64 = 0f64;

    match speed {
        1 | 2 | 3 | 4 => result = CARS_PER_HOUR_SPEED_1 * speed as f64,
        5 | 6 | 7 | 8 => result = CARS_PER_HOUR_SPEED_1 * speed as f64 * 0.9,
        _ => result = CARS_PER_HOUR_SPEED_1 * speed as f64 * 0.77
    }

    result
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    ((production_rate_per_hour(speed)) / (60 as f64)) as u32
}
