
extern crate tache;
#[macro_use]
extern crate tokio;

use tache::prelude::*;
use tache::listen;

#[tokio::main]
fn main() {
    println!("Yee");

    Robot::new()
        .on_start(|| {
            println!("Hello, world!");
        })
        .simulate();

    for e in robot.events() {
        match e {
            Teleop => {

            }
        }
    }
}
