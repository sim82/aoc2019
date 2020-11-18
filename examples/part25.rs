// #[macro_use]
// extern crate nom;

use aoc2019::intcode::*;
use aoc2019::monitoring::*;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{Read, Write};
use std::sync::mpsc::{channel, Receiver};
use termion::event::Key;
use termion::raw::IntoRawMode;
// named!(get_title<&str, &str>, ws!(tag!("==")));

// named!(get_title<&str, (&str, &str, &str)>, delimited!(tag("=="), escaped!(), tag("==");

// named!(get_description<&str, &str>, ws!(take_until!("Doors here lead:")));

// named!(get_all<&str,(&str,Option<&str>,&str)>, tuple!(take_until!("Doors here lead:"), opt!(take_until!("Items here:")), take_until!("Command?")));

// named!(
//     get_all<&str,&str>,
//     do_parse!(title: get_title >> desc: get_description)
// );
// named!(
//     get_all<&str,((&str, &str, &str),&str)>,
//     tuple!(get_title, get_description)
// );
// named!(get_title<&str, &str>, delimited!(tag!("=="), escaped!(), tag!("==")));

#[derive(Debug, Clone, PartialEq, Eq)]
struct Location {
    title: String,
    desc: String,
    directions: HashSet<GeoDir>,
    items: HashSet<String>,
}

struct Map {
    locations: HashMap<Point, Location>,
    min: Point,
    max: Point,
}
impl Map {
    fn new() -> Self {
        Map {
            locations: HashMap::new(),
            min: Point::zero(),
            max: Point::zero(),
        }
    }

    fn new_location(&mut self, coord: &Point, location: &Location) {
        self.min.x = self.min.x.min(coord.x);
        self.max.x = self.max.x.max(coord.x);

        self.min.y = self.min.y.min(coord.y);
        self.max.y = self.max.y.max(coord.y);
        self.locations.insert(coord.clone(), location.clone());
    }
    fn draw(&mut self, me: &Point, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) -> i32 {
        // for y in self.min.y..=self.max.y {
        //     for x in self.min.x..=self.max.x {
        //         let xd = x * 2;
        //         let yd = y * 2;
        //         if let Some(location) = self.locations.
        //         write!(
        //             stdout,
        //             "{}{}",
        //             termion::cursor::Goto(x as u16, y as u16),
        //             "."
        //         ).unwrap();
        //     }
        // }
        // let x_offs = self.max.x - self.min.x;
        // let y_offs = self.max.y - self.min.y;
        let x_offs = 20;
        let y_offs = 20;
        write!(
            stdout,
            "{} {} {}",
            termion::cursor::Goto(0, 20),
            // self.min,
            // self.max,
            x_offs,
            y_offs
        )
        .unwrap();
        for (coord, location) in self.locations.iter() {
            let x = (coord.x * 2) + x_offs;
            let y = (coord.y * 2) + y_offs;

            if x < 0 || y < 0 {
                panic!("bad {} {}", x, y);
            }
            let tile = if coord == me { "o" } else { "." };
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(x as u16, y as u16),
                tile
            )
            .unwrap();
            for dir in location.directions.iter() {
                let dash = match dir {
                    GeoDir::North | GeoDir::South => '|',
                    GeoDir::East | GeoDir::West => '-',
                };
                let dash_pos = Point::new(x, y).move_into(&dir.clone().into());
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(dash_pos.x as u16, dash_pos.y as u16),
                    dash
                )
                .unwrap();
            }
        }
        y_offs * 2
    }
}

impl std::str::FromStr for Location {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut title = "".into();
        let mut desc: String = "".into();
        let mut directions = HashSet::new();
        let mut items = HashSet::new();

        // let lines: Vec<_> = s.lines().collect();
        let mut state = 0;
        for line in s.lines() {
            // println!("line: {}", line);
            // let line = line.unwrap();
            if line.starts_with("==") {
                state = 1;
            }
            if line == "Doors here lead:" {
                state = 3;
                continue;
            } else if line == "Items here:" {
                state = 4;
                continue;
            } else if line == "Command?" {
                if state != 3 && state != 4 {
                    return Err(());
                }
                break;
            }
            match state {
                0 => (),
                1 => {
                    title = line.into();
                    state = 2;
                }
                2 => desc.push_str(line),
                3 => {
                    if line.starts_with("- ") {
                        directions.insert(line[2..].parse::<GeoDir>().unwrap());
                    }
                }
                4 => {
                    if line.starts_with("- ") {
                        items.insert(line[2..].into());
                    }
                }
                _ => panic!("bad state"),
            }
        }

        // let title = get_all(s);
        // println!("title: {:?}", title);
        Ok(Location {
            title,
            desc,
            directions,
            items,
        })
    }
}

struct IoAdventure {
    input_buf: String,
    current_location: Option<Location>,
    current_coord: Point,
    input_rx: Receiver<Key>,
    output_buf: VecDeque<i64>,
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
    last_movement: Option<GeoDir>,
    map: Map,
    log_file: File,
    items: Vec<String>,
    trying_combinations: bool,
}

