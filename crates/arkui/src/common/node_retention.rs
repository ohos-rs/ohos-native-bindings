//! Native-owner guards shared by mounting and retained registrations.

use std::{cell::Cell, rc::Rc};

#[derive(Default, Clone)]
pub(crate) struct NativeRetentions(Rc<Cell<usize>>);

impl NativeRetentions {
    pub(crate) fn retain(&self) -> Option<NodeRetention> {
        self.0.set(self.0.get().checked_add(1)?);
        Some(NodeRetention(self.0.clone()))
    }

    pub(crate) fn is_retained(&self) -> bool {
        self.0.get() != 0
    }
}

pub(crate) struct NodeRetention(Rc<Cell<usize>>);

impl Drop for NodeRetention {
    fn drop(&mut self) {
        self.0.set(self.0.get() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_owner_stays_retained_until_every_guard_retires() {
        let owner = NativeRetentions::default();
        let alias = owner.clone();
        let first = owner.retain().unwrap();
        let second = alias.retain().unwrap();
        assert!(owner.is_retained());
        drop(first);
        assert!(alias.is_retained());
        drop(second);
        assert!(!owner.is_retained());
    }

    #[test]
    fn a_rejected_retention_does_not_wrap_the_count() {
        let owner = NativeRetentions(Rc::new(Cell::new(usize::MAX)));
        assert!(owner.retain().is_none());
        assert!(owner.is_retained());
    }
}
