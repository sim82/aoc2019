use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
enum StackOp {
    DealIntoNewStack,
    DealWithIncrement(i32),
    Cut(i32),
}

trait Deck {
    fn apply_operation(&mut self, op: StackOp);
}

impl Deck for Vec<i32> {
    fn apply_operation(&mut self, op: StackOp) {
        match op {
            StackOp::DealIntoNewStack => self.reverse(),
            StackOp::Cut(mut p) => {
                let p = if p < 0 {
                    self.len() - p.abs() as usize
                } else {
                    p as usize
                };
                let mut other = self.split_off(p);
                std::mem::swap(self, &mut other);
                self.append(&mut other);
            }
            StackOp::DealWithIncrement(inc) => {
                let inc = inc as usize;
                let mut new = vec![-1 as i32; self.len()];
                for (i, v) in self.iter().enumerate() {
                    new[(i * inc) % self.len()] = *v;
                }
                *self = new;
            }
        }
    }
}

impl From<String> for StackOp {
    fn from(s: String) -> Self {
        if s == "deal into new stack" {
            StackOp::DealIntoNewStack
        } else if s.starts_with("deal with increment") {
            StackOp::DealWithIncrement(s.split(" ").last().unwrap().parse::<i32>().unwrap())
        } else if s.starts_with("cut") {
            StackOp::Cut(s.split(" ").last().unwrap().parse::<i32>().unwrap())
        } else {
            panic!("unknown action {}", s);
        }
    }
}

fn main() {
    // for line in std::io::stdin().lock().lines() {
    // for line in input1().lines() {
    //     let line = line.unwrap();
    //     println!("{}", line);
    // }
    let (input, num) = input2();
    let ops: Vec<StackOp> = input.lines().map(|l| l.unwrap().into()).collect();
    println!("{:?}", ops);

    let mut deck: Vec<i32> = std::iter::successors(Some(0_i32), |n| Some(n + 1))
        .take(num)
        .collect();
    for op in ops {
        deck.apply_operation(op);
    }
    if deck.len() == 10 {
        println!("{:?}", deck);
    } else {
        for (i, card) in deck.iter().enumerate() {
            if *card == 2019 {
                println!("2019 is in pos {}", i);
            }
        }
        //println!("2019 {}", deck[2019]);
    }

    // println!("{:?}", aoc2019::util::get_prime_factors(119315717514047));
}

fn input1_1() -> (&'static [u8], usize) {
    (
        "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
"
        .as_bytes(),
        10,
    )
}

fn input1_2() -> (&'static [u8], usize) {
    (
        "deal with increment 7
deal into new stack
deal into new stack
"
        .as_bytes(),
        10,
    )
}

fn input1_3() -> (&'static [u8], usize) {
    (
        "cut 6
deal with increment 7
deal into new stack
"
        .as_bytes(),
        10,
    )
}

fn input2() -> (&'static [u8], usize) {
    (
        "deal with increment 65
deal into new stack
deal with increment 25
cut -6735
deal with increment 3
cut 8032
deal with increment 71
cut -4990
deal with increment 66
deal into new stack
cut -8040
deal into new stack
deal with increment 18
cut -8746
deal with increment 42
deal into new stack
deal with increment 17
cut -8840
deal with increment 55
cut -4613
deal with increment 10
cut -5301
deal into new stack
deal with increment 21
cut -5653
deal with increment 2
cut 5364
deal with increment 72
cut -3468
deal into new stack
cut -9661
deal with increment 63
cut 6794
deal with increment 43
cut 2935
deal with increment 66
cut -1700
deal with increment 6
cut 5642
deal with increment 64
deal into new stack
cut -5699
deal with increment 43
cut -9366
deal with increment 42
deal into new stack
cut 2364
deal with increment 13
cut 8080
deal with increment 2
deal into new stack
cut -9602
deal with increment 51
cut 3214
deal into new stack
cut -2995
deal with increment 57
cut -8169
deal into new stack
cut 362
deal with increment 41
cut -4547
deal with increment 56
cut -1815
deal into new stack
cut 1554
deal with increment 71
cut 2884
deal with increment 44
cut -2423
deal with increment 4
deal into new stack
deal with increment 20
cut -2242
deal with increment 48
cut -716
deal with increment 29
cut -6751
deal with increment 45
cut -9511
deal with increment 75
cut -440
deal with increment 35
cut 6861
deal with increment 52
cut -4702
deal into new stack
deal with increment 28
cut 305
deal with increment 16
cut 7094
deal into new stack
cut 5175
deal with increment 30
deal into new stack
deal with increment 61
cut 1831
deal into new stack
deal with increment 25
cut 4043
"
        .as_bytes(),
        10007,
    )
}
