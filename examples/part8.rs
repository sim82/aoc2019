use aoc2019::spaceimg::SpaceImg;
use std::slice::Iter;

fn main() {
    let input: Vec<_> = include_str!("input_8")
        .trim()
        .chars()
        .map(|x| x as i32 - '0' as i32)
        .collect();

    let width = 25;
    let height = 6;
    let size = width * height;
    println!(
        "len {} {} {} {}",
        input.len(),
        size,
        input.len() / size,
        input.len() % size
    );
    let layer_info: Vec<_> = input
        .chunks(size)
        .map(|layer| {
            let num_zeros = layer.iter().filter(|x| **x == 0).count();
            let num_ones = layer.iter().filter(|x| **x == 1).count();
            let num_twos = layer.iter().filter(|x| **x == 2).count();
            (num_zeros, num_ones, num_twos)
        })
        .collect();
    let min_layer = layer_info.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap();
    println!("infos: {:?}", layer_info);
    println!("min layer: {:?} {}", min_layer, min_layer.1 * min_layer.2);

    println!("\n");
    input.draw(width, height);
    let decoded = input.decode(width, height);
    println!("\n");

    decoded.draw(width, height);
    println!("\n");
}
