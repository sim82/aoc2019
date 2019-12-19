use aoc2019::intcode::*;
use aoc2019::monitoring::*;
use std::collections::HashSet;
use std::ops::Range;
struct DroneSystem1 {
    affected: HashSet<Point>,
    max_x: i64,
    max_y: i64,
    x: i64,
    y: i64,
    read_y: bool,
}

impl Io2 for DroneSystem1 {
    fn read(&mut self) -> i64 {
        if !self.read_y {
            self.read_y = true;
            println!("x: {}", self.x);
            self.x
        } else {
            self.read_y = false;
            println!("y: {}", self.y);

            self.y
        }
    }

    fn write(&mut self, v: i64) {
        println!("out: {}", v);
        if v == 1 {
            self.affected
                .insert(Point::new(self.x as i32, self.y as i32));
        }

        if self.y >= self.max_y {
            return;
        }
        self.x += 1;
        if self.x >= self.max_x {
            self.x = 0;
            self.y += 1;
        }
        // if self.y >= 50 {
        //     self.y = 0;
        //     println!("affected: {}", self.affected.len());
        //     self.affected = HashSet::new();
        // }
    }
}
impl DroneSystem1 {
    fn new() -> Self {
        DroneSystem1 {
            affected: HashSet::new(),
            max_x: 40,
            max_y: 40,
            x: 0,
            y: 0,
            read_y: false,
        }
    }
    fn draw(&mut self) {
        println!("");
        for y in 0..self.max_y {
            let mut line: String = "".into();
            for x in 0..self.max_x {
                let p = Point::new(x as i32, y as i32);
                if self.affected.contains(&p) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            println!("{}", line);
        }
    }
}

enum ScanState {
    SearchStart,
    Scanning,
}

struct DroneSystem {
    lines: Vec<Range<i64>>,
    out: std::collections::VecDeque<i64>,
    state: ScanState,
    x: i64,
    x_start: i64,
    has_block: bool,
}

impl Io2 for DroneSystem {
    fn read(&mut self) -> i64 {
        self.out.pop_front().unwrap()
    }

