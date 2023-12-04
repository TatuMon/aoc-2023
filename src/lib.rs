pub mod day1;

pub fn run(day: i8) {
    match day {
        1 => day1::solution(),
        _ => println!("No day selected")
    }
}
