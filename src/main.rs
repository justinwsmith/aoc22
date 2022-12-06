mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    //println!("Day1: {}", day1::solve());
    //println!("Day1-2: {}", day1::solve2());
    //println!("Day2: {}", day2::solve());
    //println!("Day2-2: {}", day2::solve2());

    //let day3_input = include_str!("../inputs/day3.txt");
    //println!("Day3: {}", day3::solve(day3_input));
    //println!("Day3-2: {}", day3::solve2(day3_input));

    //let day4_input = include_str!("../inputs/day4.txt");
    //println!("Day4: {}", day4::solve(day4_input));
    //println!("Day4-2: {}", day4::solve2(day4_input));

    //let day5_input = include_str!("../inputs/day5.txt");
    //println!("Day5: {}", day5::solve(day5_input));
    //println!("Day5-2: {}", day5::solve2(day5_input));

    let day6_input = include_str!("../inputs/day6.txt");
    println!("Day6: {}", day6::solve(day6_input));
    println!("Day6-2: {}", day6::solve2(day6_input));
}