    fn write(&mut self, v: i64) {
        match self.state {
            ScanState::SearchStart => {
                if v == 1 {
                    self.state = ScanState::Scanning;
                    self.x_start = self.x;
                    self.x += 1;
                    self.out.push_back(self.x);
                    self.out.push_back(self.lines.len() as i64);
                } else {
                    self.x += 1;
                    self.out.push_back(self.x);
                    self.out.push_back(self.lines.len() as i64);
                    if self.x - self.x_start > 1000 {
                        println!("break");
                        self.lines.push(0..0);
                        self.x = self.x_start;
                    }
                }
            }
            ScanState::Scanning => {
                if v == 1 {
                    self.x += 1;
                    self.out.push_back(self.x);
                    self.out.push_back(self.lines.len() as i64);
                } else {
                    let cur_range = self.x_start..self.x;
                    // println!("range {} {:?}", self.lines.len(), cur_range);

                    self.lines.push(cur_range.clone());
                    self.x = self.x_start;
                    self.out.push_back(self.x);
                    self.out.push_back(self.lines.len() as i64);
                    self.state = ScanState::SearchStart;
                    if self.lines.len() >= 100 {
                        let start_line = self.lines.len() - 100;
                        let start_range = self.lines[start_line].clone();
                        if start_range.end - cur_range.start >= 100 {
                            println!(
                                "block: {} - {}: {:?} {:?} {} {}",
                                start_line,
                                self.lines.len(),
                                start_range,
                                cur_range,
                                start_range.end - 100,
                                start_line,
                            );
                            self.has_block = true;
                        }
                    }
                }
            }
        }
    }
}
impl DroneSystem {
    pub fn new() -> Self {
        let mut out = std::collections::VecDeque::new();
        out.push_back(0 as i64);
        out.push_back(0 as i64);
        DroneSystem {
            lines: Vec::new(),
            out,
            state: ScanState::SearchStart,
            x: 0,
            x_start: 0,
            has_block: false,
        }
    }
}

fn main() {
    {
        let mut drone_system = DroneSystem1::new();

        while drone_system.y < drone_system.max_y {
            let mut context = Context::new(code19());

            (&mut context, &mut drone_system as &mut dyn Io2).run();
        }
        drone_system.draw();
        println!("affected: {}", drone_system.affected.len());
    }

    {
        let mut drone_system = DroneSystem::new();
        let code = code19();
        let mut context = Context::new(code.clone());

        while drone_system.lines.len() < 10000 && !drone_system.has_block {
            (&mut context, &mut drone_system as &mut dyn Io2).run();
            context.reset(&code);
        }
        // for (i, range) in drone_system.lines.iter().enumerate() {
        //     println!("{} {:?}", i, range);
        // }
    }
}

fn code19() -> Vec<i64> {
    vec![
        109, 424, 203, 1, 21102, 1, 11, 0, 1106, 0, 282, 21101, 18, 0, 0, 1105, 1, 259, 2101, 0, 1,
        221, 203, 1, 21101, 0, 31, 0, 1105, 1, 282, 21101, 38, 0, 0, 1105, 1, 259, 21001, 23, 0, 2,
        22101, 0, 1, 3, 21102, 1, 1, 1, 21101, 57, 0, 0, 1106, 0, 303, 2102, 1, 1, 222, 20102, 1,
        221, 3, 20102, 1, 221, 2, 21101, 0, 259, 1, 21102, 80, 1, 0, 1105, 1, 225, 21102, 1, 130,
        2, 21102, 1, 91, 0, 1106, 0, 303, 2101, 0, 1, 223, 21002, 222, 1, 4, 21102, 259, 1, 3,
        21102, 1, 225, 2, 21101, 0, 225, 1, 21102, 1, 118, 0, 1106, 0, 225, 21002, 222, 1, 3,
        21101, 0, 106, 2, 21102, 1, 133, 0, 1106, 0, 303, 21202, 1, -1, 1, 22001, 223, 1, 1, 21101,
        148, 0, 0, 1105, 1, 259, 2102, 1, 1, 223, 20101, 0, 221, 4, 20102, 1, 222, 3, 21102, 1, 19,
        2, 1001, 132, -2, 224, 1002, 224, 2, 224, 1001, 224, 3, 224, 1002, 132, -1, 132, 1, 224,
        132, 224, 21001, 224, 1, 1, 21101, 195, 0, 0, 106, 0, 109, 20207, 1, 223, 2, 20101, 0, 23,
        1, 21102, -1, 1, 3, 21101, 0, 214, 0, 1105, 1, 303, 22101, 1, 1, 1, 204, 1, 99, 0, 0, 0, 0,
        109, 5, 1201, -4, 0, 249, 21201, -3, 0, 1, 21202, -2, 1, 2, 21201, -1, 0, 3, 21102, 1, 250,
        0, 1105, 1, 225, 22102, 1, 1, -4, 109, -5, 2106, 0, 0, 109, 3, 22107, 0, -2, -1, 21202, -1,
        2, -1, 21201, -1, -1, -1, 22202, -1, -2, -2, 109, -3, 2106, 0, 0, 109, 3, 21207, -2, 0, -1,
        1206, -1, 294, 104, 0, 99, 21201, -2, 0, -2, 109, -3, 2105, 1, 0, 109, 5, 22207, -3, -4,
        -1, 1206, -1, 346, 22201, -4, -3, -4, 21202, -3, -1, -1, 22201, -4, -1, 2, 21202, 2, -1,
        -1, 22201, -4, -1, 1, 22102, 1, -2, 3, 21102, 343, 1, 0, 1105, 1, 303, 1105, 1, 415, 22207,
        -2, -3, -1, 1206, -1, 387, 22201, -3, -2, -3, 21202, -2, -1, -1, 22201, -3, -1, 3, 21202,
        3, -1, -1, 22201, -3, -1, 2, 21201, -4, 0, 1, 21101, 384, 0, 0, 1106, 0, 303, 1106, 0, 415,
        21202, -4, -1, -4, 22201, -4, -3, -4, 22202, -3, -2, -2, 22202, -2, -4, -4, 22202, -3, -2,
        -3, 21202, -4, -1, -2, 22201, -3, -2, 1, 21201, 1, 0, -4, 109, -5, 2106, 0, 0,
    ]
}
