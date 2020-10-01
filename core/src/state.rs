
use crate::{
    events::{Bus, Events},
    fire,
};

pub struct State<T: Copy> {
    value: T,
    bus: Bus<T>,
}

impl<T: Copy> Events<T> for State<T> {
    fn bus(&mut self) -> &mut Bus<T> {
        &mut self.bus
    }
}

impl<T: Copy> State<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            bus: Bus::new(),
        }
    }

    pub fn get(&self) -> T {
        self.value
    }

    pub fn set(&mut self, value: T) {
        fire!(self, value);
        self.value = value;
    }

    pub fn on_change() {

    }
}
