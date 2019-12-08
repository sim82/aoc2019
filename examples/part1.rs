use std::io::{self, BufRead};

fn calc_mass(i: i32) -> i32 {
    let num = i / 3 - 2;
    if num < 0 {
        return 0;
    }
    num + calc_mass(num)
}
fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let sum = input.iter().map(|num| num / 3 - 2).sum::<i32>();
    println!("sum: {}", sum);

    let sum = input.iter().map(|num| calc_mass(*num)).sum::<i32>();
    println!("sum2: {}", sum);
}
