use std::sync::mpsc::{Sender, Receiver, channel};

pub struct Prices<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}
