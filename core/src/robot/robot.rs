use std::{io};
use crate::prelude::*;
use downcast::_std::time::Duration;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    NONE,
    DISABLED,
    TELEOP,
    AUTO,
    TEST,
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    START,
    DISABLE,
    TELEOP,
    AUTO,
    TEST,
}

pub struct Robot {
    dispatcher: Dispatcher,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            dispatcher: Dispatcher::new(),
        }
    }

    pub fn with<S: 'static + Subsystem>(mut self, s: S) -> Self {
        self.dispatcher.add(s);
        self
    }

    pub fn on_start<F>(mut self, f: F) -> Self where F: Fn() + 'static {
        self
    }

    pub fn get<S: 'static + Subsystem>(&self) -> Option<&S> {
        self.dispatcher.get::<S>()
    }

    pub fn run(self) -> Result<(), io::Error> {
        unimplemented!();
    }

    pub fn simulate(mut self) -> Result<(), &'static str> {

        loop {

        }

        Ok(())
    }
}
