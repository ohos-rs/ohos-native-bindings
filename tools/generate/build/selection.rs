use std::ffi::OsStr;
use std::fmt;

pub(crate) const CONFIG_ENV: &str = "OHOS_BINDINGS_GENERATE_CONFIG";

/// Resolves exact registry names before any generation or manifest writes.
pub(crate) struct ConfigSelector<'registry> {
    names: &'registry [&'registry str],
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SelectionError {
    Empty,
    NonUtf8,
    Unknown(String),
    Duplicate(String),
}

impl fmt::Display for SelectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{CONFIG_ENV}: ")?;
        match self {
            Self::Empty => formatter.write_str("expected one exact crate name, not an empty value"),
            Self::NonUtf8 => formatter.write_str("crate name must be valid UTF-8"),
            Self::Unknown(name) => write!(formatter, "unknown crate name {name:?}"),
            Self::Duplicate(name) => {
                write!(formatter, "ambiguous duplicate registry name {name:?}")
            }
        }
    }
}

impl<'registry> ConfigSelector<'registry> {
    pub(crate) fn new(names: &'registry [&'registry str]) -> Self {
        Self { names }
    }

    /// An unset selector preserves the full registry's declared order.
    pub(crate) fn select(&self, value: Option<&OsStr>) -> Result<Vec<usize>, SelectionError> {
        let Some(value) = value else {
            return Ok((0..self.names.len()).collect());
        };
        let name = value.to_str().ok_or(SelectionError::NonUtf8)?;
        if name.is_empty() {
            return Err(SelectionError::Empty);
        }
        let mut matches = self
            .names
            .iter()
            .enumerate()
            .filter_map(|(index, candidate)| (*candidate == name).then_some(index));
        let index = matches
            .next()
            .ok_or_else(|| SelectionError::Unknown(name.to_owned()))?;
        if matches.next().is_some() {
            return Err(SelectionError::Duplicate(name.to_owned()));
        }
        Ok(vec![index])
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfigSelector, SelectionError};
    use std::ffi::OsStr;

    const NAMES: &[&str] = &[
        "ohos-first-sys",
        "ohos-native-child-process-sys",
        "ohos-last-sys",
    ];

    #[test]
    fn unset_preserves_registry_order() {
        assert_eq!(ConfigSelector::new(NAMES).select(None), Ok(vec![0, 1, 2]));
    }

    #[test]
    fn exact_name_selects_only_one_config_deterministically() {
        let selector = ConfigSelector::new(NAMES);
        for _ in 0..8 {
            assert_eq!(
                selector.select(Some(OsStr::new("ohos-native-child-process-sys"))),
                Ok(vec![1])
            );
        }
    }

    #[test]
    fn rejects_empty_unknown_partial_multiple_and_whitespace_values() {
        let selector = ConfigSelector::new(NAMES);
        assert_eq!(
            selector.select(Some(OsStr::new(""))),
            Err(SelectionError::Empty)
        );
        for value in [
            "missing",
            "native-child-process",
            "ohos-first-sys,ohos-last-sys",
            "ohos-first-sys ohos-last-sys",
            " ohos-first-sys",
            "ohos-first-sys ",
            "*",
        ] {
            assert_eq!(
                selector.select(Some(OsStr::new(value))),
                Err(SelectionError::Unknown(value.to_owned()))
            );
        }
    }

    #[test]
    fn rejects_ambiguous_registry_names() {
        let names = &["duplicate", "duplicate"];
        assert_eq!(
            ConfigSelector::new(names).select(Some(OsStr::new("duplicate"))),
            Err(SelectionError::Duplicate("duplicate".to_owned()))
        );
    }

    #[cfg(unix)]
    #[test]
    fn rejects_non_utf8_without_lossy_identity() {
        use std::os::unix::ffi::OsStrExt;
        assert_eq!(
            ConfigSelector::new(NAMES).select(Some(OsStr::from_bytes(b"\xff"))),
            Err(SelectionError::NonUtf8)
        );
    }
}
