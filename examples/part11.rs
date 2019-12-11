use aoc2019::{intcode::*, monitoring::Point};
use std::collections::HashMap;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn turn_left(&mut self) {
        *self = match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
    fn to_vec(&self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: -0 },
        }
    }
}
struct Painter {
    tiles: HashMap<Point, i64>,
    turtle: Point,
    dir: Direction,
    acc: Option<i64>,
}
impl Painter {
    fn new() -> Self {
        Painter {
            tiles: HashMap::new(),
            turtle: Point { x: 0, y: 0 },
            dir: Direction::Up,
            acc: None,
        }
    }
}
impl Io2 for Painter {
    fn read(&mut self) -> i64 {
        match self.tiles.get(&self.turtle) {
            Some(color) => *color,
            None => 0,
        }
    }
    fn write(&mut self, v: i64) {
        if let Some(color) = self.acc {
            self.tiles.insert(self.turtle, color);
            match v {
                0 => self.dir.turn_left(),
                1 => self.dir.turn_right(),
                _ => panic!("bad turn location {}", v),
            }
            self.turtle = self.turtle + self.dir.to_vec();
            self.acc = None;
        } else {
            self.acc = Some(v);
        }
    }
}
impl Painter {
    fn draw(&self) {
        let mut min_x = std::i32::MAX;
        let mut min_y = std::i32::MAX;
        let mut max_x = std::i32::MIN;
        let mut max_y = std::i32::MIN;
        for p in self.tiles.keys() {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.x);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.x);
        }
        println!("bounds {} {} - {} {}", min_x, min_y, max_x, max_y);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let out = match self.tiles.get(&Point { x, y }) {
                    Some(0) => '.',
                    Some(1) => '#',
                    None => '.',
                    _ => panic!("bad color"),
                };
                print!("{}", out);
            }
            println!();
        }
    }
}

fn main() {
    {
        let mut context = Context::new(code11());
        let mut painter = Painter::new();
        (&mut context, &mut painter as &mut Io2).run();
        println!("{:?}", context.data);
        println!("{}", painter.tiles.len());
        painter.draw();
    }
    {
        let mut context = Context::new(code11());
        let mut painter = Painter::new();
        painter.tiles.insert(Point { x: 0, y: 0 }, 1);
        (&mut context, &mut painter as &mut Io2).run();
        println!("{:?}", context.data);
        println!("{}", painter.tiles.len());
        painter.draw();
    }
}

fn code11() -> Vec<i64> {
    vec![
        3,
        8,
        1005,
        8,
        325,
        1106,
        0,
        11,
        0,
        0,
        0,
        104,
        1,
        104,
        0,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        101,
        0,
        8,
        28,
        2,
        3,
        7,
        10,
        2,
        1109,
        3,
        10,
        2,
        102,
        0,
        10,
        2,
        1005,
        12,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        101,
        0,
        8,
        67,
        2,
        109,
        12,
        10,
        1,
        1003,
        15,
        10,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        101,
        0,
        8,
        96,
        3,
        8,
        102,
        -1,
        8,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        1002,
        8,
        1,
        119,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        101,
        0,
        8,
        141,
        3,
        8,
        1002,
        8,
        -1,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        1001,
        8,
        0,
        162,
        1,
        106,
        17,
        10,
        1006,
        0,
        52,
        1006,
        0,
        73,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        1001,
        8,
        0,
        194,
        1006,
        0,
        97,
        1,
        1004,
        6,
        10,
        1006,
        0,
        32,
        2,
        8,
        20,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        1008,
        8,
        1,
        10,
        4,
        10,
        102,
        1,
        8,
        231,
        1,
        1,
        15,
        10,
        1006,
        0,
        21,
        1,
        6,
        17,
        10,
        2,
        1005,
        8,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        102,
        1,
        8,
        267,
        2,
        1007,
        10,
        10,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        1,
        10,
        4,
        10,
        102,
        1,
        8,
        294,
        1006,
        0,
        74,
        2,
        1003,
        2,
        10,
        1,
        107,
        1,
        10,
        101,
        1,
        9,
        9,
        1007,
        9,
        1042,
        10,
        1005,
        10,
        15,
        99,
        109,
        647,
        104,
        0,
        104,
        1,
        21101,
        936333018008,
        0,
        1,
        21101,
        342,
        0,
        0,
        1106,
        0,
        446,
        21102,
        937121129228,
        1,
        1,
        21101,
        0,
        353,
        0,
        1105,
        1,
        446,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        1,
        21101,
        0,
        209383001255,
        1,
        21102,
        400,
        1,
        0,
        1106,
        0,
        446,
        21101,
        0,
        28994371675,
        1,
        21101,
        411,
        0,
        0,
        1105,
        1,
        446,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        0,
        21101,
        867961824000,
        0,
        1,
        21101,
        0,
        434,
        0,
        1106,
        0,
        446,
        21102,
        1,
        983925674344,
        1,
        21101,
        0,
        445,
        0,
        1106,
        0,
        446,
        99,
        109,
        2,
        21201,
        -1,
        0,
        1,
        21102,
        40,
        1,
        2,
        21101,
        477,
        0,
        3,
        21102,
        467,
        1,
        0,
        1106,
        0,
        510,
        109,
        -2,
        2106,
        0,
        0,
        0,
        1,
        0,
        0,
        1,
        109,
        2,
        3,
        10,
        204,
        -1,
        1001,
        472,
        473,
        488,
        4,
        0,
        1001,
        472,
        1,
        472,
        108,
        4,
        472,
        10,
        1006,
        10,
        504,
        1101,
        0,
        0,
        472,
        109,
        -2,
        2106,
        0,
        0,
        0,
        109,
        4,
        1201,
        -1,
        0,
        509,
        1207,
        -3,
        0,
        10,
        1006,
        10,
        527,
        21102,
        1,
        0,
        -3,
        21202,
        -3,
        1,
        1,
        21201,
        -2,
        0,
        2,
        21102,
        1,
        1,
        3,
        21102,
        1,
        546,
        0,
        1106,
        0,
        551,
        109,
        -4,
        2105,
        1,
        0,
        109,
        5,
        1207,
        -3,
        1,
        10,
        1006,
        10,
        574,
        2207,
        -4,
        -2,
        10,
        1006,
        10,
        574,
        22101,
        0,
        -4,
        -4,
        1105,
        1,
        642,
        21202,
        -4,
        1,
        1,
        21201,
        -3,
        -1,
        2,
        21202,
        -2,
        2,
        3,
        21101,
        0,
        593,
        0,
        1105,
        1,
        551,
        22102,
        1,
        1,
        -4,
        21101,
        1,
        0,
        -1,
        2207,
        -4,
        -2,
        10,
        1006,
        10,
        612,
        21102,
        1,
        0,
        -1,
        22202,
        -2,
        -1,
        -2,
        2107,
        0,
        -3,
        10,
        1006,
        10,
        634,
        21201,
        -1,
        0,
        1,
        21101,
        634,
        0,
        0,
        105,
        1,
        509,
        21202,
        -2,
        -1,
        -2,
        22201,
        -4,
        -2,
        -4,
        109,
        -5,
        2106,
        0,
        0,
    ]
}
