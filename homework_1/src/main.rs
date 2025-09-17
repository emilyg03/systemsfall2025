const FREEZING_FAHRENHEIT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_FAHRENHEIT) * 5.0 / 9.0
}

fn main() {
    let temp_f: f64 = 32.0;
    println!("{}°F is {:.2}°C", temp_f, fahrenheit_to_celsius(temp_f));

    for i in 1..=5 {
        let next_temp_f = temp_f + i as f64;
        println!("{}°F is {:.2}°C", next_temp_f, fahrenheit_to_celsius(next_temp_f));
    }
}