use std::sync::mpsc::{Sender, Receiver};


trait Consumer<T> {
    fn subscribe(tx: Sender<T>);
}
