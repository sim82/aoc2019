use std::collections::HashSet;
use std::io;
use std::iter::FromIterator;
fn read_line() -> String {
    let stdin = io::stdin();
    let mut line = "".into();
    stdin.read_line(&mut line).unwrap();
    line
}

fn read_input(line: String) -> Vec<(char, i32)> {
    line.split_terminator(",")
        .filter_map(|s| {
            let mut s: String = s.trim().into();
            if s.len() < 2 {
                None
            } else {
                let dir = s.remove(0);
                Some((dir, s.parse::<i32>().ok()?))
            }
        })
        .collect::<Vec<_>>()
}

fn trace(input: Vec<(char, i32)>) -> Vec<(i32, i32)> {
    let mut pos = (0, 0);
    let mut out = vec![pos];

    for p in input {
        let step = match p {
            ('R', _) => (1, 0),
            ('L', _) => (-1, 0),
            ('U', _) => (0, 1),
            ('D', _) => (0, -1),
            _ => panic!("bad input"),
        };

        for _ in 0..p.1 {
            pos.0 += step.0;
            pos.1 += step.1;
            out.push(pos);
        }
    }
    out
}

fn main() {
    let a = trace(read_input(read_line()));
    let aset: HashSet<_> = a.iter().collect();

    let b = trace(read_input(read_line()));
    let bset: HashSet<_> = b.iter().collect();
    let u = aset.intersection(&bset);
    let res1 = u
        .clone()
        .filter_map(|(x, y)| {
            if *x == 0 && *y == 0 {
                None
            } else {
                Some(x.abs() + y.abs())
            }
        })
        .min()
        .unwrap();
    println!("res1: {}", res1);

    let res2 = u
        .filter_map(|(x, y)| {
            if *x == 0 && *y == 0 {
                None
            } else {
                let posa = a.iter().position(|&r| r == (*x, *y))?;
                let posb = b.iter().position(|&r| r == (*x, *y))?;
                Some(posa + posb)
            }
        })
        .min()
        .unwrap();
    println!("res2: {}", res2);
}
