#![feature(try_from)]
mod raft;

use std::thread;

use raft::core::Raft;
use raft::rpc::RpcSender;
use raft::timer::{Timer, CancellationReason};

fn main() {
    thread::spawn(|| {
        let mut raft = Raft::new(1, None);
        raft.init();
        println!("{:?}", raft);
        thread::sleep(std::time::Duration::from_millis(5000));
        println!("{:?}", raft)
    });

    thread::spawn(|| {
        let mut raft = Raft::new(2, None);
        thread::sleep(std::time::Duration::from_millis(1000));
        raft.rpc.connect(1).unwrap();
        raft.rpc.request_vote(1, 0, 0).unwrap();
    });

    thread::sleep(std::time::Duration::from_millis(10000));
}
