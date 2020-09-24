
extern crate tache;
#[macro_use]
extern crate tokio;

use tache::prelude::*;

struct Arm;

impl Subsystem for Arm {}

fn main() {
    Robot::new()
        .with(Arm)
        .on_start(|ctx| {
            println!("Heya! I'm Starting!");
        })
        .run()
        .unwrap();
}
