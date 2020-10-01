
extern crate tache;
#[macro_use]
extern crate tokio;

use tache::prelude::*;
use tache::state::{State};
use tache::listen;

fn main() {
    let mut state = State::new(true);

    listen!(state, value => {
        println!("{}", value);
    });



    state.set(false);
}
