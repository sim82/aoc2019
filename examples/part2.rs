use std::io;

fn main() {
    let stdin = io::stdin();
    let mut line = "".into();
    stdin.read_line(&mut line).unwrap();
    let mut data = line
        .trim()
        .split_terminator(",")
        .filter_map(|s| {
            // println!("{}", s);
            s.parse::<i32>().ok()
        })
        .collect::<Vec<_>>();

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
            _ => panic!("invalid opcode {}", data[i]),
        }
        i += 4;
    }
    println!("output: {}", data[0])
}
