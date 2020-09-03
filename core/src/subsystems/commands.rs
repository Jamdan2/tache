use crate::subsystems::{Subsystem, Dispatcher};

pub struct Cmd<'a> {
    dispatcher: &'a Dispatcher,
    f: Box<dyn Fn(&Cmd) -> ()>,
    subsystems: Vec<&'a dyn Subsystem>,
}

impl<'a> Cmd<'a> {
    fn new<F: 'static + Fn(&Cmd)>(dispatcher: &'a Dispatcher, f: F) -> Self {
        Self {
            dispatcher,
            f: Box::new(f),
            subsystems: Vec::new(),
        }
    }

    fn reserve<S: Subsystem>(&mut self) {
        let s = self.dispatcher;
    }
}

struct Ctx {

}
