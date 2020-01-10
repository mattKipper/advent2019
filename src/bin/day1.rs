extern crate aoc;

use std::str::Lines;

fn required_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn required_fuel_cumulative(mass: i32) -> i32 {

    let mut total_fuel = 0;

    let mut fuel = required_fuel(mass);

    while fuel > 0 {
        total_fuel += fuel;
        fuel = required_fuel(fuel);
    }

    total_fuel
}

fn fuel_sum(masses: Lines) -> i32 {
    masses
        .map(|mass| {
            mass.parse::<i32>().unwrap()
        })
        .map(required_fuel)
        .sum()
}

fn fuel_sum_alt(masses: Lines) -> i32 {
    masses
        .map(|mass| {
            mass.parse::<i32>().unwrap()
        })
        .map(required_fuel_cumulative)
        .sum()
}

fn main()
{
    match aoc::input() {

        Some((s, sub)) => {
            match sub {
                aoc::SubProblem::One => {
                    println!("{}", fuel_sum(s.lines()));
                },
                aoc::SubProblem::Two => {
                    println!("{}", fuel_sum_alt(s.lines()));
                }
            };

            std::process::exit(0);
        },
        None => {
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(required_fuel(12), 2);
    }
    #[test]
    fn example2() {
        assert_eq!(required_fuel(14), 2);
    }
    #[test]
    fn example3() {
        assert_eq!(required_fuel(1969), 654);
    }
    #[test]
    fn example4() {
        assert_eq!(required_fuel(100756), 33583);
    }
    #[test]
    fn example2a() {
        assert_eq!(required_fuel_cumulative(14), 2);
    }
    #[test]
    fn example3a() {
        assert_eq!(required_fuel_cumulative(1969), 966);
    }
    #[test]
    fn example4a() {
        assert_eq!(required_fuel_cumulative(100756), 50346);
    }
}