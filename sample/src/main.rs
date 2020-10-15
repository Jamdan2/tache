
extern crate tache;
#[macro_use]
extern crate tokio;

use tache::prelude::*;
use tache::state::{State};
use tache::listen;

// test changes

fn main() {
    Robot::new()
        .on_start(|| {
            println!("Hello, world!");
        })
        .simulate();
}
