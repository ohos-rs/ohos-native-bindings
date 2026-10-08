use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use regex::Regex;
use syn::parse::Parser;

use super::BASELINE_API_VERSION;

/// Adds `#[cfg(feature = "api-XX")]` attributes based on `@since XX` annotations in doc comments.
/// Only adds feature gates for API versions > BASELINE_API_VERSION.
/// Returns the processed content and a set of API versions found (> baseline).
pub(crate) fn add_feature_gates(
    content: &str,
    header_since: &BTreeMap<String, u32>,
    global_symbol_usage_min: Option<&HashMap<String, u32>>,
) -> (String, BTreeSet<u32>, HashMap<String, u32>) {
    let fn_re = Regex::new(r"^pub fn\s+([A-Za-z_]\w*)\s*\(").unwrap();
    let const_re = Regex::new(r"^pub const\s+([A-Za-z_]\w*)\b").unwrap();
    let const_with_type_re =
        Regex::new(r"^pub const\s+[A-Za-z_]\w*\s*:\s*([A-Za-z_]\w*)\s*=").unwrap();
    let const_start_re = Regex::new(r"^pub const\s+[A-Za-z_]\w*\s*:\s*$").unwrap();
    let const_type_line_re = Regex::new(r"^\s*([A-Za-z_]\w*)\s*=").unwrap();
    let type_re = Regex::new(r"^pub type\s+([A-Za-z_]\w*)\b").unwrap();
    let enum_re = Regex::new(r"^pub enum\s+([A-Za-z_]\w*)\b").unwrap();
    // Also matches `pub union`: bindgen emits anonymous unions (`*__bindgen_ty_N`) with no
    // doc comment, so their `@since` has to be inferred from the member types they reference.
    let struct_re = Regex::new(r"^pub (?:struct|union)\s+([A-Za-z_]\w*)\b").unwrap();
    let field_re = Regex::new(r"^pub\s+([A-Za-z_]\w*)\s*:").unwrap();
    let static_re = Regex::new(r"^pub static (?:mut )?([A-Za-z_]\w*)\s*:").unwrap();
    let self_alias_re = Regex::new(r"^pub use self::([A-Za-z_]\w*) as ([A-Za-z_]\w*);").unwrap();
    // `impl X {` and `impl Trait for X {` both anchor on the implementing type X.
    let impl_re = Regex::new(r"^impl\s+(?:.+\s+for\s+)?(?P<ty>[A-Za-z_]\w*)\b").unwrap();
    let ident_re = Regex::new(r"\b([A-Za-z_]\w*)\b").unwrap();

    let declarations = DeclarationPatterns {
        fn_re: &fn_re,
        const_re: &const_re,
        type_re: &type_re,
        enum_re: &enum_re,
        struct_re: &struct_re,
        field_re: &field_re,
        static_re: &static_re,
        self_alias_re: &self_alias_re,
        impl_re: &impl_re,
    };

    // bindgen 0.65 drops comments on forward-declared opaque records. Recover
    // their header annotations before falling back to documented uses.
    let opaque_re =
        Regex::new(r"pub struct ([A-Za-z_]\w*)\s*\{\s*_unused:\s*\[u8;\s*0\],?\s*\}").unwrap();
    let opaque_names: HashSet<_> = opaque_re
        .captures_iter(content)
        .map(|capture| capture[1].to_owned())
        .collect();

    let lines: Vec<&str> = content.lines().collect();
    // `pub use self::X as Y;` aliases inherit X's gate; see below.
    let self_aliases: HashMap<String, String> = lines
        .iter()
        .filter_map(|l| self_alias_re.captures(l.trim()))
        .map(|c| (c[2].to_string(), c[1].to_string()))
        .collect();
    let mut min_since_by_key: HashMap<String, u32> = opaque_names
        .iter()
        .filter_map(|name| {
            header_since
                .get(name)
                .map(|since| (format!("struct:{name}"), *since))
        })
        .collect();
    let mut declaration_infos = Vec::new();

    // Pass 1: collect minimal @since for each symbol key across the whole generated file.
    let mut attrs = Vec::new();
    let mut composite = CompositeState::default();
    let mut pending_info: Option<PendingDeclInfo> = None;
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("#[") {
            attrs.push(trimmed);
            continue;
        }

        if let Some(mut pending) = pending_info.take() {
            pending.lines.push((*line).to_string());
            if pending.kind.is_complete(&pending.lines.join("\n")) {
                let joined = pending.lines.join("\n");
                declaration_infos.push(DeclarationInfo {
                    key: pending.key,
                    local_since: pending.local_since,
                    text: joined,
                });
            } else {
                pending_info = Some(pending);
            }
        }

        let item_since = parse_min_since(&attrs);
        let key = declarations.key(trimmed, &composite.current_name);
        if let Some(key) = key {
            declaration_infos.push(DeclarationInfo {
                key: key.clone(),
                local_since: item_since,
                text: (*line).to_string(),
            });

            if let Some(version) = item_since {
                let entry = min_since_by_key.entry(key.clone()).or_insert(version);
                if version < *entry {
                    *entry = version;
                }
            } else if key
                .strip_prefix("type:")
                .is_some_and(|n| self_aliases.contains_key(n))
            {
                // Handled by `inherit_self_alias_since` after this pass.
            } else if is_anonymous_bindgen_key(&key) {
                // Anonymous composites (`Parent__bindgen_ty_N`) carry no doc comment of their
                // own. They exist only as members of their parent, so they inherit its gate
                // rather than falling back to the baseline - which would otherwise drag the
                // member types they reference down to the baseline as well.
            } else if key
                .strip_prefix("struct:")
                .is_some_and(|name| opaque_names.contains(name))
            {
                // `relax_min_since_by_references` inherits documented callers.
            } else if key.starts_with("type:")
                || key.starts_with("struct:")
                || key.starts_with("enum:")
            {
                let version = BASELINE_API_VERSION;
                let entry = min_since_by_key.entry(key.clone()).or_insert(version);
                if version < *entry {
                    *entry = version;
                }
            }

            if let Some(kind) = DeclarationKind::for_key(&key) {
                if !kind.is_complete(trimmed) {
                    declaration_infos.pop();
                    pending_info = Some(PendingDeclInfo {
                        key,
                        local_since: item_since,
                        lines: vec![(*line).to_string()],
                        kind,
                    });
                }
            }
        }

        attrs.clear();
        composite.update(trimmed, &struct_re, &enum_re);
    }
    if let Some(pending) = pending_info.take() {
        declaration_infos.push(DeclarationInfo {
            key: pending.key,
            local_since: pending.local_since,
            text: pending.lines.join("\n"),
        });
    }

    inherit_anonymous_composite_since(&declaration_infos, &mut min_since_by_key);
    inherit_self_alias_since(&self_aliases, &mut min_since_by_key);
    inherit_impl_since(&declaration_infos, &mut min_since_by_key);

    relax_min_since_by_references(
        &declaration_infos,
        &mut min_since_by_key,
        &ident_re,
        &["type:", "enum:", "struct:"],
    );
    if let Some(global_usage_min) = global_symbol_usage_min {
        apply_global_symbol_usage_min_to_symbols(&mut min_since_by_key, global_usage_min);
        relax_min_since_by_references(
            &declaration_infos,
            &mut min_since_by_key,
            &ident_re,
            &["type:", "enum:", "struct:"],
        );
    }
    let symbol_usage_min =
        collect_symbol_usage_min_since(&declaration_infos, &min_since_by_key, &ident_re);

    // Pass 2: apply cfg using each symbol's minimal @since.
    let symbol_since = build_symbol_since_map(&min_since_by_key);
    let mut result = Vec::with_capacity(lines.len());
    let mut api_versions = BTreeSet::new();
    let mut attrs = Vec::new();
    let mut composite = CompositeState::default();
    let mut pending_const: Option<PendingConst> = None;
    let mut pending_decl: Option<PendingDecl> = None;
    for line in &lines {
        let trimmed = line.trim();

        if let Some(mut pending) = pending_decl.take() {
            pending.dep_since = merge_doc_and_dep_since(
                pending.dep_since,
                infer_since_from_references(trimmed, &symbol_since, &ident_re),
            );
            pending.lines.push((*line).to_string());
            if pending.kind.is_complete(&pending.lines.join("\n")) {
                emit_pending_decl(&mut result, &mut api_versions, pending);
            } else {
                pending_decl = Some(pending);
            }
            composite.update(trimmed, &struct_re, &enum_re);
            continue;
        }

        if let Some(pending) = pending_const.take() {
            if let Some(cap) = const_type_line_re.captures(trimmed) {
                let inferred = infer_type_since(&cap[1], &min_since_by_key);
                emit_with_cfg(
                    &mut result,
                    &mut api_versions,
                    pending.attrs,
                    pending.line,
                    &pending.indent,
                    pending.target_since.or(inferred),
                );
                result.push((*line).to_string());
                composite.update(trimmed, &struct_re, &enum_re);
                continue;
            }

            emit_with_cfg(
                &mut result,
                &mut api_versions,
                pending.attrs,
                pending.line,
                &pending.indent,
                pending.target_since,
            );
        }

        if trimmed.starts_with("#[") {
            attrs.push(line.to_string());
            continue;
        }

        let item_key = declarations.key(trimmed, &composite.current_name);
        let local_since = parse_min_since_owned(&attrs);
        let mut grouped_since = item_key
            .as_ref()
            .and_then(|key| min_since_by_key.get(key))
            .copied();
        if local_since.is_none()
            && item_key
                .as_deref()
                .is_some_and(|key| key.starts_with("fn:"))
        {
            grouped_since = None;
        }
        if let Some(key) = item_key.as_deref() {
            if let Some(name) = key
                .strip_prefix("type:")
                .or_else(|| key.strip_prefix("enum:"))
                .or_else(|| key.strip_prefix("struct:"))
            {
                let local_usage_min = symbol_usage_min.get(name).copied();
                let cross_file_usage_min =
                    global_symbol_usage_min.and_then(|map| map.get(name).copied());
                let usage_min = match (local_usage_min, cross_file_usage_min) {
                    (Some(a), Some(b)) => Some(a.min(b)),
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (None, None) => None,
                };
                if let Some(usage_min) = usage_min {
                    grouped_since = Some(match grouped_since {
                        Some(existing) => existing.min(usage_min),
                        None => usage_min,
                    });
                }
            }
        }
        let since_from_docs = match (local_since, grouped_since) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };
        let dep_since = infer_since_from_references(trimmed, &symbol_since, &ident_re);
        let target_since = merge_doc_and_dep_since(since_from_docs, dep_since);

        if let Some(key) = item_key.as_deref() {
            if !key.starts_with("const:") {
                if let Some(kind) = DeclarationKind::for_key(key) {
                    if !kind.is_complete(trimmed) && !trimmed.ends_with('{') {
                        let indent = line.len() - trimmed.len();
                        let indent_str = line[..indent].to_string();
                        pending_decl = Some(PendingDecl {
                            attrs: std::mem::take(&mut attrs),
                            lines: vec![(*line).to_string()],
                            indent: indent_str,
                            doc_since: since_from_docs,
                            dep_since,
                            kind,
                        });
                        composite.update(trimmed, &struct_re, &enum_re);
                        continue;
                    }
                }

                if let Some(version) = target_since {
                    if version > BASELINE_API_VERSION && !has_api_cfg(&attrs) {
                        let indent = line.len() - trimmed.len();
                        let indent_str = &line[..indent];
                        insert_cfg_after_doc_attrs(&mut attrs, indent_str, version);
                        api_versions.insert(version);
                    }
                }
            }
        }

        if trimmed.starts_with("pub const ") {
            let indent = line.len() - trimmed.len();
            let indent_str = line[..indent].to_string();

            if const_start_re.is_match(trimmed) {
                pending_const = Some(PendingConst {
                    attrs: std::mem::take(&mut attrs),
                    line: (*line).to_string(),
                    indent: indent_str,
                    target_since,
                });
                composite.update(trimmed, &struct_re, &enum_re);
                continue;
            }

            if let Some(cap) = const_with_type_re.captures(trimmed) {
                let inferred = infer_type_since(&cap[1], &min_since_by_key);
                let final_since = merge_doc_and_dep_since(target_since, inferred);
                let const_attrs = std::mem::take(&mut attrs);
                emit_with_cfg(
                    &mut result,
                    &mut api_versions,
                    const_attrs,
                    (*line).to_string(),
                    &indent_str,
                    final_since,
                );
                composite.update(trimmed, &struct_re, &enum_re);
                continue;
            }
        }

        result.append(&mut attrs);
        result.push((*line).to_string());
        composite.update(trimmed, &struct_re, &enum_re);
    }

    if let Some(pending) = pending_const.take() {
        emit_with_cfg(
            &mut result,
            &mut api_versions,
            pending.attrs,
            pending.line,
            &pending.indent,
            pending.target_since,
        );
    }

    if let Some(pending) = pending_decl.take() {
        emit_pending_decl(&mut result, &mut api_versions, pending);
    }

    if !attrs.is_empty() {
        result.extend(attrs);
    }

    let normalized = normalize_cfg_lines(result);
    let normalized = insert_stable_aliases_for_bindgen_types(normalized);
    (normalized.join("\n"), api_versions, symbol_usage_min)
}

