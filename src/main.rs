use std::io;
fn main() {
    
  loop{
    let mut temperature_type_str = String::new();
    let mut temperature = String::new();
    let mut temperature_num_str = String::new();

    println!("Press Ctrl+c to exit");

    println!("Type 'F' if you are using Fahrenheit or 'C' if you are using Celsius as a starting value: ");
    io::stdin()
        .read_line(&mut temperature_type_str)
        .expect("Invalid input");

    let temperature_type = temperature_type_str.trim().chars().nth(0).expect("no byte read");

    println!("Type in the temperature value you want to convert: ");
    io::stdin()
        .read_line(&mut temperature_num_str)
        .expect("Invalid input");
    
    let temperature_num: f32 = temperature_num_str.trim().parse().unwrap();
    println!("Your input: {temperature_type} {temperature_num}");


    match temperature_type {
        'F'=>to_celsius(temperature_num),
        'C'=>to_fahrenheit(temperature_num),
        _ => println!("invalid input"),
    }
    
}}

fn to_celsius(temperature: f32){
    println!("{}°F equals {}°C", temperature, ((temperature - 32.0)/1.8));
}
fn to_fahrenheit(temperature: f32){
    println!("{}°C equals {}°F", temperature, ((temperature*9.0)/5.0) + 32.0);
}
