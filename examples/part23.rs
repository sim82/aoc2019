use aoc2019::intcode::*;

// use futures::channel::mpsc::{channel, Receiver, Sender};
use std::collections::{HashMap, VecDeque};
use std::io::Write;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};
use std::thread::{spawn, JoinHandle};
struct Endpoint {
    addr: i64,
    tx: Sender<i64>,
    rx: Receiver<i64>,
    idle: Arc<Mutex<i64>>,
    num_idle: usize,
}

impl Io2 for Endpoint {
    fn read(&mut self) -> i64 {
        // if let Ok(idle) = self.idle.lock() {
        //     idle += 1;
        // }
        // std::thread::sleep(std::time::Duration::from_millis(10));
        match self.rx.try_recv() {
            Ok(i) => {
                if self.num_idle >= 50 {
                    if let Ok(mut idle) = self.idle.lock() {
                        *idle -= 1;
                    }
                    self.num_idle = 0;
                }
                // println!("ch({}) read {}", self.2, i);
                i
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                self.num_idle += 1;
                if self.num_idle == 50 {
                    if let Ok(mut idle) = self.idle.lock() {
                        *idle += 1;
                        println!("ch({}) idle: {}", self.addr, *idle);
                        std::io::stdout().flush().unwrap();
                    }
                }
                -1
            }
            Err(err) => panic!("ch({}) read failed: {}", self.addr, err),
        }
    }
    fn write(&mut self, v: i64) {
        // println!("ch({}) write {}", self.2, v);
        self.tx.send(v).unwrap()
    }
}
struct Nic {
    addr: i64,
    tx: Sender<i64>,
    rx: Receiver<i64>,
    join_handle: JoinHandle<()>,
}

fn start(addr: i64, code: Vec<i64>, mut idle: Arc<Mutex<i64>>) -> Nic {
    let (input_tx, input_rx) = channel::<i64>();
    let (output_tx, output_rx) = channel::<i64>();

    input_tx.send(addr).unwrap();
    let join_handle = spawn(move || {
        let mut context = Context::new(code);
        let mut io = (&output_tx, &input_rx, addr.clone());

        let mut io = Endpoint {
            addr,
            tx: output_tx,
            rx: input_rx,
            idle,
            num_idle: 0,
        };
        (&mut context, &mut io as &mut dyn Io2).run();
    });

    Nic {
        addr,
        tx: input_tx,
        rx: output_rx,
        join_handle,
    }
}

fn start_nat() -> (Nic, Arc<Mutex<i64>>) {
    let (input_tx, input_rx) = channel::<i64>();
    let (output_tx, output_rx) = channel::<i64>();

    let mut idle_tmp = Arc::new(Mutex::new(0i64));
    let mut idle = idle_tmp.clone();
    let join_handle = spawn(move || {
        let mut buf = smallvec::SmallVec::<[i64; 2]>::new();
        let mut last_y = None;
        loop {
            if let Ok(p) = input_rx.try_recv() {
                // println!("nat: {}", p);
                if buf.len() == 2 {
                    buf.clear();
                }
                buf.push(p);
                if buf.len() == 2 {
                    if let Some(last_y) = last_y {
                        if last_y == p {
                            println!("duplicate y: {}", p);
                        }
                    }
                    last_y = Some(p);
                }
            }
            if let Ok(idle) = idle.lock() {
                if *idle == 50 {
                    println!("wakeup");

                    output_tx.send(0).unwrap();
                    output_tx.send(buf[0]).unwrap();
                    output_tx.send(buf[1]).unwrap();
                }
            }
        }
    });

    (
        Nic {
            addr: 255,
            tx: input_tx,
            rx: output_rx,
            join_handle,
        },
        idle_tmp,
    )
}

fn main() {
    // let (disp_tx, disp_rx)
    let code = read_prog("examples/code23.int");
    println!("code: {:?}", code);
    // panic!("exit");
    let (nat, idle) = start_nat();
    let mut nics = Vec::new();
    for i in 0..50 {
        nics.push(start(i, code.clone(), idle.clone()));
    }
    nics.push(nat);
    // let mut routes = HashMap::<i64, (i64, i64)>::new();
    let mut out_bufs = HashMap::<i64, smallvec::SmallVec<[i64; 3]>>::new();
    let mut send_bufs = vec![smallvec::SmallVec::<[i64; 3]>::new(); 256];
    loop {
        for nic in &nics {
            if let Ok(p) = nic.rx.try_recv() {
                let i = nic.addr as usize;
                send_bufs[i].push(p);

                if send_bufs[i].len() == 3 {
                    // if send_bufs[i][0] == 255 {
                    //     println!("result: {}", send_bufs[i][2]);
                    //     panic!("result: {}", send_bufs[i][2]);
                    // }
                    let mut nic_id = send_bufs[i][0] as usize;
                    if nic_id == 255 {
                        nic_id = 50;
                    }
                    nics[nic_id].tx.send(send_bufs[i][1]);
                    nics[nic_id].tx.send(send_bufs[i][2]);
                    println!(
                        "send {} -> {} {} {}",
                        nic.addr, send_bufs[i][0], send_bufs[i][1], send_bufs[i][2]
                    );

                    send_bufs[i].clear();
                }
            }
        }
    }
}

fn code23() -> Vec<i64> {
    vec![include!("code23.int")]
}