struct PendingConst {
    attrs: Vec<String>,
    line: String,
    indent: String,
    target_since: Option<u32>,
}

struct DeclarationInfo {
    key: String,
    local_since: Option<u32>,
    text: String,
}

struct PendingDeclInfo {
    key: String,
    local_since: Option<u32>,
    lines: Vec<String>,
    kind: DeclarationKind,
}

struct PendingDecl {
    attrs: Vec<String>,
    lines: Vec<String>,
    indent: String,
    doc_since: Option<u32>,
    dep_since: Option<u32>,
    kind: DeclarationKind,
}

#[derive(Default)]
struct CompositeState {
    current_name: Option<String>,
    brace_depth: i32,
}

impl CompositeState {
    fn update(&mut self, trimmed: &str, struct_re: &Regex, enum_re: &Regex) {
        if let Some(name) = composite_name(trimmed, struct_re, enum_re) {
            let delta = brace_delta(trimmed);
            if delta > 0 {
                self.current_name = Some(name);
                self.brace_depth = delta;
            } else {
                self.current_name = None;
                self.brace_depth = 0;
            }
            return;
        }

        if self.current_name.is_some() {
            self.brace_depth += brace_delta(trimmed);
            if self.brace_depth <= 0 {
                self.current_name = None;
                self.brace_depth = 0;
            }
        }
    }
}

