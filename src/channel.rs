use std::boxed::FnBox;
use std::sync::mpsc;
use std::sync::Mutex;

pub type ClosureType = Box<FnBox() + Send>;

lazy_static! {
    pub static ref TX: Mutex<Option<mpsc::Sender<ClosureType>>> = Mutex::new(None);
}

pub fn make_channel() -> mpsc::Receiver<ClosureType> {
    let (tx, rx) = mpsc::channel::<ClosureType>();
    *TX.lock().unwrap() = Some(tx);
    rx
}

pub fn send_closure(closure: ClosureType) {
    if let Some(ref tx) = *TX.lock().unwrap() {
        tx.send(closure).unwrap();
    }
}