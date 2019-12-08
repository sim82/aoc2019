use std::io;

fn run(mut data: Vec<i32>) -> Option<i32> {
    let mut i = 0;
    // println!("len: {}", data.len());
    while i < data.len() {
        match data[i] {
            99 => break,
            1 => {
                let dest = data[i + 3] as usize;
                data[dest] = data[data[i + 1] as usize] + data[data[i + 2] as usize];
                // println!("add {} = {} + {}", data[dest], data[i + 1], data[i + 2]);
            }
            2 => {
                let dest = data[i + 3] as usize;
                data[dest] = data[data[i + 1] as usize] * data[data[i + 2] as usize];
                // println!("mul {} = {} * {}", data[dest], data[i + 1], data[i + 2]);
            }
            _ => return None, //panic!("invalid opcode {}", data[i]),
        }
        i += 4;
    }
    Some(data[0])
}
fn main() {
    let stdin = io::stdin();
    let mut line = "".into();
    stdin.read_line(&mut line).unwrap();
    let mut data = line
        .split_terminator(",")
        .filter_map(|s| {
            // println!("{}", s);
            s.trim().parse::<i32>().ok()
        })
        .collect::<Vec<_>>();

    println!("output: {}", run(data.clone()).unwrap());

    for i in 0..99 {
        for j in 0..99 {
            let mut data = data.clone();
            data[1] = i;
            data[2] = j;
            if run(data.clone()) == Some(19690720) {
                println!("result {} = 100 * {} + {}", i * 100 + j, i, j);
            }
        }
    }
}