fn parse_min_since(attrs: &[&str]) -> Option<u32> {
    attrs
        .iter()
        .filter_map(|line| parse_since_value(line))
        .min()
}

fn parse_min_since_owned(attrs: &[String]) -> Option<u32> {
    attrs
        .iter()
        .filter_map(|line| parse_since_value(line))
        .min()
}

fn parse_since_value(line: &str) -> Option<u32> {
    let pos = line.find("@since")?;
    let mut chars = line[pos + "@since".len()..].chars().peekable();
    while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
        chars.next();
    }
    let mut digits = String::new();
    while matches!(chars.peek(), Some(c) if c.is_ascii_digit()) {
        digits.push(chars.next().unwrap());
    }
    if digits.is_empty() {
        None
    } else {
        digits.parse::<u32>().ok()
    }
}

fn has_api_cfg(attrs: &[String]) -> bool {
    attrs
        .iter()
        .any(|line| line.contains("#[cfg(") && line.contains("feature = \"api-"))
}

fn infer_type_since(type_name: &str, min_since_by_key: &HashMap<String, u32>) -> Option<u32> {
    let type_key = format!("type:{type_name}");
    let enum_key = format!("enum:{type_name}");
    match (
        min_since_by_key.get(&type_key).copied(),
        min_since_by_key.get(&enum_key).copied(),
    ) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn emit_with_cfg(
    result: &mut Vec<String>,
    api_versions: &mut BTreeSet<u32>,
    mut attrs: Vec<String>,
    line: String,
    indent: &str,
    since: Option<u32>,
) {
    if let Some(version) = since {
        if version > BASELINE_API_VERSION && !has_api_cfg(&attrs) {
            insert_cfg_after_doc_attrs(&mut attrs, indent, version);
            api_versions.insert(version);
        }
    }
    result.extend(attrs);
    result.push(line);
}

fn emit_pending_decl(
    result: &mut Vec<String>,
    api_versions: &mut BTreeSet<u32>,
    mut pending: PendingDecl,
) {
    let since = merge_doc_and_dep_since(pending.doc_since, pending.dep_since);
    if let Some(version) = since {
        if version > BASELINE_API_VERSION && !has_api_cfg(&pending.attrs) {
            insert_cfg_after_doc_attrs(&mut pending.attrs, &pending.indent, version);
            api_versions.insert(version);
        }
    }
    result.extend(pending.attrs);
    result.extend(pending.lines);
}

#[derive(Clone, Copy)]
enum DeclarationKind {
    Function,
    Type,
    Field,
}

impl DeclarationKind {
    fn for_key(key: &str) -> Option<Self> {
        if key.starts_with("fn:") {
            Some(Self::Function)
        } else if key.starts_with("type:") {
            Some(Self::Type)
        } else if key.starts_with("field:") {
            Some(Self::Field)
        } else {
            None
        }
    }

    fn is_complete(self, declaration: &str) -> bool {
        match self {
            Self::Function => syn::parse_str::<syn::ForeignItemFn>(declaration).is_ok(),
            // Self aliases are also recorded under a type key.
            Self::Type => {
                syn::parse_str::<syn::ItemType>(declaration).is_ok()
                    || syn::parse_str::<syn::ItemUse>(declaration).is_ok()
            }
            Self::Field => {
                // A parameter comma inside a function pointer does not finish a field.
                let parser = |input: syn::parse::ParseStream<'_>| -> syn::Result<()> {
                    input.call(syn::Field::parse_named)?;
                    if !input.is_empty() {
                        input.parse::<syn::Token![,]>()?;
                    }
                    Ok(())
                };
                parser.parse_str(declaration).is_ok()
            }
        }
    }
}

