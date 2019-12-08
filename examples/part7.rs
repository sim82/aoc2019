use permutohedron::LexicalPermutation;
use std::io::BufRead;
use std::io::Write;
use std::iter::FromIterator;

trait Interpreter {
    fn run(&mut self) -> bool;
    fn halted(&self) -> bool;
}

struct Context {
    ip: usize,
    break_on_output: bool,
    data: Vec<i32>,
}

impl Context {
    pub fn new(data: Vec<i32>) -> Self {
        Context {
            ip: 0,
            break_on_output: false,
            data: data,
        }
    }
    fn break_on_output(mut self) -> Self {
        self.break_on_output = true;
        self
    }
}

struct Io<'a> {
    input: &'a mut dyn std::io::BufRead,
    output: &'a mut dyn std::io::Write,
}
impl<'a> Io<'a> {
    pub fn default(
        input: &'a mut dyn std::io::BufRead,
        output: &'a mut dyn std::io::Write,
    ) -> Self {
        Io {
            input: input,
            output: output,
        }
    }
}

impl Interpreter for (&mut Context, &mut Io<'_>) {
    fn run(&mut self) -> bool {
        let (context, io) = self;
        let data = &mut context.data;

        while context.ip < data.len() {
            let opcode = data[context.ip] % 100;
            let mut modes = vec![false; 0];
            let mut modenum = data[context.ip] / 100;
            // println!("opcode {}", opcode);

            while modenum != 0 {
                modes.push(modenum % 10 == 1);
                modenum /= 10;
            }
            // if !modes.is_empty() {
            //     println!("modes {:?}", modes);
            // }
            if modes.len() < 3 {
                modes.resize(3, false);
            }
            match opcode {
                // ==================== ALU
                1 | 2 | 7 | 8 => {
                    let a = if !modes[0] {
                        &data[data[context.ip + 1] as usize]
                    } else {
                        &data[context.ip + 1]
                    };
                    let b = if !modes[1] {
                        &data[data[context.ip + 2] as usize]
                    } else {
                        &data[context.ip + 2]
                    };
                    if modes[2] {
                        panic!("bad output mode 1");
                    }
                    let c = data[context.ip + 3] as usize;
                    match opcode {
                        1 => data[c] = *a + *b,
                        2 => data[c] = *a * *b,
                        7 => data[c] = (*a < *b).into(),
                        8 => data[c] = (*a == *b).into(),

                        _ => panic!("bad opcode {}", opcode),
                    }
                    context.ip += 4;
                }
                // ==================== input
                3 => {
                    let mut input: String = "".into();
                    io.input.read_line(&mut input);
                    let input = input.trim().parse::<i32>().unwrap();
                    // println!("input: {}", input);

                    let c = data[context.ip + 1] as usize;
                    data[c] = input;
                    context.ip += 2;
                }
                // ==================== output
                4 => {
                    let a = if !modes[0] {
                        &data[data[context.ip + 1] as usize]
                    } else {
                        &data[context.ip + 1]
                    };
                    writeln!(io.output, "{}", *a);
                    context.ip += 2;
                    if context.break_on_output {
                        break;
                    }
                }
                // ==================== cond jump
                5 | 6 => {
                    let a = if !modes[0] {
                        &data[data[context.ip + 1] as usize]
                    } else {
                        &data[context.ip + 1]
                    };
                    let b = if !modes[1] {
                        &data[data[context.ip + 2] as usize]
                    } else {
                        &data[context.ip + 2]
                    };
                    let do_jump = match opcode {
                        5 => *a != 0,
                        6 => *a == 0,
                        _ => panic!("bad opcode {}", opcode),
                    };
                    if do_jump {
                        context.ip = *b as usize;
                    // println!("jump to {}", context.ip);
                    } else {
                        context.ip += 3;
                    }
                }
                // ==================== break
                99 => {
                    break;
                }
                _ => panic!("bad opcode {}", opcode),
            }
        }
        if context.ip >= context.data.len() {
            panic!("ip out of range");
        }
        // println!("finished: {}", finished);
        self.halted()
    }

    fn halted(&self) -> bool {
        let (context, _) = self;

        return context.data[context.ip] == 99;
    }
}

