use std::io;
use crate::subsystems::{Dispatcher, Subsystem};
use tokio::{
    runtime::Runtime,
    spawn,
};
use tokio::time::{Delay, delay_for};
use downcast::_std::time::Duration;

pub struct Robot {
    dispatcher: Dispatcher,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            dispatcher: Dispatcher::new(),
        }
    }

    pub fn with<S: 'static + Subsystem>(&mut self, s: S) -> &mut Self {
        self.dispatcher.add(s);
        self
    }

    pub fn on_start<F>(&mut self, f: F) -> &mut Self where F: FnOnce(Context) {
        println!("Starting Robot...");
        let ctx = Context::from(self);
        f(ctx);
        self
    }

    pub fn get<S: 'static + Subsystem>(&self) -> Option<&S> {
        self.dispatcher.get::<S>()
    }

    pub fn run(&self) -> Result<(), io::Error> {
        let mut rt = Runtime::new()?;

        rt.block_on(async {
            loop {
                delay_for(Duration::from_millis(20)).await;

            };
        });

        Ok(())
    }
}

pub struct Context<'a> {
    robot: &'a mut Robot,
}

impl<'a> Context<'a> {
    fn from(robot: &'a mut Robot) -> Self {
        Self {
            robot,
        }
    }

    pub fn get<S: 'static + Subsystem>(&self) -> Option<&S> {
        self.robot.get::<S>()
    }
}
