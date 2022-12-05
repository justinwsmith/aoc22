mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    //println!("Day1: {}", day1::solve());
    //println!("Day1-2: {}", day1::solve2());
    //println!("Day2: {}", day2::solve());
    //println!("Day2-2: {}", day2::solve2());

    let day3_input = include_str!("../inputs/day3.txt");
    println!("Day3: {}", day3::solve(day3_input));
    println!("Day3-2: {}", day3::solve2(day3_input));
}
