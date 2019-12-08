// use std::{thread, time};

pub trait SpaceImg {
    fn decode(&self, width: usize, height: usize) -> Self;
    fn draw(&self, width: usize, height: usize);
}

impl SpaceImg for Vec<i32> {
    fn decode(&self, width: usize, height: usize) -> Self {
        let size = width * height;
        let mut it = self.chunks(size).rev();
        let first = it.next().unwrap().to_vec();
        it.fold(first, |mut acc, layer| {
            acc.iter_mut()
                .zip(layer.iter())
                .for_each(|(x, y)| *x = if *y != 2 { *y } else { *x });
            // acc.draw(width, height);
            // thread::sleep(time::Duration::from_millis(20));
            acc
        })
    }

    fn draw(&self, width: usize, height: usize) {
        for (y, line) in self.chunks(width).enumerate() {
            println!(
                "{}",
                String::from_utf8(
                    line.iter()
                        .map(|x| match *x {
                            0 => ' ',
                            1 => 'X',
                            2 => '.',
                            _ => '?',
                        } as u8)
                        .collect()
                )
                .unwrap()
            );

            if y >= height && y % height == 0 {
                println!("---");
            }
        }
    }
}