fn main() {
    if true {
        if true {}
        // let mut data = vec![3, 2, 1002, 6, 3, 6, 33];
        // let data = data2();
    }
    day2();
    assert_eq!(day5(), "0\n0\n0\n0\n0\n0\n0\n0\n0\n15426686\n11430197\n");
    assert_eq!(day7(), 366376);
    assert_eq!(day7_1(), 21596786);
}
fn day2() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    {
        let data = code2();
        let mut input = stdin.lock();
        let mut output = stdout.lock();
        let mut io = Io::default(&mut input, &mut output);
        let mut context = Context::new(data);
        (&mut context, &mut io).run();
        // println!("data[0] = {}", context.data[0]);
        assert_eq!(context.data[0], 3716250);
    }
    for i in 0..99 {
        for j in 0..99 {
            let data = code2();
            let mut input = stdin.lock();
            let mut output = stdout.lock();
            let mut io = Io::default(&mut input, &mut output);
            let mut context = Context::new(data);
            context.data[1] = i;
            context.data[2] = j;
            (&mut context, &mut io).run();
            if context.data[0] == 19690720 {
                //println!("answer 2: {}", 100 * i + j);
                assert_eq!(100 * i + j, 6472);
            }
        }
    }
}
fn day5() -> String {
    let input_string = "1\n5\n";
    let mut output = Vec::<u8>::new();
    let mut input = input_string.as_bytes();
    for i in 0..2 {
        // let mut input = stdin.lock();
        // let mut output = stdout.lock();
        let mut io = Io::default(&mut input, &mut output);
        let mut context = Context::new(code5());
        (&mut context, &mut io).run();
        // println!("data: {:?}", data);
    }
    String::from_utf8(output).unwrap()
}
fn day7() -> i32 {
    let mut seq = [0, 1, 2, 3, 4];
    let mut max_out = 0;
    let mut max_seq = [0; 5];
    loop {
        let mut next_input = "0\n".to_string();
        for (i, setting) in seq.iter().enumerate() {
            let input_string = format!("{}\n{}", setting, next_input);
            let mut input = input_string.as_bytes();
            let mut output = Vec::<u8>::new();
            let mut io = Io::default(&mut input, &mut output);
            let mut context = Context::new(code7());
            (&mut context, &mut io).run();
            next_input = String::from_utf8(output).unwrap();
        }
        let out_val = next_input.trim().parse::<i32>().unwrap();
        if out_val > max_out {
            max_out = out_val;
            max_seq = seq.clone();
        }
        // println!("{:?}: {}", seq, next_input);
        if !seq.next_permutation() {
            break;
        }
    }
    // println!("max: {:?} {}", max_seq, max_out);
    max_out
}
fn day7_1() -> i32 {
    let mut seq = [5, 6, 7, 8, 9];
    // let mut seq = [9, 8, 7, 6, 5];
    let mut max_out = 0;
    let mut max_seq = [0; 5];
    loop {
        let mut next_input = "0\n".to_string();
        let mut state = seq
            .iter()
            .map(|x| {
                (
                    format!("{}\n", x).to_string(),
                    Vec::<u8>::new(),
                    Context::new(code71()).break_on_output(),
                    false,
                )
            })
            .collect::<Vec<_>>();
        loop {
            let mut num_run = 0;
            for (i, ios) in &mut state.iter_mut().enumerate() {
                if !ios.3 {
                    ios.0.push_str(&next_input);
                    // println!("stage {} input: {}", i, ios.0);
                    let mut input = ios.0.as_bytes();
                    let mut io = Io::default(&mut input, &mut ios.1);
                    ios.3 = (&mut ios.2, &mut io).run();
                    ios.0 = String::from_utf8(input.into()).unwrap();
                    num_run += 1;
                    // println!(
                    //     "halt: {} {}",
                    //     ios.3,
                    //     String::from_utf8(ios.1.clone()).unwrap()
                    // );
                    // if !ios.3 {
                    next_input = String::from_utf8(ios.1.clone()).unwrap();
                    // }
                }
                if !ios.3 {
                    ios.1.clear();
                }
            }
            if num_run == 0 {
                // println!("break: {}", next_input);
                break;
            }
            // println!("feedback: {}", next_input);
        }
        let out_val = next_input.trim().parse::<i32>().unwrap();
        if out_val > max_out {
            max_out = out_val;
            max_seq = seq.clone();
        }
        // println!("{:?}: {}", seq, next_input);
        if !seq.next_permutation() {
            break;
        }
    }
    // println!("max: {:?} {}", max_seq, max_out);
    max_out
}
fn code2() -> Vec<i32> {
    vec![
        1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 2, 9, 19, 23, 2, 23, 10, 27,
        1, 6, 27, 31, 1, 31, 6, 35, 2, 35, 10, 39, 1, 39, 5, 43, 2, 6, 43, 47, 2, 47, 10, 51, 1,
        51, 6, 55, 1, 55, 6, 59, 1, 9, 59, 63, 1, 63, 9, 67, 1, 67, 6, 71, 2, 71, 13, 75, 1, 75, 5,
        79, 1, 79, 9, 83, 2, 6, 83, 87, 1, 87, 5, 91, 2, 6, 91, 95, 1, 95, 9, 99, 2, 6, 99, 103, 1,
        5, 103, 107, 1, 6, 107, 111, 1, 111, 10, 115, 2, 115, 13, 119, 1, 119, 6, 123, 1, 123, 2,
        127, 1, 127, 5, 0, 99, 2, 14, 0, 0,
    ]
}
fn code5() -> Vec<i32> {
    vec![
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 31, 68, 225, 1001, 13, 87, 224,
        1001, 224, -118, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 7, 224, 1, 223, 224, 223, 1,
        174, 110, 224, 1001, 224, -46, 224, 4, 224, 102, 8, 223, 223, 101, 2, 224, 224, 1, 223,
        224, 223, 1101, 13, 60, 224, 101, -73, 224, 224, 4, 224, 102, 8, 223, 223, 101, 6, 224,
        224, 1, 224, 223, 223, 1101, 87, 72, 225, 101, 47, 84, 224, 101, -119, 224, 224, 4, 224,
        1002, 223, 8, 223, 1001, 224, 6, 224, 1, 223, 224, 223, 1101, 76, 31, 225, 1102, 60, 43,
        225, 1102, 45, 31, 225, 1102, 63, 9, 225, 2, 170, 122, 224, 1001, 224, -486, 224, 4, 224,
        102, 8, 223, 223, 101, 2, 224, 224, 1, 223, 224, 223, 1102, 29, 17, 224, 101, -493, 224,
        224, 4, 224, 102, 8, 223, 223, 101, 1, 224, 224, 1, 223, 224, 223, 1102, 52, 54, 225, 1102,
        27, 15, 225, 102, 26, 113, 224, 1001, 224, -1560, 224, 4, 224, 102, 8, 223, 223, 101, 7,
        224, 224, 1, 223, 224, 223, 1002, 117, 81, 224, 101, -3645, 224, 224, 4, 224, 1002, 223, 8,
        223, 101, 6, 224, 224, 1, 223, 224, 223, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256,
        1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227,
        274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105,
        1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106,
        0, 0, 1105, 1, 99999, 8, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 329, 1001, 223, 1,
        223, 1108, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 344, 101, 1, 223, 223, 108, 677,
        226, 224, 102, 2, 223, 223, 1006, 224, 359, 101, 1, 223, 223, 7, 677, 226, 224, 102, 2,
        223, 223, 1005, 224, 374, 101, 1, 223, 223, 1007, 226, 677, 224, 102, 2, 223, 223, 1005,
        224, 389, 101, 1, 223, 223, 8, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 404, 1001, 223,
        1, 223, 1007, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 419, 101, 1, 223, 223, 1108,
        677, 677, 224, 1002, 223, 2, 223, 1005, 224, 434, 1001, 223, 1, 223, 1107, 226, 677, 224,
        102, 2, 223, 223, 1005, 224, 449, 101, 1, 223, 223, 107, 226, 226, 224, 102, 2, 223, 223,
        1006, 224, 464, 101, 1, 223, 223, 1108, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 479,
        1001, 223, 1, 223, 7, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 494, 1001, 223, 1, 223,
        1107, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 509, 101, 1, 223, 223, 107, 677, 677,
        224, 1002, 223, 2, 223, 1006, 224, 524, 101, 1, 223, 223, 1008, 677, 677, 224, 1002, 223,
        2, 223, 1006, 224, 539, 101, 1, 223, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1005, 224,
        554, 101, 1, 223, 223, 108, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 569, 101, 1, 223,
        223, 1008, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 584, 101, 1, 223, 223, 8, 677, 226,
        224, 1002, 223, 2, 223, 1005, 224, 599, 101, 1, 223, 223, 1007, 226, 226, 224, 1002, 223,
        2, 223, 1005, 224, 614, 101, 1, 223, 223, 1107, 226, 226, 224, 1002, 223, 2, 223, 1006,
        224, 629, 101, 1, 223, 223, 107, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 644, 1001,
        223, 1, 223, 1008, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 659, 101, 1, 223, 223, 108,
        677, 677, 224, 1002, 223, 2, 223, 1005, 224, 674, 1001, 223, 1, 223, 4, 223, 99, 226,
    ]
}

