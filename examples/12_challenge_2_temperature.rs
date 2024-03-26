/* Fahrenheit to Celsius converter app */
fn main() {
    let temp_c : f64 = 23.3;

    let temp_f = celsius_to_fahrenheit(temp_c);
    let inv_temp_f = fahrenheit_to_celsius(temp_f);
    println!("{temp_c} C corresponds to {temp_f} F");

    assert_eq!(temp_c as f32, inv_temp_f as f32);

}

fn fahrenheit_to_celsius(temp_f: f64) -> f64 {
    return (temp_f - 32.0) / 1.8;
}

fn celsius_to_fahrenheit(temp_c: f64) -> f64 {
    return (temp_c * 1.8) + 32.0;
}