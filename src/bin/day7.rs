extern crate aoc;
use std::io;

fn generate_permutations(k: usize, input: &mut Vec<usize>, output: &mut Vec<Vec<usize>>)
{
    match k {
        1 => output.push(input.clone()),
        k => {
            generate_permutations(k - 1, input, output);

            for i in 0..k-1 {

                if k % 2 == 0 {
                    input.swap(i, k-1);
                }
                else {
                    input.swap(0, k-1);
                }

                generate_permutations(k-1, input, output);
            }
        }
    };
}

fn phase_permutations(stages: usize) -> Vec<Vec<usize>>
{
    let mut seed: Vec<usize> = (0..stages).collect();
    let mut out: Vec<Vec<usize>> = Vec::new();

    generate_permutations(stages, &mut seed, &mut out);

    out
}

fn main()
{
    match aoc::input() {

        Some((input,_)) => {
            let mut program = aoc::intcode::parse_input(input);
            let stdin = io::stdin();
            let mut stdout = io::stdout();

            aoc::intcode::execute(&mut program, &mut stdin.lock(), &mut stdout);
            std::process::exit(0);
        },
        None => {
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate factorial;
    extern crate itertools;
    
    use factorial::Factorial;
    use itertools::Itertools;

    #[test]
    fn permutations() {
        for n in 1..5 {
            let perms = phase_permutations(n);
            let len = perms.len();
            assert_eq!(len, n.factorial());
            assert_eq!(perms.into_iter()
                         .unique()
                         .collect::<Vec<Vec<usize>>>()
                         .len(),
                        len);
        }
    }
}