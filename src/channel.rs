/*
 * MIT License
 *
 * Copyright (c) 2017 Robert Swain <robert.swain@gmail.com
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::boxed::FnBox;
use std::sync::mpsc;
use std::sync::Mutex;

pub type ClosureType = Box<FnBox() + Send>;
pub type MessageType = Vec<ClosureType>;

lazy_static! {
    pub static ref TX: Mutex<Option<mpsc::SyncSender<MessageType>>> = Mutex::new(None);
    pub static ref TX_QUEUE: Mutex<Option<MessageType>> = Mutex::new(None);
}

pub fn make_channel() -> mpsc::Receiver<MessageType> {
    let (tx, rx) = mpsc::sync_channel::<MessageType>(1);
    *TX.lock().unwrap() = Some(tx);
    *TX_QUEUE.lock().unwrap() = Some(Vec::new());
    rx
}

pub fn send() {
    if let Some(ref mut tx_queue) = *TX_QUEUE.lock().unwrap() {
        if let Some(ref tx) = *TX.lock().unwrap() {
            let message: MessageType = tx_queue.drain(..).collect();
            tx.send(message).unwrap();
        }
    }
}

pub fn push(closure: ClosureType) {
    if let Some(ref mut tx_queue) = *TX_QUEUE.lock().unwrap() {
        tx_queue.push(closure);
    }
}