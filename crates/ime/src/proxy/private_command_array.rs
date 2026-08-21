/// Copy NDK private-command pointers into wrappers.
///
/// This helper only copies pointers and does not call NDK APIs, so it can be
/// unit-tested on the host with dummy pointers.
pub(crate) fn collect_private_commands<T, W>(
    commands: &[*mut T],
    wrap: impl Fn(*mut T) -> W,
) -> Vec<W> {
    commands.iter().copied().map(wrap).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_private_commands_copies_two_dummy_pointers() {
        let mut first = 0u8;
        let mut second = 1u8;
        let commands = [&mut first as *mut u8, &mut second as *mut u8];

        let wrapped = collect_private_commands(&commands, |raw| raw);

        assert_eq!(wrapped.len(), 2);
        assert_eq!(wrapped[0], commands[0]);
        assert_eq!(wrapped[1], commands[1]);
    }

    #[test]
    fn collect_private_commands_empty_len_is_empty() {
        let commands: [*mut u8; 0] = [];

        let wrapped = collect_private_commands(&commands, |raw| raw);

        assert_eq!(wrapped.len(), 0);
    }
}
