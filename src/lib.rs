pub mod day1;
pub mod day2;

pub fn run(day: &str) {
    match day {
        "1" => day1::solution(),
        "2" => day2::solution(),
        _ => println!("No day selected")
    }
}
