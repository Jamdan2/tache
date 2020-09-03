use std::collections::HashMap;
use std::any::{TypeId, Any};

pub trait Subsystem {
    fn on_start(&self) {}
}
