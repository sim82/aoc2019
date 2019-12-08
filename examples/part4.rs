use std::iter::Iterator;

fn main() {
    let mut count = 0;
    for i in 357253..892942 {
        let s = format!("{}", i);
        let s = s.as_bytes();

        let num_eq = s.windows(2).filter(|x| x[0] == x[1]).count();
        let num_inc = s.windows(2).filter(|x| x[0] < x[1]).count();

        if num_eq + num_inc == 5 && num_eq >= 1 {
            count += 1;
        }
    }

    println!("count: {}", count);
}
