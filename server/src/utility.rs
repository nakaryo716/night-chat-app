use std::sync::{Mutex, MutexGuard};

pub fn acquire_lock<T>(mutex: &Mutex<T>) -> Option<MutexGuard<T>> {
    match mutex.lock() {
        Ok(a) => Some(a),
        Err(_e) => None,
    }
}
