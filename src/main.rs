pub mod utils;
pub mod day1;
pub mod day2;

fn main() {
    day1();
    day2();
}

fn day2() {
    day2::part1("data/day2/input.txt");
    day2::part2("data/day2/input.txt");
}

fn day1() {
    day1::part1("data/day1/input.txt");
    day1::part2("data/day1/input.txt");
}