fn insert_cfg_after_doc_attrs(attrs: &mut Vec<String>, indent: &str, version: u32) {
    let cfg_line = format!("{indent}#[cfg(feature = \"api-{version}\")]");
    let insert_at = attrs
        .iter()
        .take_while(|line| line.trim_start().starts_with("#[doc ="))
        .count();
    attrs.insert(insert_at, cfg_line);
}

/// Whether `key` names an anonymous composite generated by bindgen (`Parent__bindgen_ty_N`).
fn is_anonymous_bindgen_key(key: &str) -> bool {
    key.split_once(':')
        .is_some_and(|(_, name)| name.contains("__bindgen_ty_"))
}

/// Gives anonymous composites the `@since` of the parent they are named after.
///
/// bindgen names them `Parent__bindgen_ty_N` and emits no doc comment, so they have no
/// `@since` of their own. Leaving them ungated emits a type that references gated members,
/// which fails to compile below the members' API level.
fn inherit_anonymous_composite_since(
    declarations: &[DeclarationInfo],
    min_since_by_key: &mut HashMap<String, u32>,
) {
    for decl in declarations {
        if !is_anonymous_bindgen_key(&decl.key) || decl.local_since.is_some() {
            continue;
        }
        let Some((kind, name)) = decl.key.split_once(':') else {
            continue;
        };
        let Some((parent, _)) = name.split_once("__bindgen_ty_") else {
            continue;
        };
        // The parent may be declared as either a struct or a union; both use the `struct:` key.
        if let Some(parent_since) = min_since_by_key.get(&format!("struct:{parent}")).copied() {
            min_since_by_key.insert(decl.key.clone(), parent_since);
        } else if let Some(parent_since) =
            min_since_by_key.get(&format!("{kind}:{parent}")).copied()
        {
            min_since_by_key.insert(decl.key.clone(), parent_since);
        }
    }
}

