
use tokio::sync::{
    mpsc::{self, Sender, Receiver},
    Mutex,
};
use crate::events::Bus;

pub trait Events<E: Copy> {
    fn bus(&mut self) -> &mut Bus<E>;

    fn fire(&mut self, e: E) {
        self.bus().fire(e);
    }

    fn listen<F>(&mut self, f: F) where F: Fn(E) + 'static {
        self.bus().listen(f);
    }
}

#[macro_export]
macro_rules! listen(
    ($events:expr, $($pat:pat => $expr:expr),* $(,)?) => {
        $events.listen(|e| {
            match e {
                $($pat => $expr),*,
                _ => (),
            };
        });
    };
);

#[macro_export]
macro_rules! fire(
    ($events:expr, $event:expr) => {
        $events.fire($event);
    };
);
