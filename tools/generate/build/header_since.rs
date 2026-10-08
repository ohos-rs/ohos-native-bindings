use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Context;
use regex::Regex;

/// Records dependencies without doing IO inside bindgen's parse callbacks.
#[derive(Clone, Debug, Default)]
pub(crate) struct IncludedHeaders(Rc<RefCell<BTreeSet<PathBuf>>>);

impl IncludedHeaders {
    pub(crate) fn paths(&self) -> Vec<PathBuf> {
        self.0.borrow().iter().cloned().collect()
    }
}

impl bindgen::callbacks::ParseCallbacks for IncludedHeaders {
    fn include_file(&self, filename: &str) {
        self.0.borrow_mut().insert(PathBuf::from(filename));
    }
}

/// Caches forward-declaration annotations once per header across all sys crates.
pub(crate) struct HeaderSinceInventory {
    headers: BTreeMap<PathBuf, BTreeMap<String, u32>>,
    comments: Regex,
    declaration: Regex,
    since: Regex,
}

impl Default for HeaderSinceInventory {
    fn default() -> Self {
        Self {
            headers: BTreeMap::new(),
            comments: Regex::new(r"(?s)/\*.*?\*/|//[^\r\n]*").unwrap(),
            declaration: Regex::new(
                r"^\s*(?:typedef\s+)?(?:struct|union)\s+([A-Za-z_]\w*)\s*(?:\*\s*)*(?:([A-Za-z_]\w*)\s*)?;",
            )
            .unwrap(),
            since: Regex::new(r"[@\\]since\s+(\d+)").unwrap(),
        }
    }
}

impl HeaderSinceInventory {
    pub(crate) fn for_headers(
        &mut self,
        paths: Vec<PathBuf>,
    ) -> anyhow::Result<BTreeMap<String, u32>> {
        let mut result = BTreeMap::new();
        for path in paths {
            let path = path
                .canonicalize()
                .with_context(|| format!("Cannot resolve included header {}", path.display()))?;
            if !self.headers.contains_key(&path) {
                let source = fs::read_to_string(&path)
                    .with_context(|| format!("Cannot read included header {}", path.display()))?;
                let annotations = self.parse(&source);
                self.headers.insert(path.clone(), annotations);
                println!("cargo:rerun-if-changed={}", path.display());
            }
            for (name, since) in &self.headers[&path] {
                result
                    .entry(name.clone())
                    .and_modify(|current: &mut u32| *current = (*current).min(*since))
                    .or_insert(*since);
            }
        }
        Ok(result)
    }

    fn parse(&self, source: &str) -> BTreeMap<String, u32> {
        let mut result = BTreeMap::new();
        for comment in self.comments.find_iter(source) {
            if !comment.as_str().starts_with("/**") && !comment.as_str().starts_with("/*!") {
                continue;
            }
            // Only an immediately following forward declaration owns this comment.
            // Definitions and intervening declarations must not donate their @since.
            let Some(declaration) = self.declaration.captures(&source[comment.end()..]) else {
                continue;
            };
            let Some(since) = self
                .since
                .captures_iter(comment.as_str())
                .filter_map(|capture| capture[1].parse::<u32>().ok())
                .min()
            else {
                continue;
            };
            // Bindgen can retain either the record tag or its typedef name.
            for name in declaration.iter().skip(1).flatten() {
                result
                    .entry(name.as_str().to_owned())
                    .and_modify(|current: &mut u32| *current = (*current).min(since))
                    .or_insert(since);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::HeaderSinceInventory;

    #[test]
    fn reads_forward_records_and_pointer_aliases_with_earliest_since() {
        let source = r"
/** @since 26.0.0 */
typedef struct Record Record;
/** @since 24 */
typedef struct Record Record;
/*! @since 18 */
typedef struct Data__
    *Data;
/** @since 20 */
union Storage;
";
        let since = HeaderSinceInventory::default().parse(source);
        assert_eq!(since["Record"], 24);
        assert_eq!(since["Data__"], 18);
        assert_eq!(since["Data"], 18);
        assert_eq!(since["Storage"], 20);
    }

    #[test]
    fn does_not_inherit_annotations_from_other_declarations_or_comments() {
        let source = r"
/** @since 26 */
void unrelated(void);
typedef struct Undocumented Undocumented;
/** @since 25 */
typedef struct Defined { int value; } Defined;
typedef struct Another Another;
/** @since 24 */
/** No version for this declaration. */
typedef struct NoVersion NoVersion;
// /** @since 23 */ typedef struct Commented Commented;
";
        assert!(HeaderSinceInventory::default().parse(source).is_empty());
    }
}
