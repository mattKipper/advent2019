extern crate aoc;
#[macro_use] extern crate ego_tree;

use std::str::FromStr;
use ego_tree::{Tree,NodeMut,NodeRef};

#[derive(Debug)]
struct Orbit {
    primary: String,
    satellite: String
}

impl FromStr for Orbit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut bodies = s.split(')');

        match bodies.next() {
            Some(primary) => {
                match bodies.next() {
                    Some(satellite) => {
                        Ok(
                            Orbit {
                                primary: String::from(primary),
                                satellite: String::from(satellite)
                            }
                        )
                    },
                    None => {
                        Err(format!("Cannot parse satellite from {}", s))
                    }
                }
            },
            None => Err(format!("Cannot parse primary from \"{}\"", s))
        }
    }
}


fn orbits(input: String) -> Result<Vec<Orbit>, String> {
    input.lines().map(|s| s.trim()).map(Orbit::from_str).collect()
}

fn build_tree(orbits: &Vec<Orbit>, mut root: NodeMut<String> ) {
    // `orbits` is sorted by orbit primary, so search for the start
    // of the slice with the root as its primary. The binary search
    // might return an index in the slice other than the first, so
    // iterate backwards until a non-matching primary is found and
    // return the index
    let start = match orbits.binary_search_by(|o| o.primary.cmp(root.value())) {
        Ok(mut index) => { // At least one orbit
            let mut iter = orbits.iter().rev().skip(orbits.len() - index);
            while let Some(v) = iter.next() {
                if &v.primary == root.value() {
                    index = index - 1;
                }
                else {
                    break;
                }
            }
            index
        },
        Err(_) => { // No orbits. Nothing to do.
            return;
        }
    };

    assert_eq!(root.value(), &orbits[start].primary);
    if start != 0 {
        assert_ne!(root.value(), &orbits[start - 1].primary);

    }

    let mut iter = orbits.iter().skip(start);
    while let Some(orbit) = iter.next() {
        if &orbit.primary == root.value() {
            let child = root.append(orbit.satellite.clone());
            build_tree(orbits, child);
        }
    }
}

fn create_orbit_tree(mut orbits: Vec<Orbit>) -> Tree<String> {
    orbits.sort_by(|a,b| a.primary.cmp(&b.primary));

    let mut tree = Tree::new(String::from("COM"));
    build_tree(&orbits, tree.root_mut());

    tree
}

fn do_orbit_count(root_orbit: NodeRef<String>, root_depth: usize) -> usize {
    let mut count = root_depth;
    for satellite in root_orbit.children() {
        count = count + do_orbit_count(satellite, root_depth + 1);
    }
    count
}

fn orbit_count(orbits: &Tree<String>) -> usize {
    do_orbit_count(orbits.root(), 0)
}

fn main()
{
    match aoc::input() {

        Some((input, sub)) => {
            match orbits(input) {
                Ok(orbits) => {
                    match sub {
                        _ => {
                            let tree = create_orbit_tree(orbits);
                            println!("{}", orbit_count(&tree));
                            std::process::exit(0);
                        }
                    }
                },
                Err(s) => {
                    println!("{}", s);
                    std::process::exit(1);
                }
            }
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
    fn eg() {
        let input = String::from(
            "COM)B
             B)C
             C)D
             D)E
             E)F
             B)G
             G)H
             D)I
             E)J
             J)K
             K)L");
        
        let o = orbits(input);
        let tree = create_orbit_tree(o.unwrap());
        assert_eq!(orbit_count(&tree), 42);
    }
}


