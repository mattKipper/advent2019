extern crate aoc;
use std::io;

fn solve(program: &mut Vec<i64>) -> i64 {
    program[1] = 12;
    program[2] = 2;

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    aoc::intcode::execute(program, &mut stdin.lock(), &mut stdout);

    program[0]
}

fn solve_alt(program: &mut Vec<i64>) -> i64 {
    for noun in 1..100 {
        for verb in 1..100 {

            let mut p = program.clone();
            p[1] = noun;
            p[2] = verb;

            let stdin = io::stdin();
            let mut stdout = io::stdout();
            aoc::intcode::execute(&mut p, &mut stdin.lock(), &mut stdout);

            if p[0] == 19690720 {
                return (100 * noun) + verb;
            }

        }
    }

    panic!("No answer found")
}

fn main()
{
    match aoc::input() {

        Some((input, sub)) => {

            let mut program = aoc::intcode::parse_input(input);

            match sub {
                aoc::SubProblem::One => {
                    println!("{}", solve(&mut program))
                },
                aoc::SubProblem::Two => {
                    println!("{}", solve_alt(&mut program))
                }
            };
            std::process::exit(0);
        },
        None => {
            std::process::exit(1);
        }
    }
}
