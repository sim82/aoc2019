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

    let mut it = input.chunks(size).rev();
    let mut first = it.next().unwrap().to_vec();
    let output: Vec<_> = it.fold(first, |acc, layer| {
        acc.iter()
            .zip(layer.iter())
            .map(|(x, y)| if *y != 2 { *y } else { *x })
            .collect()
    });
    // let output: Vec<_> = it.fold(first, |acc, layer| {
    //     acc.iter_mut()
    //         .zip(layer.iter())
    //         .for_each(|(x, y)| *x = if *y != 2 { *y } else { *x });
    //     acc
    // });

    println!("\n");
    for line in output.chunks(25) {
        println!(
            "{}",
            String::from_utf8(
                line.iter()
                    .map(|x| if *x == 1 { 'X' as u8 } else { ' ' as u8 })
                    .collect()
            )
            .unwrap()
        );
    }
    println!("\n");
}
