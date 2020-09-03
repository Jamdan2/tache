extern crate tache;

use tache::prelude::*;

struct A;

impl Subsystem for A {
    fn on_start(&self) {
        println!("Hello, world!");
    }
}

fn main() {
    let mut d = Dispatcher::new();
    d.subsystem(A);
    d.for_each(|w| {
        w.on_start();
    });
}
