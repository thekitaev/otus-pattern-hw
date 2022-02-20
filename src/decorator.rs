#![allow(dead_code)]

use std::time::Instant;

pub trait Operation {
    fn perform(&self);
}

pub struct Notification {
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

pub struct TimeIt<T: Operation> {
    inner: T,
}

impl<T> TimeIt<T>
where
    T: Operation,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
    pub fn perform(&self) {
        let start = Instant::now();
        self.inner.perform();
        let finish = Instant::now();
        let duration = finish - start;
        println!("operation took {}ns", duration.as_nanos())
    }
}

#[cfg(test)]
mod test {
    use crate::decorator::{Notification, TimeIt};
    #[test]
    fn test_decorator() {
        let notification = Notification::new("hello world");
        let decorated = TimeIt::new(notification);
        decorated.perform()
    }
}
