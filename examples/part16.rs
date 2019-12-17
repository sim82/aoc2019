use std::iter;

fn pattern(repeat: usize) -> impl Iterator<Item = i32> {
    const pattern: [i32; 4] = [0, 1, 0, -1];

    let mut iter = iter::repeat_with(move || {
        pattern
            .iter()
            .flat_map(move |x| iter::repeat(x).take(repeat))
            .cloned()
    })
    .flatten();
    iter.next().unwrap();
    iter
}

fn intermingle(input: Vec<i32>) -> Vec<i32> {
    (0..input.len())
        .map(|i| {
            let pat = pattern(i + 1);
            input.iter().zip(pat).map(|(i, p)| i * p).sum::<i32>().abs() % 10
        })
        .collect()
}

fn intermingle2(input: Vec<i32>) -> Vec<i32> {
    let len = input.len();
    let mut out = vec![0 as i32; len];
    for i in 0..len {
        println!("{}", i);
        for (j, chunk) in input[i..].chunks(i + 1).enumerate() {
            match j % 4 {
                0 => out[i] += chunk.iter().sum::<i32>(),
                2 => out[i] -= chunk.iter().sum::<i32>(),
                _ => (),
            }
        }
    }

    for i in out.iter_mut() {
        *i = i.abs() % 10;
    }
    out
}

fn main() {
    {
        let len = 20;

        for i in 0..len {
            let line = pattern(i + 1)
                .take(len)
                .map(|x| match x {
                    0 => ' ',
                    1 => '+',
                    -1 => '-',
                    _ => panic!("unhandled"),
                })
                .collect::<String>();
            println!("{}", line);
        }
        return;
    }

    {
        let mut input: Vec<i32> = input_msg().collect();
        for i in 0..100 {
            input = intermingle2(input);
            // println!("{}", input.iter().map(|x| format!("{}", x)).join(""));
            // println!("{:?}", input);
        }
        for i in input.iter().take(8) {
            print!("{}", i);
        }
        println!("");
    }

    {
        let input: Vec<i32> = input_msg().collect();
        let mut input: Vec<i32> = iter::repeat(input).take(10000).flatten().collect();
        println!("len: {}", input.len());
        for i in 0..100 {
            println!("i: {}", i);
            input = intermingle2(input);
            // println!("{}", input.iter().map(|x| format!("{}", x)).join(""));
            // println!("{:?}", input);
        }
        for i in input.iter().take(8) {
            print!("{}", i);
        }
        println!("");
    }
    // for i in input {
    //     println!("{:?}", input[0..8]);
    // }
}

fn input_msg() -> impl Iterator<Item = i32> {
    const input : &str = "59773419794631560412886746550049210714854107066028081032096591759575145680294995770741204955183395640103527371801225795364363411455113236683168088750631442993123053909358252440339859092431844641600092736006758954422097244486920945182483159023820538645717611051770509314159895220529097322723261391627686997403783043710213655074108451646685558064317469095295303320622883691266307865809481566214524686422834824930414730886697237161697731339757655485312568793531202988525963494119232351266908405705634244498096660057021101738706453735025060225814133166491989584616948876879383198021336484629381888934600383957019607807995278899293254143523702000576897358";

    (0..input.len()).map(|i| input[i..(i + 1)].parse::<i32>().unwrap())
}
