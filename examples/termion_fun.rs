use aoc2019::monitoring::*;
use pathfinding::directed::dijkstra;
use pathfinding::prelude::dijkstra;
use rand::prelude::*;
use std::collections::HashSet;
use std::io::Read;
use std::io::{Stdin, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::IntoRawMode;

fn input_keys() -> Receiver<Key> {
    let stdin = std::io::stdin();
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        for k in stdin.keys() {
            if tx.send(k.unwrap()).is_err() {
                break;
            }
        }
    });

    return rx;
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

fn main() {
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    let mut cursor = Point::new(10, 10);

    let mut start = Point::new(1, 1);
    let mut goal = Point::new(100, 100);
    let mut walls = HashSet::<Point>::new();
    let mut path = Vec::new();
    let mut touched: HashSet<Point> = HashSet::new();
    let mut draw = false;
    let mut do_path = false;
    // loop {
    let rx = input_keys();
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();
    loop {
        let cursor_old = cursor.clone();
        match rx.recv().unwrap() {
            Key::Up => cursor.y -= 1,
            Key::Down => cursor.y += 1,
            Key::Left => cursor.x -= 1,
            Key::Right => cursor.x += 1,
            Key::Char('s') => start = cursor,
            Key::Char('g') => goal = cursor,
            Key::Char('w') => draw = !draw,
            Key::Char('p') => do_path = true,
            Key::Char('q') => break,
            _ => (),
        }

        write!(stdout, "{}", termion::clear::All).unwrap();
        for t in &touched {
            if t.x > 0 && t.y > 0 && t.x < 100 && t.y < 60 {
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(t.x as u16, t.y as u16),
                    "."
                )
                .unwrap();
            }
        }
        write!(
            stdout,
            "{}{}{}{}",
            termion::cursor::Goto(cursor_old.x as u16, cursor_old.y as u16),
            " ",
            termion::cursor::Goto(cursor.x as u16, cursor.y as u16),
            if draw { "@" } else { "+" }
        )
        .unwrap();
        if draw {
            walls.insert(cursor.clone());
        }

        for w in &walls {
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(w.x as u16, w.y as u16),
                "W"
            )
            .unwrap();
        }
        write!(
            stdout,
            "{}{}{}{}",
            termion::cursor::Goto(start.x as u16, start.y as u16),
            "S",
            termion::cursor::Goto(goal.x as u16, goal.y as u16),
            "G"
        )
        .unwrap();

        if do_path {
            touched = HashSet::new();
            let res = dijkstra(
                &start,
                |p| {
                    touched.insert(*p);
                    (1..=4)
                        .filter_map(|dir| {
                            let next = simulate_move(p, dir);
                            if !walls.contains(&next) {
                                Some((next, 1))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<(Point, i64)>>()
                },
                |p| return *p == goal,
            )
            .unwrap();
            path = res.0;
            do_path = false;
        }

        for p in &path {
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(p.x as u16, p.y as u16),
                "o"
            )
            .unwrap();
        }
        stdout.flush().unwrap();
    }
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Show).unwrap();
    // }
}
