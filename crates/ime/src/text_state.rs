use std::ops::Range;

/// A bounded snapshot for IME queries. All positions are UTF-16 document offsets.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextState {
    pub text: String,
    /// Document offset of the first character in `text`.
    pub offset: usize,
    pub selection: Range<usize>,
    pub reversed: bool,
}

impl TextState {
    pub(crate) fn cursor(&self) -> usize {
        if self.reversed {
            self.selection.start
        } else {
            self.selection.end
        }
    }

    pub(crate) fn before_cursor(&self, number: i32) -> String {
        let text: Vec<u16> = self.text.encode_utf16().collect();
        let end = self
            .selection
            .start
            .saturating_sub(self.offset)
            .min(text.len());
        utf16_substring(&text, end.saturating_sub(number.max(0) as usize)..end)
    }

    pub(crate) fn after_cursor(&self, number: i32) -> String {
        let text: Vec<u16> = self.text.encode_utf16().collect();
        let start = self
            .selection
            .end
            .saturating_sub(self.offset)
            .min(text.len());
        let end = start.saturating_add(number.max(0) as usize).min(text.len());
        utf16_substring(&text, start..end)
    }
}

fn utf16_substring(text: &[u16], mut range: Range<usize>) -> String {
    // A count or editor selection can land inside a surrogate pair. Omit a
    // partial character instead of returning an invalid UTF-16 sequence.
    if range.start < range.end && (0xdc00..=0xdfff).contains(&text[range.start]) {
        range.start += 1;
    }
    if range.start < range.end && (0xd800..=0xdbff).contains(&text[range.end - 1]) {
        range.end -= 1;
    }
    String::from_utf16_lossy(&text[range])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surrounding_text_excludes_selection_and_keeps_surrogate_pairs() {
        let state = TextState {
            text: "a😀中b".into(),
            offset: 100,
            selection: 103..104,
            reversed: true,
        };
        assert_eq!(state.cursor(), 103);
        assert_eq!(state.before_cursor(1), "");
        assert_eq!(state.before_cursor(2), "😀");
        assert_eq!(state.before_cursor(100), "a😀");
        assert_eq!(state.after_cursor(100), "b");
        assert_eq!(state.before_cursor(-1), "");
    }

    #[test]
    fn right_text_never_returns_half_an_emoji() {
        let state = TextState {
            text: "😀x".into(),
            ..Default::default()
        };
        assert_eq!(state.after_cursor(1), "");
        assert_eq!(state.after_cursor(2), "😀");
        assert_eq!(state.after_cursor(i32::MAX), "😀x");
    }
}
