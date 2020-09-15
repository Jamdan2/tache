use crate::subsystems::{Dispatcher, Subsystem};

pub struct Robot {
    dispatcher: Dispatcher,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            dispatcher: Dispatcher::new(),
        }
    }

    pub fn add<S: 'static + Subsystem>(&mut self, s: S) {
        self.dispatcher.add(s);
    }
}

macro_rules! robot {
    ($( with $s:expr ),*) => {{
        let robot = Robot::new();
        $( robot.add($s) )*
    }};
}
