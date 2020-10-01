
pub struct Bus<E: Copy> {
    listeners: Vec<Box<dyn Fn(E)>>,
}

impl<E: Copy> Bus<E> {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new()
        }
    }

    pub fn fire(&self, e: E) {
        for l in &self.listeners {
            let f = l.as_ref();
            f(e);
        }
    }

    pub fn listen<F>(&mut self, f: F) where F: Fn(E) + 'static {
        self.listeners.push(Box::new(f));
    }
}
