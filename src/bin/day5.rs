extern crate aoc;
fn main()
{
    match aoc::input() {

        Some((input, sub)) => {

            let mut program = aoc::intcode::parse_input(input);

            match sub {
                _ => {
                    aoc::intcode::execute(&mut program);
                    std::process::exit(0);
                }
            };
        },
        None => {
            std::process::exit(1);
        }
    }
}
