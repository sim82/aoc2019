use aoc2019::{intcode::*, monitoring::*};
use pathfinding::prelude::dijkstra;
use rand::prelude::*;
use std::collections::HashSet;
use std::io::Read;
use std::io::{Stdin, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct RepairBot {
    empty: HashSet<Point>,
    wall: HashSet<Point>,
    stdin: std::io::Stdin,
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
    pos: Point,
    new_pos: Option<Point>,
    oxygen_pos: Option<Point>,
    num_move: i64,
    path: HashSet<Point>,
    has_oxygen: HashSet<Point>,
}
impl RepairBot {
    fn new(stdin: std::io::Stdin, stdout: termion::raw::RawTerminal<std::io::Stdout>) -> Self {
        RepairBot {
            // tiles: Vec::new(),
            empty: [Point::new(0, 0)].iter().cloned().collect(),
            wall: HashSet::new(),
            stdin,
            stdout,
            pos: Point::zero(),
            new_pos: None,
            oxygen_pos: None,
            num_move: 0,
            path: HashSet::new(),
            has_oxygen: HashSet::new(),
        }
    }

    fn draw(&mut self) {
        let x: Vec<_> = self
            .empty
            .iter()
            .chain(self.wall.iter())
            .map(|pos| pos.x)
            .collect();
        let y: Vec<_> = self
            .empty
            .iter()
            .chain(self.wall.iter())
            .map(|pos| pos.y)
            .collect();

        let max_x = *x.iter().max().unwrap_or(&0);
        let min_x = *x.iter().min().unwrap_or(&0);
        let max_y = *y.iter().max().unwrap_or(&0);
        let min_y = *y.iter().min().unwrap_or(&0);

        for y in min_y..=max_y {
            let mut line: String = "".into();
            for x in min_x..=max_x {
                let p = Point::new(x, y);
                if p.x == 0 && p.y == 0 {
                    line.push('+');
                } else if self.oxygen_pos == Some(p) {
                    line.push('@');
                } else if self.path.contains(&p) {
                    line.push('o');
                } else if self.empty.get(&p).is_some() {
                    line.push('.');
                } else if self.wall.get(&p).is_some() {
                    line.push(' ');
                } else {
                    line.push('X');
                }
            }
            // println!("{}", line);
            write!(
                self.stdout,
                "{}{}",
                termion::cursor::Goto(1, (y - min_y + 1) as u16),
                // termion::clear::CurrentLine,
                line
            )
            .unwrap();
        }
        write!(
            self.stdout,
            "{}{}{}{}",
            termion::cursor::Goto(1, (max_y - min_y + 2) as u16),
            format!(
                "{} {} - {} {} {}",
                min_x, min_y, max_x, max_y, self.num_move
            ),
            termion::cursor::Goto(1, (max_y - min_y + 3) as u16),
            format!(
                "path: {} {} {}",
                self.path.len(),
                self.empty.len(),
                self.wall.len()
            )
        )
        .unwrap();
    }
    fn empty_successors(&self, pos: &Point) -> Vec<Point> {
        let mut res = Vec::new();
        res.reserve(4);
        for i in 1..=4 {
            let p = simulate_move(pos, i as i64);
            if self.empty.contains(&p) {
                res.push(p);
            }
        }
        res
    }
}

fn simulate_move(pos: &Point, dir: i64) -> Point {
    *pos + match dir {
        1 => Point::new(0, -1),
        2 => Point::new(0, 1),
        3 => Point::new(1, 0),
        4 => Point::new(-1, 0),
        _ => panic!("bad direction"),
    }
}
impl Io2 for RepairBot {
    fn read(&mut self) -> i64 {
        if self.num_move % 1024 == 0 {
            self.draw();
        }
        self.num_move += 1;
        let mut rng = rand::thread_rng();

        loop {
            let dir = (1 + rng.next_u32() % 4) as i64;
            let pos = simulate_move(&self.pos, dir);
            if !self.wall.contains(&pos) {
                self.new_pos = Some(pos);
                return dir;
            }
        }
    }
    fn write(&mut self, v: i64) {
        match v {
            0 => {
                let wall = self.new_pos.unwrap();
                self.wall.insert(wall);
                self.new_pos = None;
            }
            1 => {
                let empty = self.new_pos.unwrap();
                self.pos = empty;

                self.empty.insert(empty);
                self.new_pos = None;
            }
            2 => {
                self.oxygen_pos = self.new_pos;
                self.pos = self.new_pos.unwrap();
                self.empty.insert(self.pos.clone());

                self.new_pos = None;

                let res = dijkstra(
                    &Point::new(0, 0),
                    |p| {
                        self.empty_successors(p)
                            .iter()
                            .map(|x| (*x, 1))
                            .collect::<Vec<_>>()
                    },
                    |p| *p == self.oxygen_pos.unwrap(),
                );
                if let Some(res) = res {
                    self.path = res.0.iter().cloned().collect();
                } else {
                    panic!("dijkstra failed");
                }
            }
            _ => panic!("bad output"),
        }
    }
}
// impl Drop for RepairBot {
//     fn drop(&mut self) {
//         write!(self.stdout, "{}", termion::cursor::Show).unwrap();
//     }
// }

fn main() {
    let time = {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();
        // ctrlc::set_handler(|| {
        //     let mut stdout = std::io::stdout().into_raw_mode().unwrap();
        //     write!(stdout, "{}", termion::cursor::Show).unwrap();
        //     panic!("ctrl-c");
        // });
        write!(stdout, "{}{}", termion::cursor::Show, termion::clear::All).unwrap();

        let mut context = Context::new(code15());
        let mut screen = RepairBot::new(stdin, stdout);
        loop {
            (&mut context, &mut screen as &mut dyn Io2).run_for(1000);
            if screen.empty.len() == 799 && screen.wall.len() == 859 && screen.path.len() == 355 {
                break;
            }
        }
        screen.has_oxygen.insert(screen.oxygen_pos.unwrap());
        let mut time = 0;
        while screen.has_oxygen.len() < screen.empty.len() {
            time += 1;
            let new_oxy: Vec<_> = screen
                .has_oxygen
                .iter()
                .flat_map(|x| screen.empty_successors(x))
                .collect();
            for x in new_oxy {
                screen.has_oxygen.insert(x);
            }
            // screen.has_oxygen = screen.has_oxygen.union(new_oxycollect()).collect();
        }
        time
    };
    println!("done");
    println!("time: {}", time);
}

fn code15() -> Vec<i64> {
    vec![
        3, 1033, 1008, 1033, 1, 1032, 1005, 1032, 31, 1008, 1033, 2, 1032, 1005, 1032, 58, 1008,
        1033, 3, 1032, 1005, 1032, 81, 1008, 1033, 4, 1032, 1005, 1032, 104, 99, 101, 0, 1034,
        1039, 101, 0, 1036, 1041, 1001, 1035, -1, 1040, 1008, 1038, 0, 1043, 102, -1, 1043, 1032,
        1, 1037, 1032, 1042, 1105, 1, 124, 1001, 1034, 0, 1039, 102, 1, 1036, 1041, 1001, 1035, 1,
        1040, 1008, 1038, 0, 1043, 1, 1037, 1038, 1042, 1106, 0, 124, 1001, 1034, -1, 1039, 1008,
        1036, 0, 1041, 1001, 1035, 0, 1040, 101, 0, 1038, 1043, 101, 0, 1037, 1042, 1106, 0, 124,
        1001, 1034, 1, 1039, 1008, 1036, 0, 1041, 1002, 1035, 1, 1040, 1002, 1038, 1, 1043, 1001,
        1037, 0, 1042, 1006, 1039, 217, 1006, 1040, 217, 1008, 1039, 40, 1032, 1005, 1032, 217,
        1008, 1040, 40, 1032, 1005, 1032, 217, 1008, 1039, 1, 1032, 1006, 1032, 165, 1008, 1040,
        35, 1032, 1006, 1032, 165, 1102, 1, 2, 1044, 1105, 1, 224, 2, 1041, 1043, 1032, 1006, 1032,
        179, 1101, 0, 1, 1044, 1106, 0, 224, 1, 1041, 1043, 1032, 1006, 1032, 217, 1, 1042, 1043,
        1032, 1001, 1032, -1, 1032, 1002, 1032, 39, 1032, 1, 1032, 1039, 1032, 101, -1, 1032, 1032,
        101, 252, 1032, 211, 1007, 0, 63, 1044, 1105, 1, 224, 1102, 1, 0, 1044, 1105, 1, 224, 1006,
        1044, 247, 1001, 1039, 0, 1034, 102, 1, 1040, 1035, 1001, 1041, 0, 1036, 1001, 1043, 0,
        1038, 101, 0, 1042, 1037, 4, 1044, 1105, 1, 0, 60, 55, 93, 19, 49, 51, 86, 12, 18, 69, 42,
        30, 84, 1, 28, 84, 15, 15, 70, 11, 75, 8, 67, 37, 76, 61, 72, 2, 49, 82, 25, 57, 77, 51,
        87, 60, 21, 66, 5, 90, 56, 21, 74, 75, 51, 54, 83, 69, 57, 85, 99, 40, 94, 14, 84, 69, 34,
        51, 92, 29, 28, 2, 76, 1, 35, 70, 5, 91, 91, 61, 86, 2, 35, 74, 78, 44, 98, 44, 5, 78, 4,
        79, 53, 99, 80, 11, 75, 29, 2, 82, 31, 71, 82, 60, 22, 90, 68, 11, 84, 69, 8, 66, 74, 53,
        22, 69, 19, 49, 55, 69, 75, 36, 65, 18, 83, 37, 17, 10, 78, 89, 4, 74, 29, 51, 96, 11, 64,
        15, 99, 52, 51, 99, 14, 78, 66, 7, 99, 20, 26, 64, 91, 12, 94, 38, 65, 87, 91, 69, 5, 87,
        28, 2, 62, 45, 83, 35, 52, 19, 21, 83, 25, 51, 93, 92, 7, 70, 39, 92, 84, 31, 1, 98, 92,
        58, 30, 75, 22, 89, 79, 44, 14, 66, 11, 93, 36, 45, 90, 42, 18, 87, 73, 99, 5, 95, 94, 20,
        64, 78, 70, 98, 41, 52, 98, 5, 73, 94, 19, 57, 64, 88, 59, 83, 33, 51, 71, 25, 93, 43, 14,
        92, 83, 44, 83, 41, 52, 31, 91, 95, 51, 36, 98, 65, 45, 10, 89, 58, 51, 52, 88, 94, 59, 98,
        2, 45, 93, 83, 46, 74, 76, 11, 38, 9, 84, 99, 43, 97, 6, 28, 64, 28, 72, 81, 87, 74, 68,
        14, 27, 80, 96, 44, 10, 96, 36, 2, 33, 96, 78, 45, 30, 87, 89, 90, 50, 2, 72, 77, 10, 12,
        64, 74, 53, 7, 74, 57, 81, 28, 68, 11, 8, 47, 16, 88, 17, 42, 99, 58, 92, 36, 70, 32, 83,
        37, 49, 16, 97, 61, 88, 91, 54, 17, 33, 55, 29, 22, 85, 82, 30, 81, 40, 62, 69, 94, 47, 69,
        25, 77, 33, 87, 67, 40, 44, 96, 31, 75, 27, 80, 8, 16, 75, 67, 41, 82, 52, 95, 17, 56, 99,
        84, 66, 53, 65, 70, 87, 61, 15, 82, 86, 55, 96, 8, 24, 79, 99, 8, 79, 80, 7, 64, 69, 1, 67,
        5, 74, 20, 64, 4, 98, 13, 53, 2, 64, 23, 33, 78, 77, 51, 91, 13, 24, 69, 49, 56, 77, 64,
        10, 75, 11, 67, 86, 48, 98, 95, 19, 94, 20, 11, 62, 97, 62, 83, 97, 12, 95, 97, 90, 20, 72,
        75, 49, 56, 16, 65, 52, 88, 95, 61, 44, 86, 83, 94, 9, 25, 71, 99, 46, 80, 80, 32, 38, 56,
        83, 49, 89, 55, 75, 98, 52, 77, 85, 29, 42, 94, 29, 7, 75, 81, 16, 28, 57, 24, 92, 57, 67,
        27, 83, 42, 75, 88, 62, 50, 2, 94, 3, 42, 73, 17, 80, 73, 91, 62, 67, 84, 16, 76, 44, 16,
        70, 36, 79, 90, 41, 90, 91, 62, 26, 86, 94, 34, 68, 59, 27, 82, 74, 18, 19, 98, 56, 2, 90,
        96, 70, 28, 67, 38, 51, 84, 83, 13, 34, 4, 52, 67, 77, 31, 93, 12, 41, 86, 26, 61, 59, 67,
        73, 80, 19, 48, 60, 94, 57, 72, 56, 36, 77, 73, 57, 59, 94, 69, 5, 37, 90, 72, 62, 4, 85,
        12, 65, 94, 81, 5, 99, 30, 58, 73, 18, 90, 89, 6, 87, 82, 27, 41, 87, 46, 97, 19, 85, 11,
        81, 79, 17, 12, 94, 46, 99, 56, 77, 86, 11, 20, 65, 97, 37, 1, 71, 21, 37, 72, 29, 41, 83,
        39, 24, 86, 72, 25, 26, 20, 75, 78, 34, 75, 33, 38, 89, 13, 31, 55, 82, 81, 15, 88, 36, 76,
        82, 22, 24, 84, 73, 53, 8, 82, 83, 71, 15, 82, 44, 88, 41, 74, 80, 86, 19, 59, 65, 70, 76,
        62, 59, 79, 34, 20, 30, 28, 67, 35, 93, 34, 56, 65, 98, 97, 59, 93, 54, 84, 11, 85, 70, 95,
        17, 69, 28, 79, 65, 52, 69, 72, 10, 72, 2, 68, 84, 56, 12, 64, 74, 83, 13, 69, 78, 5, 51,
        91, 41, 88, 72, 10, 97, 33, 97, 33, 86, 19, 96, 59, 64, 44, 42, 88, 4, 57, 20, 84, 54, 44,
        92, 28, 17, 86, 15, 50, 5, 76, 37, 10, 97, 39, 33, 94, 5, 82, 7, 92, 9, 84, 55, 64, 23, 69,
        9, 96, 49, 81, 28, 69, 76, 92, 53, 88, 92, 92, 61, 78, 44, 74, 99, 96, 51, 79, 65, 71, 58,
        86, 34, 96, 96, 96, 26, 88, 0, 0, 21, 21, 1, 10, 1, 0, 0, 0, 0, 0, 0,
    ]
}
