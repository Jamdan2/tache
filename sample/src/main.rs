extern crate tache;

use tache::prelude::*;

struct Arm;

impl Arm {
    pub fn run(&mut self, output: i32) {
        println!("Running arm at {}!", output);
    }
}

impl Subsystem for Arm {
    fn on_start(&self) {
        println!("Starting the arm!");
    }
}

fn main() {
    let mut d = Dispatcher::new();
    d.add(Arm);
    d.for_each(|s| {
        s.on_start();
    });
    let s = d.get::<Arm>();
}

fn somethin<S: Subsystem>(d: &mut Dispatcher) {
    let s: Option<&S> = d.get();
}
