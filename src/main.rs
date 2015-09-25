use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let v = Arc::new(Mutex::new(vec![1, 2, 3]));
    let t = thread::spawn(|| {
        let mut v = v.lock().unwrap();
        v.push(4);
    });
    //println!("{:?}", v);
    t.join();
    println!("{:?}", v);
}
