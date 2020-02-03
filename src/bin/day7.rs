extern crate aoc;
use aoc::intcode;

fn generate_permutations(k: usize, input: &mut Vec<i64>, output: &mut Vec<Vec<i64>>)
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

fn phase_permutations(stages: usize) -> Vec<Vec<i64>>
{
    let mut seed: Vec<i64> = (0..stages as i64).collect();
    let mut out: Vec<Vec<i64>> = Vec::new();

    generate_permutations(stages, &mut seed, &mut out);

    out
}


fn main()
{
    match aoc::input() {

        Some((input,_)) => {
            let program = intcode::parse_input(input);

            let mut max_output: i64 = 0;
            let phase_perms = phase_permutations(5);

            for phases in phase_perms {

                let mut stage_input = 0;
                let mut output = aoc::intcode::OutputCollector::new();

                for phase in phases {
                    let mut program = program.clone();
                    let mut input = intcode::InputProvider::new(vec![phase,stage_input]);
                    intcode::execute(&mut program, &mut input, &mut output);
                    
                    stage_input = output.outputs.last().unwrap().clone();
                }

                max_output = std::cmp::max(max_output, output.outputs.last().unwrap().clone());
            }

            println!("{}", max_output);

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