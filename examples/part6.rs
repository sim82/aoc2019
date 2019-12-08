use std::collections::HashMap;
use std::io::{self, BufRead};

fn parse_line(l: String) -> (String, String) {
    let mut s = l.split(")");
    let parent = s.next().unwrap().to_string();
    let child = s.next().unwrap().to_string();
    (child, parent)
}

trait TraceTo {
    fn len_to(&self, leaf: &str, root: &str) -> usize;
    fn trace_to(&self, leaf: &str, root: &str) -> Vec<String>;
}

impl TraceTo for HashMap<String, String> {
    fn trace_to(&self, leaf: &str, root: &str) -> Vec<String> {
        if leaf == root {
            return vec![root.into()];
        }
        // std::iter::once(leaf.to_string())
        //     .chain(self.trace_to(&self[leaf], root).into_iter())
        //     .collect()
        // let mut ret = vec![leaf.to_string()];
        // ret.append(&mut self.trace_to(&self[leaf], root));
        // ret
        let mut rest = self.trace_to(&self[leaf], root);
        rest.push(leaf.into());
        rest
    }
    fn len_to(&self, leaf: &str, root: &str) -> usize {
        self.trace_to(leaf, root).len()
    }
}
fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|line| parse_line(line.unwrap()))
        .collect::<HashMap<String, String>>();

    // println!("input: {:?}", input);
    let len = input
        .iter()
        .map(|(child, parent)| input.len_to(child, "COM"))
        .sum::<usize>();

    println!("all: {}", len);

    let you = input.trace_to("YOU", "COM");
    let san = input.trace_to("SAN", "COM");
    println!("you: {} {:?}", you.len(), you);
    println!("san: {} {:?}", san.len(), san);

    let common_path = you.iter().zip(san.iter()).filter(|(x, y)| x == y).count();
    println!("common len: {}", common_path);
    let dist = you.len() + san.len() - (2 * common_path) - 2;
    println!("dist: {}", dist);
}
