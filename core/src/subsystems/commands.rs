use crate::subsystems::{Subsystem, Dispatcher};

pub struct Cmd<'a> {
    dispatcher: &'a mut Dispatcher,
    f: Box<dyn Fn(&Cmd) -> ()>,
    reqs: Vec<&'a dyn Subsystem>,
}

impl<'a> Cmd<'a> {
    pub fn new<F: 'static + Fn(&Cmd)>(dispatcher: &'a mut Dispatcher, f: F) -> Self {
        Self {
            dispatcher,
            f: Box::new(f),
            reqs: Vec::new(),
        }
    }

    fn reserve<S: 'static + Subsystem>(&mut self) {
        let s: Option<&mut S> = unsafe { self.dispatcher.get_mut::<S>() };
        self.dispatcher.
    }
}

pub struct Ctx {

}
