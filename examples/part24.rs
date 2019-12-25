use std::collections::HashSet;

type Field = u32;
const PAT1: Field = 0b1000101;
const PAT5: Field = 0b10001000001;
const PAT6: Field = 0b100010100010;

const PAT9: Field = 0b100000100010000;
const PAT21: Field = 0b10100010000000000000000;
const PATTERN: [Field; 25] = [
    0b100010,
    PAT1,
    PAT1 << 1,
    PAT1 << 2,
    0b1000001000,
    PAT5,
    PAT6,
    PAT6 << 1,
    PAT6 << 2,
    PAT9,
    PAT5 << 5,
    PAT6 << 5,
    PAT6 << 6,
    PAT6 << 7,
    PAT9 << 5,
    PAT5 << 10,
    PAT6 << 10,
    PAT6 << 11,
    PAT6 << 12,
    PAT9 << 10,
    0b1000001000000000000000,
    PAT21,
    PAT21 << 1,
    PAT21 << 2,
    0b0100010000000000000000000,
];

fn display(mut f: Field) {
    for i in 0..25 {
        let c = if (f & 1) == 1 { '#' } else { '.' };
        print!("{}", c);
        // println!("f: {}", f);
        f >>= 1;
        if i % 5 == 4 {
            println!("");
        }
    }
    println!("")
}
fn evolve(f: Field) -> Field {
    let mut new = 0;
    let mut probe = 1;
    for (i, pat) in PATTERN.iter().enumerate() {
        let cnt = (f & *pat).count_ones();
        // println!("cnt: {} {} {}", i, cnt, probe);
        if f & probe == 0 {
            if cnt == 1 || cnt == 2 {
                new |= probe;
            }
        } else {
            if cnt == 1 {
                new |= probe;
            }
        }
        probe <<= 1;
    }
    new
}

fn main() {
    // display(0b10101010);
    for (i, p) in PATTERN.iter().enumerate() {
        println!("p: {}", i);
        display(*p);
    }
    // ....#
    // #..#.
    // #..##
    // ..#..
    // #....
    //let mut f = 0b0000100100110010100110000;

    // ..#.#
    // .#.##
    // ...#.
    // ...##
    // #.###
    let mut f = 0b1110111000010001101010100;

    display(f);
    let mut all_pattern = HashSet::new();
    all_pattern.insert(f);
    for i in 1.. {
        f = evolve(f);
        if all_pattern.contains(&f) {
            println!("cycle:");
            println!("n: {} {}", i, f);
            display(f);
            break;
        }
        all_pattern.insert(f);
    }
}
