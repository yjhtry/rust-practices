use std::{
    cell::RefCell,
    sync::{atomic::Ordering, Arc},
    thread::spawn,
};

struct Lock<T> {
    lock: std::sync::atomic::AtomicBool,
    data: RefCell<T>,
}

unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T> {
    fn new(data: T) -> Self {
        Lock {
            lock: std::sync::atomic::AtomicBool::new(false),
            data: RefCell::new(data),
        }
    }

    fn lock(&self, op: impl FnOnce(&mut T)) {
        while self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.lock.load(Ordering::Relaxed) == true {}
        }

        op(&mut *self.data.borrow_mut());

        self.lock.store(false, Ordering::Release);
    }
}

fn main() {
    let lock = Arc::new(Lock::new(0));

    let lock1 = lock.clone();
    spawn(move || {
        lock1.lock(|v| *v += 1);
        *lock1.data.borrow_mut() = 1;
    })
    .join()
    .unwrap();

    let lock2 = lock.clone();
    spawn(move || {
        lock2.lock(|v| *v += 1);
        *lock2.data.borrow_mut() = 2;
    })
    .join()
    .unwrap();

    println!("{}", *lock.data.borrow());
}
