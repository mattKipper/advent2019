extern crate aoc;

#[derive(Debug,PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}


#[derive(Debug, Clone, PartialEq, Copy)]
struct PathComponent {
    origin: Point,
    direction: Direction,
    distance: i32
}

impl PathComponent {
    fn new(origin: Point, direction: Direction, distance: i32) -> PathComponent {
        PathComponent { origin, direction, distance }
    }

    fn from_str(origin: Point, mut input: String) -> Result<PathComponent, String> {
        let dir = match input.chars().next() {
            Some('U') => Ok(Direction::Up),
            Some('D') => Ok(Direction::Down),
            Some('L') => Ok(Direction::Left),
            Some('R') => Ok(Direction::Right),
            Some(c) => Err(format!("Invalid direction: {}", c)),
            None => Err(String::from("Cannot parse zero-length direction string"))
        };

        match dir {
            Err(s) => Err(s),
            Ok(d) => {
                input.remove(0);
                match input.parse::<i32>() {
                    Ok(m) => Ok(PathComponent::new(origin, d,m)),
                    Err(_) => Err(format!("Invalid magnitude: {}", input))
                }
            }
        }
    }

    fn end(&self) -> Point {
        match self.direction {
            Direction::Up => Point::new(self.origin.x, self.origin.y + self.distance),
            Direction::Down => Point::new(self.origin.x, self.origin.y - self.distance),
            Direction::Right => Point::new(self.origin.x + self.distance, self.origin.y),
            Direction::Left => Point::new(self.origin.x - self.distance, self.origin.y),
        }
    }

    fn start(&self) -> Point {
        self.origin.clone()
    }

    fn min_x(&self) -> i32 {
        match self.direction {
            Direction::Up | Direction::Down | Direction::Right => self.start().x,
            Direction::Left => self.end().x,
        }
    }

    fn max_x(&self) -> i32 {
        match self.direction {
            Direction::Up | Direction::Down | Direction::Left => self.start().x,
            Direction::Right => self.end().x
        }
    }

    fn min_y(&self) -> i32 {
        match self.direction {
            Direction::Left | Direction::Right | Direction::Up  => self.start().y,
            Direction::Down => self.end().y
        }
    }

    fn max_y(&self) -> i32 {
        match self.direction {
            Direction::Left | Direction::Right | Direction::Down=> self.start().y,
            Direction::Up => self.end().y
        }
    }
}

struct Collision {
    first_segment: PathComponent,
    second_segment: PathComponent,
    location: Point
}

fn collision(a: &PathComponent, b: &PathComponent) -> Option<Point> {
    type D = Direction;

    match (a.direction, b.direction) {
        (D::Left, D::Up) | (D::Right, D::Up) | (D::Left, D::Down) | (D::Right, D::Down) => {
            let ay = a.start().y;
            let bx = b.start().x;

            if ay > b.min_y() && ay < b.max_y() && bx > a.min_x() && bx < a.max_x() {
                Some(Point::new(bx,ay))
            }
            else {
                None
            }
        },
        (D::Up, D::Left) | (D::Down, D::Left) | (D::Up, D::Right) | (D::Down, D::Right) => {
            let ax = a.start().x;
            let by = b.start().y;

            if ax > b.min_x() && ax < b.max_x() && by > a.min_y() && by < a.max_y() {
                Some(Point::new(ax,by))
            }
            else {
                None
            }
        },
        _ => None
    }
}

impl Collision {
    fn detect(a: &PathComponent, b: &PathComponent) -> Option<Collision> {
        match collision(a,b) {
            Some(p) => Some(Collision { 
                first_segment: a.clone(),
                second_segment: b.clone(),
                location: p
            }),
            None => None
        }
    }
}