fn code7() -> Vec<i32> {
    // vec![
    //     3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    // ]

    // vec![
    //     3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
    //     0, 0,
    // ]
    // vec![
    //     3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
    //     31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    // ]
    vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 47, 64, 85, 106, 187, 268, 349, 430, 99999, 3,
        9, 1002, 9, 4, 9, 1001, 9, 4, 9, 1002, 9, 4, 9, 4, 9, 99, 3, 9, 1002, 9, 4, 9, 4, 9, 99, 3,
        9, 1001, 9, 3, 9, 102, 5, 9, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 101, 3, 9, 9, 102, 5, 9, 9,
        1001, 9, 4, 9, 102, 4, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 3, 9, 101, 2, 9, 9, 102, 4, 9, 9,
        101, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9,
        2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002,
        9, 2, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1,
        9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3,
        9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1,
        9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4,
        9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
        101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2,
        9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9,
        3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102,
        2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4,
        9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9,
        1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        99,
    ]
}

fn code71() -> Vec<i32> {
    // vec![
    //     3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
    //     1005, 28, 6, 99, 0, 0, 5,
    // ]
    // vec![
    //     3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
    //     54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
    //     1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    // ]
    vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 47, 64, 85, 106, 187, 268, 349, 430, 99999, 3,
        9, 1002, 9, 4, 9, 1001, 9, 4, 9, 1002, 9, 4, 9, 4, 9, 99, 3, 9, 1002, 9, 4, 9, 4, 9, 99, 3,
        9, 1001, 9, 3, 9, 102, 5, 9, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 101, 3, 9, 9, 102, 5, 9, 9,
        1001, 9, 4, 9, 102, 4, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 3, 9, 101, 2, 9, 9, 102, 4, 9, 9,
        101, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9,
        2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002,
        9, 2, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1,
        9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3,
        9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1,
        9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4,
        9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
        101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2,
        9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9,
        3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102,
        2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4,
        9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9,
        1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        99,
    ]
}
