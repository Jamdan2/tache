
pub mod prelude;
pub mod robot;

pub use crate::robot::{
    robot::{
        Robot,
        Event::{
            self,
            START,
            DISABLE,
            TELEOP,
            AUTO,
            TEST,
        }
    },
};
