use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Polar {
    pub angle: f32,
    pub dist: f32,
}

impl PartialOrd for Polar {
    fn partial_cmp(&self, other: &Polar) -> Option<Ordering> {
        let delta = 1e-4;
        if (self.angle - other.angle).abs() < delta {
            // println!(
            //     "cmp1: {:?} {:?} {:?}",
            //     self,
            //     other,
            //     self.dist.partial_cmp(&other.dist)
            // );
            self.dist.partial_cmp(&other.dist)
        } else {
            // println!(
            //     "cmp2: {:?} {:?} {:?}",
            //     self,
            //     other,
            //     self.angle.partial_cmp(&other.angle)
            // );

            self.angle.partial_cmp(&other.angle)
        }
    }
}
#[derive(Clone, Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    pub fn to_char(&self) -> char {
        match self {
            Dir::Up => '^',
            Dir::Right => '>',
            Dir::Down => 'v',
            Dir::Left => '<',
        }
    }
    // pub fn to_point(&self) -> Point {
    //     self.into()
    // }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum GeoDir {
    North,
    South,
    West,
    East,
}

impl std::str::FromStr for GeoDir {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "north" => Ok(GeoDir::North),
            "south" => Ok(GeoDir::South),
            "west" => Ok(GeoDir::West),
            "east" => Ok(GeoDir::East),
            _ => Err(()),
        }
    }
}

pub trait TurnDir {
    fn turn_right(&self) -> Self;
    fn turn_left(&self) -> Self;
}

impl TurnDir for Dir {
    fn turn_right(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
    fn turn_left(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        }
    }
}
impl From<GeoDir> for Dir {
    fn from(d: GeoDir) -> Dir {
        match d {
            GeoDir::North => Dir::Up,
            GeoDir::South => Dir::Down,
            GeoDir::West => Dir::Left,
            GeoDir::East => Dir::Right,
        }
    }
}
impl From<char> for Dir {
    fn from(c: char) -> Dir {
        match c {
            '^' => Dir::Up,
            '>' => Dir::Right,
            'v' => Dir::Down,
            '<' => Dir::Left,
            _ => panic!("bad direction {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn zero() -> Self {
        Point { x: 0, y: 0 }
    }
    pub fn up() -> Self {
        Point { x: 0, y: -1 }
    }
    pub fn down() -> Self {
        Point { x: 0, y: 1 }
    }
    pub fn left() -> Self {
        Point { x: -1, y: 0 }
    }
    pub fn right() -> Self {
        Point { x: 1, y: 0 }
    }
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn move_into(&self, dir: &Dir) -> Point {
        *self + dir.into()
    }
    pub fn minimize(&self) -> Point {
        let mut p = self.clone();
        let primes = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        for prime in primes.iter() {
            while p.x % prime == 0 && p.y % prime == 0 {
                // println!("prime: {:?} {}", p, prime);

                p.x /= prime;
                p.y /= prime;
            }
        }
        // println!("{:?} -> {:?}", self, p);
        p
    }
    pub fn angle(&self) -> f32 {
        let x = self.x as f32;
        let y = self.y as f32;
        let len = (x * x + y * y).sqrt();
        -((x / len).atan2(y / len) - std::f32::consts::PI)
    }

    pub fn to_polar(&self) -> Polar {
        let angle = {
            let min = self.minimize();
            let x = min.x as f32;
            let y = min.y as f32;
            let len = (x * x + y * y).sqrt();
            -((x / len).atan2(y / len) - std::f32::consts::PI)
        };
        let x = self.x as f32;
        let y = self.y as f32;
        let dist = (x * x + y * y).sqrt();
        Polar {
            angle,
            dist,
            // oo_dist: 1.0 / dist,
        }
    }
}
impl From<Dir> for Point {
    fn from(dir: Dir) -> Point {
        match dir {
            Dir::Up => Point::up(),
            Dir::Down => Point::down(),
            Dir::Left => Point::left(),
            Dir::Right => Point::right(),
        }
    }
}
impl From<&Dir> for Point {
    fn from(dir: &Dir) -> Point {
        match dir {
            Dir::Up => Point::up(),
            Dir::Down => Point::down(),
            Dir::Left => Point::left(),
            Dir::Right => Point::right(),
        }
    }
}
impl Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn minimize() {
        assert_eq!(Point { x: 6, y: 9 }.minimize(), Point { x: 2, y: 3 });
        assert_eq!(Point { x: 9, y: 6 }.minimize(), Point { x: 3, y: 2 });
        assert_eq!(Point { x: 14, y: 10 }.minimize(), Point { x: 7, y: 5 });
        assert_eq!(Point { x: 9, y: 90 }.minimize(), Point { x: 1, y: 10 });
        assert_eq!(Point { x: 7, y: 7 }.minimize(), Point { x: 1, y: 1 });
        assert_eq!(Point { x: -7, y: 7 }.minimize(), Point { x: -1, y: 1 });
        assert_eq!(Point { x: 7, y: -7 }.minimize(), Point { x: 1, y: -1 });
        assert_eq!(Point { x: 123, y: 45 }.minimize(), Point { x: 41, y: 15 });
        assert_eq!(Point { x: 100, y: 0 }.minimize(), Point { x: 1, y: 0 });
        assert_eq!(Point { x: 0, y: 7 }.minimize(), Point { x: 0, y: 1 });
    }
}