fn taxicab_distance(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[derive(Debug, Clone, PartialEq)]
struct Path {
    segments: Vec<PathComponent>
}

impl Path {
    fn from_str(input: String) -> Result<Path, String> {
        let mut origin = Point::new(0,0);

        // Create a path component, updating the origin as a side-effect.
        // The origin isn't updated if string parsing fails
        let build_component = | s: String | {
            let c = PathComponent::from_str(origin.clone(), s);
            match c {
                Ok(p) => { origin = p.end() }
                _ => {}
            };
            c
        };

        let segments = 
            input.trim().split(',')
                 .map(String::from)
                 .map(build_component)
                 .collect();
        
        match segments {
            Ok(segments) => Ok(Path {segments}),
            Err(s) => Err(s)
        }
    }
}

fn collisions(a: &Path, b: &Path) -> Vec<Collision> {

    let mut collisions: Vec<Collision> = Vec::new();

    for a_seg in &a.segments {
        for b_seg in &b.segments {
            match Collision::detect(a_seg, b_seg) {
                Some(p) => collisions.push(p),
                _ =>  {}
            };
        }
    }

    collisions
}

fn input_paths(input: String) -> Result<(Path,Path), String> {

    let mut lines = input.lines();

    match lines.next() {
        Some(l1) => {
            match lines.next() {
                Some(l2) => {
                    let s1 = String::from(l1);
                    let s2 = String::from(l2);
                    match (Path::from_str(s1), Path::from_str(s2)) {
                        (Ok(p1), Ok(p2)) => Ok((p1, p2)),
                        (Err(s), _) | (_, Err(s)) => Err(s)
                    }
                },
                None => Err(String::from("Second input line not provided."))
            }
        },
        None => Err(String::from("First input line not provided."))
    }
}

fn closest_collision(collisions: &Vec<Collision>) -> Option<i32> {
    let origin = Point::new(0,0);
    collisions.iter().map(|c| taxicab_distance(&origin, &c.location)).min()
}

fn steps_to(path: &Path, segment: &PathComponent, position: &Point) -> Option<i32> {

    let mut steps = 0;

    let reached = path.segments
                      .iter()
                      .any(|s| {
                          if s != segment {
                              steps += s.distance;
                              false
                          }
                          else {
                              steps += match s.direction {
                                  Direction::Up | Direction::Down => {
                                      (position.y - s.start().y).abs()
                                  },
                                  Direction::Left | Direction::Right => {
                                      (position.x - s.start().x).abs()
                                  }
                              };
                              true
                          }
                        });

    if reached { Some(steps) } else { None }
}

fn minimum_steps_to_collision(a: &Path, b: &Path) -> Option<i32> {

    let collisions = collisions(a, b);

    collisions.iter()
              .map(|c| { 
                  let a_steps = steps_to(a, &c.first_segment, &c.location);
                  let b_steps = steps_to(b, &c.second_segment, &c.location);
                  match (a_steps, b_steps) {
                      (Some(sa), Some(sb)) => Some(sa + sb),
                      _ => None
                  }
                })
              .filter(|d| d.is_some())
              .map(|d| d.unwrap())
              .min()
}

fn main()
{
    match aoc::input() {

        Some((input, sub)) => {
            match sub {
                aoc::SubProblem::One => {
                    match input_paths(input) {
                        Ok((p1, p2)) => {
                            let collisions = collisions(&p1, &p2);
                            match closest_collision(&collisions) {
                                Some(d) => println!("{}", d),
                                None => println!("Paths do not collide.")
                            };
                        }
                        Err(s) => {
                            println!("{}", s);
                        }
                    };
                },
                aoc::SubProblem::Two => {
                    match input_paths(input) {
                        Ok((p1, p2)) => {
                            match minimum_steps_to_collision(&p1, &p2) {
                                Some(d) => println!("{}", d),
                                None => println!("Paths do not collide.")
                            };
                        }
                        Err(s) => {
                            println!("{}", s);
                        }
                    };
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
    fn eg1() {
        let a = Path::from_str(String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"));
        let b = Path::from_str(String::from("U62,R66,U55,R34,D71,R55,D58,R83"));
        let collisions = collisions(&a.unwrap(), &b.unwrap());
        assert_eq!(closest_collision(&collisions).unwrap(), 159);
    }
    
    #[test]
    fn eg2() {
        let a = Path::from_str(String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"));
        let b = Path::from_str(String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));
        let collisions = collisions(&a.unwrap(), &b.unwrap());
        assert_eq!(closest_collision(&collisions).unwrap(), 135);
    }

    #[test]
    fn eg1a() {
        let a = Path::from_str(String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"));
        let b = Path::from_str(String::from("U62,R66,U55,R34,D71,R55,D58,R83"));
        assert_eq!(minimum_steps_to_collision(&a.unwrap(), &b.unwrap()).unwrap(), 610);
    }
    
    #[test]
    fn eg2a() {
        let a = Path::from_str(String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"));
        let b = Path::from_str(String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));
        assert_eq!(minimum_steps_to_collision(&a.unwrap(), &b.unwrap()).unwrap(), 410);
    }
}