use anyhow::anyhow;
use anyhow::Result;
use std::mem::swap;
use std::sync::atomic::Ordering;
use std::{
    collections::VecDeque,
    sync::{atomic::AtomicUsize, Arc, Condvar, Mutex},
};

const INITIAL_CAPACITY: usize = 32;

pub struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(INITIAL_CAPACITY)),
            available: Condvar::new(),
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
        }
    }
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) -> Result<()> {
        if self.total_receivers() == 0 {
            return Err(anyhow!("No more receivers"));
        }

        let empty = {
            let mut inner = self.shared.queue.lock().unwrap();
            let empty = inner.is_empty();
            inner.push_back(t);

            empty
        };

        if empty {
            self.shared.available.notify_one();
        }

        Ok(())
    }

    pub fn total_queued_items(&self) -> usize {
        let queue = self.shared.queue.lock().unwrap();
        queue.len()
    }

    pub fn total_receivers(&self) -> usize {
        self.shared.receivers.load(Ordering::SeqCst)
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.senders.fetch_add(1, Ordering::AcqRel);

        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let old = self.shared.senders.fetch_sub(1, Ordering::AcqRel);

        if old <= 1 {
            self.shared.available.notify_all();
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    cache: VecDeque<T>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        loop {
            if let Some(t) = self.cache.pop_front() {
                return Ok(t);
            }

            let mut inner = self.shared.queue.lock().unwrap();
            match inner.pop_front() {
                Some(t) => {
                    if !inner.is_empty() {
                        swap(&mut self.cache, &mut inner)
                    }
                    return Ok(t);
                }
                None if self.total_senders() == 0 => return Err(anyhow!("No more senders")),
                None => {
                    let _unused = self
                        .shared
                        .available
                        .wait(inner)
                        .map_err(|_| anyhow!("Lock poisoned"));
                }
            }
        }
    }

    pub fn total_senders(&self) -> usize {
        self.shared.senders.load(Ordering::SeqCst)
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.receivers.fetch_sub(1, Ordering::AcqRel);
    }
}

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared::default();
    let shared = Arc::new(shared);

    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared,
            cache: VecDeque::with_capacity(INITIAL_CAPACITY),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        thread::{sleep, spawn},
        time::Duration,
    };

    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();

        s.send("Hello world").unwrap();

        assert_eq!(r.recv().unwrap(), "Hello world");
    }

    #[test]
    fn multiple_senders_should_work() {
        let (mut s, mut r) = unbounded();

        let mut s1 = s.clone();
        let mut s2 = s.clone();

        let jh1 = spawn(move || {
            s1.send(1).unwrap();
        });

        let jh2 = spawn(move || {
            s2.send(2).unwrap();
        });

        let jh3 = spawn(move || {
            s.send(3).unwrap();
        });

        for jh in [jh1, jh2, jh3] {
            jh.join().unwrap();
        }

        let mut res = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap()];

        res.sort();

        assert_eq!(res, [1, 2, 3]);
    }

    #[test]
    fn receiver_should_be_block_when_nothing_read() {
        let (mut s, r) = unbounded();

        spawn(move || {
            for (idx, i) in r.into_iter().enumerate() {
                assert_eq!(i, idx);
            }

            assert!(false);
        });

        for i in 0..100 {
            s.send(i).unwrap();
        }

        sleep(Duration::from_millis(1000));

        for i in 100..200 {
            s.send(i).unwrap();
        }

        sleep(Duration::from_millis(1000));

        assert_eq!(s.total_queued_items(), 0);
    }

    #[test]
    fn last_sender_drop_should_error_when_receiver() {
        let (s, mut r) = unbounded();
        let s1 = s.clone();
        let senders = [s, s1];
        let total = senders.len();

        for mut s in senders {
            spawn(move || {
                s.send(1).unwrap();
            })
            .join()
            .unwrap();
        }

        for _ in 0..total {
            r.recv().unwrap();
        }

        assert!(r.recv().is_err());
    }

    #[test]
    fn receiver_drop_should_error_when_send() {
        let (mut s1, mut s2) = {
            let (s, _) = unbounded();
            let s1 = s.clone();
            let s2 = s.clone();
            (s1, s2)
        };

        assert!(s1.send(1).is_err());
        assert!(s2.send(1).is_err());
    }

    #[test]
    fn channel_fast_path_should_work() {
        let (mut s, mut r) = unbounded();

        for _ in 0..10 {
            s.send(1).unwrap();
        }

        assert!(r.cache.is_empty());

        assert_eq!(r.recv().unwrap(), 1);

        assert_eq!(r.cache.len(), 9);
    }
}
