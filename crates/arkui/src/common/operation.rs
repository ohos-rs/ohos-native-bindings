//! Exclusive UI-thread operations without retaining a RefCell borrow over callbacks.

use std::cell::Cell;

#[derive(Default)]
pub(crate) struct Operation(Cell<bool>);

impl Operation {
    pub(crate) fn enter(&self) -> Option<Active<'_>> {
        if self.0.replace(true) {
            None
        } else {
            Some(Active(&self.0))
        }
    }
}

pub(crate) struct Active<'a>(&'a Cell<bool>);
impl Drop for Active<'_> {
    fn drop(&mut self) {
        self.0.set(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reentry_is_rejected_until_the_outer_operation_returns_or_unwinds() {
        let operation = Operation::default();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _active = operation.enter().unwrap();
            assert!(operation.enter().is_none());
            panic!("callback panicked");
        }));
        assert!(operation.enter().is_some());
    }
}
