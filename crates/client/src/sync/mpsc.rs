use std::sync::mpsc::{Receiver, Sender};

pub use std::sync::mpsc::SendError;
pub use std::sync::mpsc::TryRecvError;

pub struct Channel<T> {
    s: Sender<T>,
    r: Receiver<T>,
}
impl<T> Channel<T> {
    pub fn send(&self, t: T) -> Result<(), SendError<T>> {
        self.s.send(t)
    }

    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.r.try_recv()
    }

    pub fn sender(&self) -> &Sender<T> {
        &self.s
    }
}
impl<T> Default for Channel<T> {
    fn default() -> Self {
        let (s, r) = std::sync::mpsc::channel();
        Channel { s, r }
    }
}
