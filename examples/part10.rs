use aoc2019::monitoring::*;
use std::collections::HashSet;

fn load_map(data: &str, filter: char) -> Vec<Point> {
    data.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == filter {
                    Some(Point {
                        x: x as i32,
                        y: y as i32,
                    })
                } else {
                    None
                }
            })
        })
        .collect()
}

fn num_blockers(points: Vec<Point>) -> Vec<(i32, Point)> {
    let mut num_blocked = Vec::new();
    for src in &points {
        let mut delta: Vec<_> = points.iter().map(|dest| dest - src).collect();
        delta.sort_by_key(|dist| dist.x.abs() + dist.y.abs());
        let mut blocked = 0;
        for d in &delta {
            if d.x == 0 && d.y == 0 {
                continue;
            }
            let mind = d.minimize();
            let mut d = d.clone();
            loop {
                d = &d - &mind;
                if d.x == 0 && d.y == 0 {
                    break;
                }
                if delta.iter().find(|x| **x == d).is_some() {
                    blocked += 1;
                    break;
                }
            }
        }
        num_blocked.push((blocked, *src));
    }
    num_blocked.sort_by_key(|x| x.0);
    num_blocked
}

fn part1() {
    let num_blocked = num_blockers(load_map(include_str!("input10_1.txt"), '#'));
    println!("{} {:?}", num_blocked[0].0, num_blocked[0].1);

    let num_blocked = num_blockers(load_map(include_str!("input10_2.txt"), '#'));
    println!("{} {:?}", num_blocked[0].0, num_blocked[0].1);

    let num_blocked = num_blockers(load_map(include_str!("input10_3.txt"), '#'));
    println!("{} {:?}", num_blocked[0].0, num_blocked[0].1);

    let num_blocked = num_blockers(load_map(include_str!("input10_4.txt"), '#'));
    println!("{} {:?}", num_blocked[0].0, num_blocked[0].1);

    let points = load_map(include_str!("input10_5.txt"), '#');
    let num_points = points.len() as i32;
    let num_blocked = num_blockers(points);
    println!(
        "{} {:?}",
        num_points - num_blocked[0].0 - 1,
        num_blocked[0].1
    );
}

fn num_blockers2(points: Vec<Point>) -> (i32, Point) {
    for src in &points {
        let polar_points: Vec<_> = points
            .iter()
            .filter(|point| *point != src)
            .map(|point| (point - &src).to_polar())
            .collect();
        polar_points.sort_by(|pola, polb| pola.partial_cmp(polb).unwrap());
    }
    // (0, {})
}

fn main() {
    part1();
    let points = load_map(include_str!("input10_5.txt"), '#');
    let num_blocked = num_blockers(points.clone());

    let start = num_blocked[0].1;
    println!("start: {:?}", start);
    let rel_points: Vec<_> = points
        .iter()
        .filter(|point| **point != start)
        .map(|point| point - &start)
        .collect();
    let polar_points: Vec<_> = rel_points.iter().map(|point| point.to_polar()).collect();

    for (point, polar) in rel_points.iter().zip(&polar_points) {
        println!("{:?} {:?}", point, polar);
    }
    let mut pairs: Vec<_> = polar_points.iter().zip(&rel_points).collect();

    pairs.sort_by(|(pola, _), (polb, _)| pola.partial_cmp(polb).unwrap());
    for (point, polar) in &pairs {
        println!("{:?} {:?}", point, polar);
    }
    let mut shot = HashSet::<Point>::new();
    // shot.insert(pairs[0].1.clone());
    while shot.len() != pairs.len() {
        let mut shot_dir = None;

        for (polar, point) in &pairs {
            if shot.contains(point) {
                continue;
            }
            if let Some(shot_dir) = shot_dir {
                if polar.angle <= shot_dir {
                    continue;
                }
            }
            shot_dir = Some(polar.angle);
            shot.insert(*point.clone());
            println!("{} {:?}", shot.len(), start + **point);
        }
    }
}