impl IoAdventure {
    fn new() -> Self {
        IoAdventure {
            input_buf: "".into(),
            current_location: None,
            current_coord: Point::zero(),
            input_rx: aoc2019::util::input_keys(),
            output_buf: VecDeque::new(),
            stdout: std::io::stdout().into_raw_mode().unwrap(),
            last_movement: None,
            map: Map::new(),
            log_file: File::create("/tmp/log.txt").unwrap(),
            items: Vec::new(),
            trying_combinations: false,
        }
    }
    fn output_command(&mut self, cmd: &str) {
        for c in cmd.chars() {
            self.output_buf.push_back(c as u8 as i64);
        }
        self.output_buf.push_back(10);
    }
}

impl Io2 for IoAdventure {
    fn read(&mut self) -> i64 {
        while self.output_buf.is_empty() {
            match self.input_rx.recv() {
                Ok(Key::Up) => {
                    self.last_movement = Some(GeoDir::North);
                    self.output_command("north")
                }
                Ok(Key::Down) => {
                    self.last_movement = Some(GeoDir::South);
                    self.output_command("south")
                }
                Ok(Key::Left) => {
                    self.last_movement = Some(GeoDir::West);
                    self.output_command("west")
                }
                Ok(Key::Right) => {
                    self.last_movement = Some(GeoDir::East);
                    self.output_command("east")
                }

                Ok(Key::F(12)) => panic!("exit!"),
                Ok(Key::Char('t')) => match &self.current_location {
                    Some(location) => {
                        if !location.items.is_empty() {
                            let item = location.items.iter().last().unwrap();
                            self.items.push(item.clone());
                            self.output_command(&format!("take {}", *item));
                        }
                    }
                    _ => (),
                },
                Ok(Key::Char(c)) if c.is_digit(10) => {
                    match &self.current_location {
                        Some(location) => {
                            let num = (c.to_digit(10).unwrap() - 1) as usize;
                            if num < self.items.len() {
                                self.output_command(&format!("drop {}", self.items[num]));
                                self.items.remove(num);
                            }
                        }
                        _ => (),
                    };
                    self.last_movement = None
                }
                Ok(Key::Char('p')) => {
                    let items: Vec<_> = self.items.iter().cloned().collect();

                    let mut cur_items = items.clone();

                    for i in 1..2u32.pow(items.len() as u32) {
                        for item in cur_items.iter() {
                            self.output_command(&format!("drop {}", item));
                        }
                        for (j, item) in items.iter().enumerate() {
                            if (i & (0b1 << j)) != 0 {
                                cur_items.push(item.clone());
                                self.output_command(&format!("take {}", item));
                            }
                        }
                        // self.log_file
                        //     .write(&format!("combination: {:?}", cur_items).into_bytes()[..]);

                        self.output_command("north");
                        // println!("combination: ")
                    }
                    // self.log_file.flush();
                }

                Ok(_) => self.last_movement = None,
                Err(_) => panic!("recv error"),
            }
        }
        return self.output_buf.pop_front().unwrap();
    }

    fn write(&mut self, v: i64) {
        self.input_buf.push(v as u8 as char);
        if self.input_buf.ends_with("Command?\n") {
            write!(self.log_file, "{}", self.input_buf);
            self.log_file.flush().unwrap();
            //println!("buffer: {}", self.input_buf);
            let new_location = self.input_buf.parse::<Location>().ok();
            let did_move = match (&new_location, &self.current_location) {
                (Some(new_location), Some(old_location)) => {
                    new_location.title != old_location.title
                }
                _ => true,
            };

            if did_move && new_location.is_some() && self.last_movement.is_some() {
                self.current_coord = self
                    .current_coord
                    .move_into(&(self.last_movement.as_ref().unwrap().clone().into()));

                self.current_location = new_location;
                self.map.new_location(
                    &self.current_coord,
                    &self.current_location.as_ref().unwrap(),
                );
            } else if self.trying_combinations && did_move {
                println!("correct combination: {:?}", self.items);
            } else {
                // println!("nothing new: {}", self.input_buf);
            }
            // println!("{:?}", self.current_location);
            self.input_buf.clear();
            write!(self.stdout, "{}", termion::clear::All).unwrap();
            let map_size = self.map.draw(&self.current_coord, &mut self.stdout);
            let map_size = 40;
            write!(
                self.stdout,
                "{}{:?}",
                termion::cursor::Goto(1, map_size as u16 + 2),
                self.items
            )
            .unwrap();
            write!(
                self.stdout,
                "{}{:?}",
                termion::cursor::Goto(1, map_size as u16 + 1),
                self.current_location
            )
            .unwrap();
            self.stdout.flush().unwrap();
        }
    }
}

fn main() {
    // let stdout = std::io::stdout().into_raw_mode().unwrap();

    let code = read_prog("examples/code25.int");
    let mut context = Context::new(code).break_on_output();
    let mut io = IoAdventure::new();

    loop {
        (&mut context, &mut io as &mut dyn Io2).run();
    }
}
