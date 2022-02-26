#![allow(dead_code)]
use std::fmt::Debug;
use std::time::Instant;

trait Operation: Debug {
    fn perform(&self);
}

#[derive(Debug)]
struct Notification {
    message: String,
}

impl Notification {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Operation for Notification {
    fn perform(&self) {
        println!("{}", self.message)
    }
}

#[derive(Debug)]
struct TimeIt<T: Operation> {
    inner: T,
}

impl<T: Operation> TimeIt<T> {
    fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Operation> Operation for TimeIt<T> {
    fn perform(&self) {
        let start = Instant::now();
        println!("starting {:?}", self.inner);

        self.inner.perform();

        let finish = Instant::now();
        let duration = finish - start;
        println!("operation took {}ns", duration.as_nanos())
    }
}

#[cfg(test)]
mod test {
    use crate::decorator::{Notification, Operation, TimeIt};
    #[test]
    fn test_decorator() {
        let notification = Notification::new("hello world");
        notification.perform();

        let decorated = TimeIt::new(notification);
        decorated.perform();
    }
}
