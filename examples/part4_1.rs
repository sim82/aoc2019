use std::iter::Iterator;

fn main() {
    let mut count = 0;
    for i in 357253..892942 {
        let s = format!("{}", i);
        let s = s.as_bytes();

        let mut ranges = Vec::new();
        let mut start = 0;
        // println!("{}", i);
        for i in 1..6 {
            if s[start] != s[i] {
                ranges.push(start..i);
                start = i
            }
        }
        ranges.push(start..6);

        let num_pairs = ranges.iter().filter(|x| x.len() == 2).count();
        let num_nondec = s.windows(2).filter(|x| x[0] <= x[1]).count();
        // println!("num pairs: {} {}", num_pairs, num_nondec);
        if num_nondec == 5 && num_pairs >= 1 {
            count += 1;
        }
    }

    println!("count: {}", count);
}
