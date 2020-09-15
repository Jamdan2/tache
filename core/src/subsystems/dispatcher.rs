use std::collections::HashMap;
use std::any::{Any, TypeId};
use crate::subsystems::{Subsystem};
use std::intrinsics::transmute;
use downcast::Downcast;
use futures::Future;

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
        let id = TypeId::of::<S>();
        let s = self.workers.get(&id)?.0.as_ref();
        Result::ok(s.downcast_ref())
    }

    pub unsafe fn get_mut<S: 'static + Subsystem>(&mut self) -> Option<&mut S> {
        let id = TypeId::of::<S>();
        let s = self.workers.get_mut(&id)?.0.as_mut();
        Result::ok(s.downcast_mut())
    }

    pub fn for_each<F: Fn(&dyn Subsystem)>(&self, f: F) {
        for (_, v) in self.workers.iter() {
            let worker = v.0.as_ref();
            f(worker);
        }
    }
}
