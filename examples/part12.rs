use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Point3 { x, y, z }
    }
    pub fn zero() -> Self {
        Point3 { x: 0, y: 0, z: 0 }
    }
    pub fn abs(&self) -> Self {
        Point3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
    pub fn hreduce(&self) -> i32 {
        self.x + self.y + self.z
    }
    pub fn neg(&self) -> Point3 {
        Point3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl Add for &Point3 {
    type Output = Point3;

    fn add(self, other: &Point3) -> Point3 {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for Point3 {
    type Output = Point3;

    fn add(self, other: Point3) -> Point3 {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    pub pos: Point3,
    pub velocity: Point3,
}
fn gravity(me: i32, other: i32) -> i32 {
    (other - me).signum()
}
impl Body {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Body {
            pos: Point3::new(x, y, z),
            velocity: Point3::zero(),
        }
    }
    pub fn new2(x: i32, y: i32, z: i32, xvel: i32, yvel: i32, zvel: i32) -> Self {
        Body {
            pos: Point3::new(x, y, z),
            velocity: Point3::new(xvel, yvel, zvel),
        }
    }
    pub fn apply_gravity(&mut self, other: &Body) {
        self.velocity.x += gravity(self.pos.x, other.pos.x);
        self.velocity.y += gravity(self.pos.y, other.pos.y);
        self.velocity.z += gravity(self.pos.z, other.pos.z);
    }
    pub fn apply_velocity(&mut self) {
        self.pos = &self.pos + &self.velocity;
    }
    fn energy(&self) -> i32 {
        self.pos.abs().hreduce() * self.velocity.abs().hreduce()
    }
}
fn simulate(state: [Body; 4]) -> [Body; 4] {
    // println!("state: {:?}", state);
    // let mut state_new: Vec<_> = state
    //     .iter()
    //     .map(|me| {
    //         let mut me_new = (*me).clone();
    //         for other in &state {
    //             me_new.apply_gravity(other);
    //         }
    //         me_new
    //     })
    //     .collect();
    let mut state_new = state.clone();

    for me in &mut state_new {
        for other in &state {
            me.apply_gravity(other);
        }
        me.apply_velocity();
    }
    state_new
}
fn main() {
    // let mut state = [
    //     Body::new(-1, 0, 2),
    //     Body::new(2, -10, -7),
    //     Body::new(4, -8, 8),
    //     Body::new(3, 5, -1),
    // ];
    let mut state = [
        Body::new(-8, -10, 0),
        Body::new(5, 5, 10),
        Body::new(2, -7, 3),
        Body::new(9, -8, -3),
    ];
    // let mut state = [
    //     Body::new(9, 13, -8),
    //     Body::new(-3, 16, -17),
    //     Body::new(-4, 11, -10),
    //     Body::new(0, -2, -2),
    // ];
    {
        let mut state = state.clone();
        for _ in 0..100 {
            // println!("state: {:?}", state);
            state = simulate(state);
        }
        let energy = state.iter().map(|body| body.energy()).sum::<i32>();
        println!("energy: {}", energy);
    }

    let initial_state = state.clone();
    let mut cycles = [0 as i64; 4];
    let mut num_cycles = 0;
    // let mut last_state2: Option<[Body; 4]> = None;
    let mut last_state: Option<[Body; 4]> = None;
    let mut last_state2: Option<[Body; 4]> = None;

    for i in 0 as i64.. {
        state = simulate(state);

        if let Some(ls) = last_state2 {
            let mut num_rev = 0;
            for j in 0..4 {
                if state[j].velocity == ls[j].velocity.neg() {
                    // println!("reversal: {} {}", j, i + 1);
                    num_rev += 1;
                }
            }
            if num_rev == 4 {
                println!("reversal: {}", i + 1);
            }
        }
        last_state2 = last_state;
        last_state = Some(state.clone());

        if i % 100000000 == 0 {
            println!("{}", i as i64);
        }

        let mut x_eq = 0;
        let mut y_eq = 0;
        let mut z_eq = 0;
        for (j, (init, s)) in initial_state.iter().zip(&state).enumerate() {
            if init.pos.x == s.pos.x && init.velocity.x == s.velocity.x {
                // println!("cycle {} {}", j, i + 1);
                // cycles[j] = i + 1;
                // num_cycles += 1;
                x_eq += 1;
            }
            if init.pos.y == s.pos.y && init.velocity.y == s.velocity.y {
                // println!("cycle {} {}", j, i + 1);
                // cycles[j] = i + 1;
                // num_cycles += 1;
                y_eq += 1;
            }
            if init.pos.z == s.pos.z && init.velocity.z == s.velocity.z {
                // println!("cycle {} {}", j, i + 1);
                // cycles[j] = i + 1;
                // num_cycles += 1;
                z_eq += 1;
            }
        }

        if x_eq == 4 {
            println!("x cycle: {}", i + 1);
        }
        if y_eq == 4 {
            println!("y cycle: {}", i + 1);
        }
        if z_eq == 4 {
            println!("z cycle: {}", i + 1);
        }
        // let energy = state.iter().map(|body| body.energy()).sum::<i32>();
        // println!("energy: {} {}", i, energy);
        if state == initial_state {
            println!("cycle: {}", i + 1);
            break;
        }
    }
}
