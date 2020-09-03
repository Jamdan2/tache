use std::collections::HashMap;
use std::any::{Any, TypeId};
use crate::subsystems::{Subsystem, Cmd};
use std::intrinsics::transmute;

pub struct Dispatcher {
    workers: HashMap<TypeId, (Box<dyn Subsystem>,)>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            workers: HashMap::new(),
        }
    }

    pub fn add<S: 'static + Subsystem>(&mut self, subsystem: S) {
        self.workers.insert(subsystem.type_id(), (Box::new(subsystem),));
    }

    pub fn get<S: 'static + Subsystem>(&self) -> Option<&S> {
        self.workers.get(&TypeId::of()).map(|(s)| {
            unsafe { transmute::<&Box<dyn Subsystem>, &Box<S>>(s) }.as_ref()
        })
    }

    pub fn for_each<F: Fn(&dyn Subsystem)>(&self, f: F) {
        for (_, v) in self.workers.iter() {
            let worker = v.0.as_ref();
            f(worker);
        }
    }

    pub fn exec<F: Fn()>(&mut self, cmd: Cmd) {

    }
}
