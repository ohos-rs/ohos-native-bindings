//! Only readable, healthy acquire fences authorize CPU access.

use crate::Error;

pub(crate) fn ready(events: i16) -> Result<bool, Error> {
    if events & (libc::POLLERR | libc::POLLNVAL | libc::POLLHUP) != 0 {
        return Err(Error {
            operation: "fence status",
            code: events as i32,
        });
    }
    Ok(events & libc::POLLIN != 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_pollin_without_error_or_hangup_signals_completion() {
        assert!(ready(libc::POLLIN).unwrap());
        assert!(!ready(0).unwrap());
        assert!(!ready(libc::POLLOUT).unwrap());
        for events in [
            libc::POLLHUP,
            libc::POLLERR,
            libc::POLLNVAL,
            libc::POLLIN | libc::POLLHUP,
        ] {
            assert!(ready(events).is_err());
        }
    }
}
