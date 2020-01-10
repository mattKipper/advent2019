extern crate aoc;

fn input_range(input: String) -> Result<(u32,u32), String> {
    let mut input = input.trim().split('-');

    match input.next() {
        Some(low) => {
            match input.next() {
                Some(high) =>{
                    match (low.parse::<u32>(), high.parse::<u32>()) {
                        (Ok(low), Ok(high)) => Ok((low, high)),
                        (Err(_), _) => Err(format!("Failed to parse range min: {}", low)),
                        (Ok(_), Err(_)) => Err(format!("Failed to parse range max: {}", high))
                    }
                },
                None => Err(String::from("No range max provided"))
            }
        },
        None => Err(String::from("No range min provided"))
    }
}

fn valid_password(mut password: u32) -> bool {

    let mut has_double = false;

    let mut last_digit = password % 10;
    password /= 10;

    while password != 0 {
        let next_digit = password % 10;

        if next_digit > last_digit {
            return false;
        }

        if last_digit == next_digit {
            has_double = true;
        }

        last_digit = next_digit;
        password /= 10;
    }

    return has_double;
}

fn valid_alternate_password(mut password: u32) -> bool {

    let mut has_double = false;
    let mut consecutive_digits = 1;

    let mut last_digit = password % 10;
    password /= 10;

    while password != 0 {

        let next_digit = password % 10;

        if next_digit > last_digit {
            return false;
        }

        if !has_double {

            if next_digit == last_digit {
                consecutive_digits += 1;
            }
            else {
                if consecutive_digits == 2 {
                    has_double = true;
                }
                consecutive_digits = 1;
            }
        }

        last_digit = next_digit;
        password /= 10;

    }

    has_double || (consecutive_digits == 2)
}

fn valid_passwords(min: u32, max:u32) -> Vec<u32> {
    (min..max+1).filter(|x| valid_password(x.clone())).collect()
}

fn valid_alternate_passwords(min: u32, max:u32) -> Vec<u32> {
    (min..max+1).filter(|x| valid_alternate_password(x.clone())).collect()
}

fn main()
{
    match aoc::input() {

        Some((input, sub)) => {

            match sub {
                aoc::SubProblem::One => {
                    match input_range(input) {
                        Ok((min,max)) => {
                            println!("{}", valid_passwords(min,max).len());
                            std::process::exit(0);
                        }
                        Err(s) => {
                            println!("{}", s);
                            std::process::exit(1);
                        }
                    }
                },
                aoc::SubProblem::Two => {
                    match input_range(input) {
                        Ok((min,max)) => {
                            println!("{}", valid_alternate_passwords(min,max).len());
                            std::process::exit(0);
                        }
                        Err(s) => {
                            println!("{}", s);
                            std::process::exit(1);
                        }
                    }
                }
            };
        },
        None => {
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eg1() {
        assert!(valid_password(111111));
    }
    #[test]
    fn eg2() {
        assert!(!valid_password(223450));
    }
    #[test]
    fn eg3() {
        assert!(!valid_password(123789));
    }

    #[test]
    fn eg1a() {
        assert!(valid_alternate_password(112233));
    }

    #[test]
    fn eg2a() {
        assert!(!valid_alternate_password(123444));
    }

    #[test]
    fn eg3a() {
        assert!(valid_alternate_password(111122));
    }
}