/// Gives `pub use self::X as Y;` aliases the `@since` of the type they alias.
/// Looks a symbol name up under every key kind a type can be declared with.
fn since_of_type_name(name: &str, min_since_by_key: &HashMap<String, u32>) -> Option<u32> {
    ["type:", "struct:", "enum:"]
        .iter()
        .find_map(|k| min_since_by_key.get(&format!("{k}{name}")).copied())
}

/// Gives `pub use self::X as Y;` aliases the `@since` of the type they alias.
///
/// Iterates to a fixed point so a chain of aliases resolves, and accepts a target declared
/// as a struct or enum rather than a plain type alias.
fn inherit_self_alias_since(
    self_aliases: &HashMap<String, String>,
    min_since_by_key: &mut HashMap<String, u32>,
) {
    for _ in 0..8 {
        let mut changed = false;
        for (alias, target) in self_aliases {
            let Some(since) = since_of_type_name(target, min_since_by_key) else {
                continue;
            };
            if min_since_by_key.insert(format!("type:{alias}"), since) != Some(since) {
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}

/// Gives `impl` blocks the `@since` of the type they implement.
fn inherit_impl_since(
    declarations: &[DeclarationInfo],
    min_since_by_key: &mut HashMap<String, u32>,
) {
    for decl in declarations {
        let Some(name) = decl.key.strip_prefix("impl:") else {
            continue;
        };
        if let Some(since) = since_of_type_name(name, min_since_by_key) {
            min_since_by_key.insert(decl.key.clone(), since);
        }
    }
}

fn relax_min_since_by_references(
    declarations: &[DeclarationInfo],
    min_since_by_key: &mut HashMap<String, u32>,
    ident_re: &Regex,
    symbol_prefixes: &[&str],
) {
    let mut symbol_to_keys: HashMap<String, Vec<String>> = HashMap::new();
    for decl in declarations {
        for prefix in symbol_prefixes {
            if let Some(name) = decl.key.strip_prefix(prefix) {
                symbol_to_keys
                    .entry(name.to_string())
                    .or_default()
                    .push(decl.key.clone());
            }
        }
    }

    // Fixed-point relaxation:
    // if an item appears since S, referenced symbols must exist no later than S.
    for _ in 0..8 {
        let mut changed = false;
        for decl in declarations {
            let source_since =
                declaration_effective_since(&decl.key, decl.local_since, min_since_by_key);
            let Some(source_since) = source_since else {
                continue;
            };

            for cap in ident_re.captures_iter(&decl.text) {
                let ident = &cap[1];
                let Some(target_keys) = symbol_to_keys.get(ident) else {
                    continue;
                };
                for target_key in target_keys {
                    let entry = min_since_by_key
                        .entry(target_key.clone())
                        .or_insert(source_since);
                    if source_since < *entry {
                        *entry = source_since;
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
}

fn declaration_effective_since(
    key: &str,
    local_since: Option<u32>,
    min_since_by_key: &HashMap<String, u32>,
) -> Option<u32> {
    if let Some(since) = min_since_by_key.get(key).copied().or(local_since) {
        return Some(since);
    }

    if let Some(rest) = key.strip_prefix("field:") {
        if let Some((container, _field)) = rest.split_once("::") {
            let struct_key = format!("struct:{container}");
            let enum_key = format!("enum:{container}");
            return match (
                min_since_by_key.get(&struct_key).copied(),
                min_since_by_key.get(&enum_key).copied(),
            ) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            };
        }
    }

    None
}

fn collect_symbol_usage_min_since(
    declarations: &[DeclarationInfo],
    min_since_by_key: &HashMap<String, u32>,
    ident_re: &Regex,
) -> HashMap<String, u32> {
    let mut usage_min: HashMap<String, u32> = HashMap::new();
    for decl in declarations {
        let Some(source_since) =
            declaration_effective_since(&decl.key, decl.local_since, min_since_by_key)
        else {
            continue;
        };
        for cap in ident_re.captures_iter(&decl.text) {
            let ident = cap[1].to_string();
            match usage_min.get_mut(&ident) {
                Some(existing) => {
                    if source_since < *existing {
                        *existing = source_since;
                    }
                }
                None => {
                    usage_min.insert(ident, source_since);
                }
            }
        }
    }
    usage_min
}

fn apply_global_symbol_usage_min_to_symbols(
    min_since_by_key: &mut HashMap<String, u32>,
    global_symbol_usage_min: &HashMap<String, u32>,
) {
    for (symbol, usage_min) in global_symbol_usage_min {
        for prefix in ["type:", "enum:", "struct:"] {
            let key = format!("{prefix}{symbol}");
            if min_since_by_key.contains_key(&key) {
                upsert_min(min_since_by_key, &key, *usage_min);
            }
        }
    }
}

pub(crate) fn merge_symbol_usage_min(
    target: &mut HashMap<String, u32>,
    source: &HashMap<String, u32>,
) {
    for (symbol, usage_min) in source {
        upsert_min(target, symbol, *usage_min);
    }
}

fn insert_stable_aliases_for_bindgen_types(lines: Vec<String>) -> Vec<String> {
    let bindgen_alias_re = Regex::new(r"^pub type (_bindgen_ty_\d+)\s*=\s*.+;$").unwrap();
    let const_bindgen_alias_re =
        Regex::new(r"^pub const ([A-Za-z_]\w*)\s*:\s*(_bindgen_ty_\d+)\s*=").unwrap();
    let type_decl_re = Regex::new(r"^pub type ([A-Za-z_]\w*)\b").unwrap();

    let mut existing_type_names = HashSet::new();
    let mut alias_to_const_names: HashMap<String, Vec<String>> = HashMap::new();
    for line in &lines {
        let trimmed = line.trim();
        if let Some(cap) = type_decl_re.captures(trimmed) {
            existing_type_names.insert(cap[1].to_string());
        }
        if let Some(cap) = const_bindgen_alias_re.captures(trimmed) {
            alias_to_const_names
                .entry(cap[2].to_string())
                .or_default()
                .push(cap[1].to_string());
        }
    }

    let mut alias_to_stable_name = HashMap::new();
    for (alias, const_names) in alias_to_const_names {
        let Some(common_prefix) = longest_common_prefix(&const_names) else {
            continue;
        };
        let Some(pos) = common_prefix.rfind('_') else {
            continue;
        };
        let base = common_prefix[..pos].trim_end_matches('_');
        if base.is_empty() {
            continue;
        }
        let stable_name = choose_unique_type_alias_name(base, &mut existing_type_names);
        alias_to_stable_name.insert(alias, stable_name);
    }
    let mut result = Vec::with_capacity(lines.len() + alias_to_stable_name.len() * 2);
    let mut pending_attrs: Vec<String> = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("#[") {
            pending_attrs.push(line);
            continue;
        }

        let cfg_attrs: Vec<String> = pending_attrs
            .iter()
            .filter(|attr| attr.trim_start().starts_with("#[cfg"))
            .cloned()
            .collect();

        result.append(&mut pending_attrs);
        result.push(line.clone());

        if let Some(cap) = bindgen_alias_re.captures(trimmed) {
            let alias = &cap[1];
            if let Some(stable_name) = alias_to_stable_name.get(alias) {
                let indent = line.len() - trimmed.len();
                let indent_str = &line[..indent];
                result.extend(cfg_attrs);
                result.push(format!("{indent_str}pub type {stable_name} = {alias};"));
            }
        }
    }

    if !pending_attrs.is_empty() {
        result.extend(pending_attrs);
    }

    result
}

fn longest_common_prefix(values: &[String]) -> Option<String> {
    let first = values.first()?.clone();
    let mut prefix = first;
    for value in values.iter().skip(1) {
        let bytes_a = prefix.as_bytes();
        let bytes_b = value.as_bytes();
        let mut idx = 0;
        while idx < bytes_a.len() && idx < bytes_b.len() && bytes_a[idx] == bytes_b[idx] {
            idx += 1;
        }
        prefix.truncate(idx);
        if prefix.is_empty() {
            break;
        }
    }
    if prefix.is_empty() {
        None
    } else {
        Some(prefix)
    }
}

fn choose_unique_type_alias_name(base: &str, existing_type_names: &mut HashSet<String>) -> String {
    let mut candidate = base.to_string();
    if existing_type_names.insert(candidate.clone()) {
        return candidate;
    }

    let mut index = 1;
    loop {
        candidate = format!("{base}_TYPE_{index}");
        if existing_type_names.insert(candidate.clone()) {
            return candidate;
        }
        index += 1;
    }
}

fn normalize_cfg_lines(lines: Vec<String>) -> Vec<String> {
    let cfg_api_re = Regex::new(r#"^\s*#\[cfg\(feature = "api-(\d+)"\)\]\s*$"#).unwrap();
    let mut deduped = Vec::with_capacity(lines.len());
    for line in lines {
        let is_api_cfg = cfg_api_re.is_match(line.trim());
        if is_api_cfg
            && deduped
                .last()
                .is_some_and(|prev: &String| prev.trim() == line.trim())
        {
            continue;
        }
        deduped.push(line);
    }

    let mut pruned = Vec::with_capacity(deduped.len());
    let mut scope_cfg_stack: Vec<Option<u32>> = vec![None];
    let mut pending_cfg: Option<u32> = None;
    for line in deduped {
        let trimmed = line.trim();

        if let Some(cap) = cfg_api_re.captures(trimmed) {
            let version = cap
                .get(1)
                .and_then(|m| m.as_str().parse::<u32>().ok())
                .unwrap_or(BASELINE_API_VERSION);
            let active = scope_cfg_stack.last().copied().flatten();
            if active.is_some_and(|outer| version <= outer) {
                continue;
            }
            pending_cfg = Some(match pending_cfg {
                Some(existing) => existing.max(version),
                None => version,
            });
            pruned.push(line);
            continue;
        }

        if trimmed.starts_with("#[") {
            pruned.push(line);
            continue;
        }

        let outer = scope_cfg_stack.last().copied().flatten();
        let effective = match (outer, pending_cfg) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };
        pending_cfg = None;

        pruned.push(line.clone());

        let opens = trimmed.chars().filter(|c| *c == '{').count();
        for _ in 0..opens {
            scope_cfg_stack.push(effective);
        }

        let closes = trimmed.chars().filter(|c| *c == '}').count();
        for _ in 0..closes {
            if scope_cfg_stack.len() > 1 {
                scope_cfg_stack.pop();
            }
        }
    }

    pruned
}

struct DeclarationPatterns<'a> {
    fn_re: &'a Regex,
    const_re: &'a Regex,
    type_re: &'a Regex,
    enum_re: &'a Regex,
    struct_re: &'a Regex,
    field_re: &'a Regex,
    static_re: &'a Regex,
    self_alias_re: &'a Regex,
    impl_re: &'a Regex,
}

impl DeclarationPatterns<'_> {
    fn key(&self, trimmed: &str, current_composite: &Option<String>) -> Option<String> {
        if let Some(cap) = self.fn_re.captures(trimmed) {
            return Some(format!("fn:{}", &cap[1]));
        }
        // Statics inside `extern "C"` blocks carry their own `@since`, and gating them matters:
        // they are typed by structs that may themselves be gated.
        if let Some(cap) = self.static_re.captures(trimmed) {
            return Some(format!("static:{}", &cap[1]));
        }
        // bindgen renders `typedef enum X Y;` as `pub use self::X as Y;`. The alias inherits X's
        // `@since`; without this it stays ungated and fails to resolve below X's API level.
        if let Some(cap) = self.self_alias_re.captures(trimmed) {
            return Some(format!("type:{}", &cap[2]));
        }
        // bindgen emits inherent/trait impls for generated types. A gated type with an ungated
        // impl block does not resolve below the type's API level.
        if let Some(cap) = self.impl_re.captures(trimmed) {
            return Some(format!(
                "impl:{}",
                cap.name("ty").map_or("", |m| m.as_str())
            ));
        }
        if let Some(cap) = self.const_re.captures(trimmed) {
            return Some(format!("const:{}", &cap[1]));
        }
        if let Some(cap) = self.type_re.captures(trimmed) {
            return Some(format!("type:{}", &cap[1]));
        }
        if let Some(cap) = self.enum_re.captures(trimmed) {
            return Some(format!("enum:{}", &cap[1]));
        }
        if let Some(cap) = self.struct_re.captures(trimmed) {
            return Some(format!("struct:{}", &cap[1]));
        }
        if let Some(cap) = self.field_re.captures(trimmed) {
            if let Some(container) = current_composite.as_ref() {
                return Some(format!("field:{container}::{}", &cap[1]));
            }
        }
        None
    }
}

fn build_symbol_since_map(min_since_by_key: &HashMap<String, u32>) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for (key, version) in min_since_by_key {
        if let Some(name) = key.strip_prefix("type:") {
            upsert_min(&mut map, name, *version);
        } else if let Some(name) = key.strip_prefix("enum:") {
            upsert_min(&mut map, name, *version);
        } else if let Some(name) = key.strip_prefix("struct:") {
            upsert_min(&mut map, name, *version);
        }
    }
    map
}

fn upsert_min(map: &mut HashMap<String, u32>, key: &str, value: u32) {
    match map.get_mut(key) {
        Some(existing) => {
            if value < *existing {
                *existing = value;
            }
        }
        None => {
            map.insert(key.to_string(), value);
        }
    }
}

fn infer_since_from_references(
    line: &str,
    symbol_since: &HashMap<String, u32>,
    ident_re: &Regex,
) -> Option<u32> {
    ident_re
        .captures_iter(line)
        .filter_map(|cap| symbol_since.get(&cap[1]).copied())
        .filter(|version| *version > BASELINE_API_VERSION)
        .max()
}

fn merge_doc_and_dep_since(doc_since: Option<u32>, dep_since: Option<u32>) -> Option<u32> {
    match (doc_since, dep_since) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn composite_name(trimmed: &str, struct_re: &Regex, enum_re: &Regex) -> Option<String> {
    if let Some(cap) = struct_re.captures(trimmed) {
        return Some(cap[1].to_string());
    }
    if let Some(cap) = enum_re.captures(trimmed) {
        return Some(cap[1].to_string());
    }
    None
}

fn brace_delta(trimmed: &str) -> i32 {
    // Generated doc lines are in attributes, so braces here are syntax braces we need for scope tracking.
    let opens = trimmed.chars().filter(|c| *c == '{').count() as i32;
    let closes = trimmed.chars().filter(|c| *c == '}').count() as i32;
    opens - closes
}
