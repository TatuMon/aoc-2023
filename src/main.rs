fn main() {
    let day_input = std::env::args().nth(1).unwrap_or(String::from("2"));

    println!("Running solution for day: {}", day_input);
    aoc_2023::run(&day_input);
}
