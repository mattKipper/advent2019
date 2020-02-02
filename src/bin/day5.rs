extern crate aoc;
use std::io;
fn main()
{
    match aoc::input() {

        Some((input, sub)) => {

            let mut program = aoc::intcode::parse_input(input);

            match sub {
                _ => {
                    let stdin = io::stdin();
                    let mut stdout = io::stdout();

                    aoc::intcode::execute(&mut program, &mut stdin.lock(), &mut stdout);
                    std::process::exit(0);
                }
            };
        },
        None => {
            std::process::exit(1);
        }
    }
}
