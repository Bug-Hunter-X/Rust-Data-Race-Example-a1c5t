use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(5));
    let y = x.clone();
    let z = x.clone();

    let y_thread = std::thread::spawn(move || {
        let mut x_mut = y.lock().unwrap();
        *x_mut = 6;
    });

    let z_thread = std::thread::spawn(move || {
        let mut x_mut = z.lock().unwrap();
        *x_mut = 7;
    });

    y_thread.join().unwrap();
    z_thread.join().unwrap();

    println!("x = {}", *x.lock().unwrap());
}