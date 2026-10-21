//! Retryable ownership transfer from native child edges into pending disposal.

pub(crate) fn retire<T, E>(
    active: &mut Vec<T>,
    pending: &mut Vec<T>,
    mut detach: impl FnMut(&T) -> Result<(), E>,
    mut dispose: impl FnMut(&T) -> Result<(), E>,
) -> Result<(), E> {
    loop {
        while let Some(child) = pending.last() {
            dispose(child)?;
            pending.pop();
        }
        let Some(child) = active.last() else {
            return Ok(());
        };
        detach(child)?;
        // The native edge is already gone. Keep ownership separately so a
        // failed disposal retries without removing that edge a second time.
        if let Some(child) = active.pop() {
            pending.push(child);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_disposal_keeps_the_child_and_does_not_repeat_detachment() {
        let mut active = vec![7];
        let mut pending = Vec::new();
        let mut removed = 0;
        let result = retire(
            &mut active,
            &mut pending,
            |_| {
                removed += 1;
                Ok(())
            },
            |_| Err("retained"),
        );
        assert_eq!(result, Err("retained"));
        assert!(active.is_empty());
        assert_eq!(pending, [7]);
        retire(
            &mut active,
            &mut pending,
            |_| -> Result<(), &str> { panic!("repeated native removal") },
            |_| Ok(()),
        )
        .unwrap();
        assert_eq!(removed, 1);
        assert!(pending.is_empty());
    }

    #[test]
    fn failed_native_detachment_keeps_the_active_edge() {
        let mut active = vec![7];
        let mut pending = Vec::new();
        assert_eq!(
            retire(&mut active, &mut pending, |_| Err("attached"), |_| Ok(())),
            Err("attached")
        );
        assert_eq!(active, [7]);
        assert!(pending.is_empty());
    }
}
