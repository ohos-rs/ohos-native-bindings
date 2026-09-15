/// Owns a fixed-address sequence of linked nodes. No ABI declarations live here;
/// the target supplies the generated SDK node type and its link setter.
pub(crate) struct LinkedNodes<T> {
    nodes: Box<[T]>,
}

impl<T> LinkedNodes<T> {
    pub(crate) fn new(nodes: Vec<T>, mut link: impl FnMut(&mut T, *mut T)) -> Self {
        let mut nodes = nodes.into_boxed_slice();
        let start = nodes.as_mut_ptr();
        let count = nodes.len();
        for (index, node) in nodes.iter_mut().enumerate() {
            let next = if index + 1 == count {
                std::ptr::null_mut()
            } else {
                // SAFETY: The next node is in bounds in the fixed-size boxed
                // slice. Moving LinkedNodes never moves its allocation.
                unsafe { start.add(index + 1) }
            };
            link(node, next);
        }
        Self { nodes }
    }
    pub(crate) fn head(&mut self) -> *mut T {
        if self.nodes.is_empty() {
            std::ptr::null_mut()
        } else {
            self.nodes.as_mut_ptr()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestNode {
        value: usize,
        next: *mut TestNode,
    }
    #[test]
    fn zero_one_many_and_move_preserve_linked_storage() {
        for count in [0, 1, 16] {
            let list = LinkedNodes::new(
                (0..count)
                    .map(|value| TestNode {
                        value,
                        next: std::ptr::null_mut(),
                    })
                    .collect(),
                |node, next| node.next = next,
            );
            let mut moved = list;
            let mut cursor = moved.head();
            for value in 0..count {
                assert!(!cursor.is_null());
                // SAFETY: cursor comes from the still-live list and every next
                // link was set to an in-bounds node by LinkedNodes.
                let node = unsafe { &*cursor };
                assert_eq!(node.value, value);
                cursor = node.next;
            }
            assert!(cursor.is_null());
        }
    }
}
