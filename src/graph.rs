use crate::monitoring::*;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;
use std::str::FromStr;

pub struct CharMap2d {
    data: Vec<Vec<char>>,
}

impl CharMap2d {
    pub fn new(data: &Vec<&'static str>) -> Self {
        println!("lines: {}", data.len());
        let data = data.iter().map(|line| line.chars().collect()).collect();
        CharMap2d { data }
    }

    pub fn get_point(&self, p: &Point) -> char {
        let x = p.x as usize;
        let y = p.y as usize;
        if y >= self.data.len() || x >= self.data[y].len() {
            panic!("out of bounds: {} {}", x, y);
        }
        self.data[y][x]
    }

    pub fn get_neighbors<'a, P: 'a>(
        &'a self,
        p: &'a Point,
        mut predicate: P,
    ) -> impl Iterator<Item = (Dir, char)> + 'a
    //Vec<(Dir, char)>
    where
        P: FnMut(&char) -> bool,
    {
        const DIRS: [Dir; 4] = [Dir::Up, Dir::Right, Dir::Down, Dir::Left];
        DIRS.iter().filter_map(move |dir| {
            let neighbor = p.move_into(dir);
            let nc = self.get_point(&neighbor);
            if predicate(&nc) {
                Some((dir.clone(), nc))
            } else {
                None
            }
        })
        // .collect()
    }

    pub fn get_x_range(&self, y: i32) -> std::ops::Range<i32> {
        0..self.data[y as usize].len() as i32
    }
    pub fn get_y_range(&self) -> std::ops::Range<i32> {
        0..self.data.len() as i32
    }
}
