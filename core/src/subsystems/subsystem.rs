use downcast::*;

pub trait Subsystem: Any {
    fn on_start(&self) {}
}

downcast!(Subsystem);
