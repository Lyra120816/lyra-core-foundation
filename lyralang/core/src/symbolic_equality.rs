use std::collections::{BTreeMap, BTreeSet};

use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolicEqualityRuleDescriptor {
    pub id: &'static str,
    pub domain: &'static str,
    pub relation: &'static str,
    pub law: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolicEquivalenceClassDescriptor {
    pub id: &'static str,
    pub members: &'static str,
    pub canonical: &'static str,
    pub normalizer: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolicNormalizationCaseDescriptor {
    pub id: &'static str,
    pub input: &'static str,
    pub output: &'static str,
    pub law: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolicSubstitutionCaseDescriptor {
    pub id: &'static str,
    pub target: &'static str,
    pub replacement: &'static str,
    pub scope: &'static str,
    pub expected: &'static str,
    pub law: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolicEqualityError {
    EmptySymbol,
    EmptyText,
    DuplicateRecordKey { key: String },
    CaptureRisk { binder: String },
    UnboundDescriptor { id: String },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SymbolicTerm {
    Unit,
    Bool(bool),
    Integer(i128),
    Text(String),
    Symbol(String),
    Pair(Box<SymbolicTerm>, Box<SymbolicTerm>),
    List(Vec<SymbolicTerm>),
    Record(Vec<(String, SymbolicTerm)>),
    Apply(Box<SymbolicTerm>, Box<SymbolicTerm>),
    Bind {
        symbol: String,
        value: Box<SymbolicTerm>,
        body: Box<SymbolicTerm>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicEqualityWitness {
    pub left_canonical: String,
    pub right_canonical: String,
    pub left_normal: String,
    pub right_normal: String,
    pub equal: bool,
    pub witness_hash: String,
}

pub const LYRALANG_SYMBOLIC_EQUALITY_RULE_DESCRIPTORS: &[SymbolicEqualityRuleDescriptor] = &[
    SymbolicEqualityRuleDescriptor {
        id: "reflexive",
        domain: "term",
        relation: "equal(term,term)",
        law: "normal_form_identity",
        status: "artifact_emitted",
    },
    SymbolicEqualityRuleDescriptor {
        id: "symmetric",
        domain: "term",
        relation: "equal(a,b)->equal(b,a)",
        law: "normal_form_equality_symmetric",
        status: "artifact_emitted",
    },
    SymbolicEqualityRuleDescriptor {
        id: "transitive",
        domain: "term",
        relation: "equal(a,b)&equal(b,c)->equal(a,c)",
        law: "normal_form_equality_transitive",
        status: "artifact_emitted",
    },
    SymbolicEqualityRuleDescriptor {
        id: "alpha_equivalent",
        domain: "binder",
        relation: "bind_name_irrelevant",
        law: "binder_scope_canonicalized",
        status: "artifact_emitted",
    },
    SymbolicEqualityRuleDescriptor {
        id: "structural",
        domain: "composition",
        relation: "equal_children_imply_equal_parent",
        law: "canonical_child_order",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SYMBOLIC_EQUIVALENCE_CLASS_DESCRIPTORS: &[SymbolicEquivalenceClassDescriptor] =
    &[
        SymbolicEquivalenceClassDescriptor {
            id: "unit_singleton",
            members: "unit",
            canonical: "unit",
            normalizer: "literal_identity",
            status: "artifact_emitted",
        },
        SymbolicEquivalenceClassDescriptor {
            id: "bool_true_singleton",
            members: "bool.true",
            canonical: "bool.true",
            normalizer: "literal_identity",
            status: "artifact_emitted",
        },
        SymbolicEquivalenceClassDescriptor {
            id: "record_order_class",
            members: "record(b=integer.1,a=integer.0),record(a=integer.0,b=integer.1)",
            canonical: "record(a=integer.0,b=integer.1)",
            normalizer: "record_key_sort",
            status: "artifact_emitted",
        },
        SymbolicEquivalenceClassDescriptor {
            id: "list_child_normal_class",
            members: "list(record(b=integer.1,a=integer.0)),list(record(a=integer.0,b=integer.1))",
            canonical: "list(record(a=integer.0,b=integer.1))",
            normalizer: "recursive_child_normalization",
            status: "artifact_emitted",
        },
        SymbolicEquivalenceClassDescriptor {
            id: "alpha_bind_class",
            members: "bind(x=unit in symbol.x),bind(y=unit in symbol.y)",
            canonical: "bind($0=unit in symbol.$0)",
            normalizer: "alpha_binder_canonicalization",
            status: "artifact_emitted",
        },
    ];

pub const LYRALANG_SYMBOLIC_NORMALIZATION_CASE_DESCRIPTORS:
    &[SymbolicNormalizationCaseDescriptor] = &[
    SymbolicNormalizationCaseDescriptor {
        id: "literal_unit_normal",
        input: "unit",
        output: "unit",
        law: "literal_identity",
        status: "artifact_emitted",
    },
    SymbolicNormalizationCaseDescriptor {
        id: "record_key_sort_normal",
        input: "record(b=integer.1,a=integer.0)",
        output: "record(a=integer.0,b=integer.1)",
        law: "record_key_sort",
        status: "artifact_emitted",
    },
    SymbolicNormalizationCaseDescriptor {
        id: "nested_record_normal",
        input: "list(record(b=integer.1,a=integer.0))",
        output: "list(record(a=integer.0,b=integer.1))",
        law: "recursive_child_normalization",
        status: "artifact_emitted",
    },
    SymbolicNormalizationCaseDescriptor {
        id: "apply_child_normal",
        input: "apply(symbol.f,record(b=integer.1,a=integer.0))",
        output: "apply(symbol.f,record(a=integer.0,b=integer.1))",
        law: "normalize_before_symbolic_apply",
        status: "artifact_emitted",
    },
    SymbolicNormalizationCaseDescriptor {
        id: "bind_alpha_normal",
        input: "bind(x=unit in symbol.x)",
        output: "bind($0=unit in symbol.$0)",
        law: "alpha_binder_canonicalization",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SYMBOLIC_SUBSTITUTION_CASE_DESCRIPTORS: &[SymbolicSubstitutionCaseDescriptor] =
    &[
        SymbolicSubstitutionCaseDescriptor {
            id: "substitute_symbol",
            target: "x",
            replacement: "integer.1",
            scope: "symbol.x",
            expected: "integer.1",
            law: "free_symbol_replacement",
            status: "artifact_emitted",
        },
        SymbolicSubstitutionCaseDescriptor {
            id: "substitute_pair",
            target: "x",
            replacement: "unit",
            scope: "pair(symbol.x,bool.true)",
            expected: "pair(unit,bool.true)",
            law: "structural_substitution",
            status: "artifact_emitted",
        },
        SymbolicSubstitutionCaseDescriptor {
            id: "substitute_record",
            target: "x",
            replacement: "integer.0",
            scope: "record(b=symbol.x,a=unit)",
            expected: "record(a=unit,b=integer.0)",
            law: "substitute_then_normalize",
            status: "artifact_emitted",
        },
        SymbolicSubstitutionCaseDescriptor {
            id: "binder_shadow_guard",
            target: "x",
            replacement: "integer.1",
            scope: "bind(x=unit in symbol.x)",
            expected: "bind($0=unit in symbol.$0)",
            law: "binder_shadow_blocks_substitution",
            status: "artifact_emitted",
        },
        SymbolicSubstitutionCaseDescriptor {
            id: "capture_rejection",
            target: "x",
            replacement: "symbol.y",
            scope: "bind(y=unit in symbol.x)",
            expected: "reject_capture_risk",
            law: "capture_avoidance_required",
            status: "artifact_emitted",
        },
    ];

pub fn symbolic_equality_rule_ids() -> Vec<&'static str> {
    LYRALANG_SYMBOLIC_EQUALITY_RULE_DESCRIPTORS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn symbolic_equivalence_class_ids() -> Vec<&'static str> {
    LYRALANG_SYMBOLIC_EQUIVALENCE_CLASS_DESCRIPTORS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn symbolic_normalization_case_ids() -> Vec<&'static str> {
    LYRALANG_SYMBOLIC_NORMALIZATION_CASE_DESCRIPTORS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn symbolic_substitution_case_ids() -> Vec<&'static str> {
    LYRALANG_SYMBOLIC_SUBSTITUTION_CASE_DESCRIPTORS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn symbolic_equality_rule_descriptor(
    id: &str,
) -> Option<&'static SymbolicEqualityRuleDescriptor> {
    LYRALANG_SYMBOLIC_EQUALITY_RULE_DESCRIPTORS
        .iter()
        .find(|item| item.id == id)
}

pub fn symbolic_equivalence_class_descriptor(
    id: &str,
) -> Option<&'static SymbolicEquivalenceClassDescriptor> {
    LYRALANG_SYMBOLIC_EQUIVALENCE_CLASS_DESCRIPTORS
        .iter()
        .find(|item| item.id == id)
}

pub fn symbolic_normalization_case_descriptor(
    id: &str,
) -> Option<&'static SymbolicNormalizationCaseDescriptor> {
    LYRALANG_SYMBOLIC_NORMALIZATION_CASE_DESCRIPTORS
        .iter()
        .find(|item| item.id == id)
}

pub fn symbolic_substitution_case_descriptor(
    id: &str,
) -> Option<&'static SymbolicSubstitutionCaseDescriptor> {
    LYRALANG_SYMBOLIC_SUBSTITUTION_CASE_DESCRIPTORS
        .iter()
        .find(|item| item.id == id)
}

pub fn canonical_symbolic_term(term: &SymbolicTerm) -> Result<String, SymbolicEqualityError> {
    canonical_symbolic_term_with_binders(term, &mut Vec::new())
}

fn canonical_symbolic_term_with_binders(
    term: &SymbolicTerm,
    binders: &mut Vec<String>,
) -> Result<String, SymbolicEqualityError> {
    match term {
        SymbolicTerm::Unit => Ok("unit".to_string()),
        SymbolicTerm::Bool(value) => Ok(if *value {
            "bool.true".to_string()
        } else {
            "bool.false".to_string()
        }),
        SymbolicTerm::Integer(value) => Ok(format!("integer.{value}")),
        SymbolicTerm::Text(value) => {
            if value.is_empty() {
                Ok("text.\"\"".to_string())
            } else if value
                .bytes()
                .all(|byte| byte.is_ascii_graphic() || byte == b' ')
            {
                Ok(format!(
                    "text.\"{}\"",
                    value.replace('\\', "\\\\").replace('"', "\\\"")
                ))
            } else {
                Err(SymbolicEqualityError::EmptyText)
            }
        }
        SymbolicTerm::Symbol(symbol) => canonical_symbol(symbol, binders),
        SymbolicTerm::Pair(left, right) => Ok(format!(
            "pair({},{})",
            canonical_symbolic_term_with_binders(left, binders)?,
            canonical_symbolic_term_with_binders(right, binders)?
        )),
        SymbolicTerm::List(items) => {
            let mut rendered = Vec::with_capacity(items.len());
            for item in items {
                rendered.push(canonical_symbolic_term_with_binders(item, binders)?);
            }
            Ok(format!("list({})", rendered.join(",")))
        }
        SymbolicTerm::Record(entries) => {
            let mut seen = BTreeSet::new();
            let mut rendered = Vec::with_capacity(entries.len());
            for (key, value) in entries {
                if key.is_empty() {
                    return Err(SymbolicEqualityError::EmptySymbol);
                }
                if !seen.insert(key.clone()) {
                    return Err(SymbolicEqualityError::DuplicateRecordKey { key: key.clone() });
                }
                rendered.push((
                    key.clone(),
                    canonical_symbolic_term_with_binders(value, binders)?,
                ));
            }
            rendered.sort_by(|left, right| left.0.cmp(&right.0));
            Ok(format!(
                "record({})",
                rendered
                    .into_iter()
                    .map(|(key, value)| format!("{key}={value}"))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
        }
        SymbolicTerm::Apply(function, argument) => Ok(format!(
            "apply({},{})",
            canonical_symbolic_term_with_binders(function, binders)?,
            canonical_symbolic_term_with_binders(argument, binders)?
        )),
        SymbolicTerm::Bind {
            symbol,
            value,
            body,
        } => {
            if symbol.is_empty() {
                return Err(SymbolicEqualityError::EmptySymbol);
            }
            let binder_index = binders.len();
            let value_text = canonical_symbolic_term_with_binders(value, binders)?;
            binders.push(symbol.clone());
            let body_text = canonical_symbolic_term_with_binders(body, binders)?;
            binders.pop();
            Ok(format!("bind(${binder_index}={value_text} in {body_text})"))
        }
    }
}

fn canonical_symbol(symbol: &str, binders: &[String]) -> Result<String, SymbolicEqualityError> {
    if symbol.is_empty() {
        return Err(SymbolicEqualityError::EmptySymbol);
    }
    if let Some(index) = binders.iter().rposition(|binder| binder == symbol) {
        return Ok(format!("symbol.${index}"));
    }
    Ok(format!("symbol.{symbol}"))
}

pub fn normalize_symbolic_term(term: &SymbolicTerm) -> Result<String, SymbolicEqualityError> {
    canonical_symbolic_term(term)
}

pub fn symbolic_terms_equal(
    left: &SymbolicTerm,
    right: &SymbolicTerm,
) -> Result<SymbolicEqualityWitness, SymbolicEqualityError> {
    let left_canonical = canonical_symbolic_term(left)?;
    let right_canonical = canonical_symbolic_term(right)?;
    let left_normal = normalize_symbolic_term(left)?;
    let right_normal = normalize_symbolic_term(right)?;
    let equal = left_normal == right_normal;
    let preimage = format!("left:{left_normal}|right:{right_normal}|equal:{equal}");
    Ok(SymbolicEqualityWitness {
        left_canonical,
        right_canonical,
        left_normal,
        right_normal,
        equal,
        witness_hash: stable_hash_label("lyra.p01.symbolic_equality.witness", &preimage),
    })
}

pub fn substitute_symbolic_term(
    term: &SymbolicTerm,
    target: &str,
    replacement: &SymbolicTerm,
) -> Result<SymbolicTerm, SymbolicEqualityError> {
    if target.is_empty() {
        return Err(SymbolicEqualityError::EmptySymbol);
    }
    let replacement_free = free_symbols(replacement);
    substitute_inner(term, target, replacement, &replacement_free)
}

fn substitute_inner(
    term: &SymbolicTerm,
    target: &str,
    replacement: &SymbolicTerm,
    replacement_free: &BTreeSet<String>,
) -> Result<SymbolicTerm, SymbolicEqualityError> {
    match term {
        SymbolicTerm::Unit => Ok(SymbolicTerm::Unit),
        SymbolicTerm::Bool(value) => Ok(SymbolicTerm::Bool(*value)),
        SymbolicTerm::Integer(value) => Ok(SymbolicTerm::Integer(*value)),
        SymbolicTerm::Text(value) => Ok(SymbolicTerm::Text(value.clone())),
        SymbolicTerm::Symbol(symbol) => {
            if symbol == target {
                Ok(replacement.clone())
            } else {
                Ok(SymbolicTerm::Symbol(symbol.clone()))
            }
        }
        SymbolicTerm::Pair(left, right) => Ok(SymbolicTerm::Pair(
            Box::new(substitute_inner(
                left,
                target,
                replacement,
                replacement_free,
            )?),
            Box::new(substitute_inner(
                right,
                target,
                replacement,
                replacement_free,
            )?),
        )),
        SymbolicTerm::List(items) => {
            let mut substituted = Vec::with_capacity(items.len());
            for item in items {
                substituted.push(substitute_inner(
                    item,
                    target,
                    replacement,
                    replacement_free,
                )?);
            }
            Ok(SymbolicTerm::List(substituted))
        }
        SymbolicTerm::Record(entries) => {
            let mut substituted = Vec::with_capacity(entries.len());
            for (key, value) in entries {
                substituted.push((
                    key.clone(),
                    substitute_inner(value, target, replacement, replacement_free)?,
                ));
            }
            Ok(SymbolicTerm::Record(substituted))
        }
        SymbolicTerm::Apply(function, argument) => Ok(SymbolicTerm::Apply(
            Box::new(substitute_inner(
                function,
                target,
                replacement,
                replacement_free,
            )?),
            Box::new(substitute_inner(
                argument,
                target,
                replacement,
                replacement_free,
            )?),
        )),
        SymbolicTerm::Bind {
            symbol,
            value,
            body,
        } => {
            let value = Box::new(substitute_inner(
                value,
                target,
                replacement,
                replacement_free,
            )?);
            if symbol == target {
                return Ok(SymbolicTerm::Bind {
                    symbol: symbol.clone(),
                    value,
                    body: body.clone(),
                });
            }
            if replacement_free.contains(symbol) && free_symbols(body).contains(target) {
                return Err(SymbolicEqualityError::CaptureRisk {
                    binder: symbol.clone(),
                });
            }
            Ok(SymbolicTerm::Bind {
                symbol: symbol.clone(),
                value,
                body: Box::new(substitute_inner(
                    body,
                    target,
                    replacement,
                    replacement_free,
                )?),
            })
        }
    }
}

pub fn free_symbols(term: &SymbolicTerm) -> BTreeSet<String> {
    let mut symbols = BTreeSet::new();
    collect_free_symbols(term, &mut Vec::new(), &mut symbols);
    symbols
}

fn collect_free_symbols(
    term: &SymbolicTerm,
    binders: &mut Vec<String>,
    output: &mut BTreeSet<String>,
) {
    match term {
        SymbolicTerm::Unit
        | SymbolicTerm::Bool(_)
        | SymbolicTerm::Integer(_)
        | SymbolicTerm::Text(_) => {}
        SymbolicTerm::Symbol(symbol) => {
            if !binders.iter().any(|binder| binder == symbol) {
                output.insert(symbol.clone());
            }
        }
        SymbolicTerm::Pair(left, right) | SymbolicTerm::Apply(left, right) => {
            collect_free_symbols(left, binders, output);
            collect_free_symbols(right, binders, output);
        }
        SymbolicTerm::List(items) => {
            for item in items {
                collect_free_symbols(item, binders, output);
            }
        }
        SymbolicTerm::Record(entries) => {
            for (_, value) in entries {
                collect_free_symbols(value, binders, output);
            }
        }
        SymbolicTerm::Bind {
            symbol,
            value,
            body,
        } => {
            collect_free_symbols(value, binders, output);
            binders.push(symbol.clone());
            collect_free_symbols(body, binders, output);
            binders.pop();
        }
    }
}

pub fn normalization_case_digest(id: &str, input: &str, output: &str, law: &str) -> String {
    stable_hash_label(
        "lyra.p01.symbolic_equality.normalization_case",
        &format!("id:{id}|input:{input}|output:{output}|law:{law}"),
    )
}

pub fn substitution_case_digest(
    id: &str,
    target: &str,
    replacement: &str,
    scope: &str,
    expected: &str,
    law: &str,
) -> String {
    stable_hash_label(
        "lyra.p01.symbolic_equality.substitution_case",
        &format!("id:{id}|target:{target}|replacement:{replacement}|scope:{scope}|expected:{expected}|law:{law}"),
    )
}

pub fn canonical_symbolic_equality_rule_signature(item: &SymbolicEqualityRuleDescriptor) -> String {
    format!(
        "equality_rule:{}|domain:{}|relation:{}|law:{}|status:{}",
        item.id, item.domain, item.relation, item.law, item.status
    )
}

pub fn canonical_symbolic_equivalence_class_signature(
    item: &SymbolicEquivalenceClassDescriptor,
) -> String {
    format!(
        "equivalence_class:{}|members:{}|canonical:{}|normalizer:{}|status:{}",
        item.id, item.members, item.canonical, item.normalizer, item.status
    )
}

pub fn canonical_symbolic_normalization_case_signature(
    item: &SymbolicNormalizationCaseDescriptor,
) -> String {
    format!(
        "normalization:{}|input:{}|output:{}|law:{}|digest:{}|status:{}",
        item.id,
        item.input,
        item.output,
        item.law,
        normalization_case_digest(item.id, item.input, item.output, item.law),
        item.status
    )
}

pub fn canonical_symbolic_substitution_case_signature(
    item: &SymbolicSubstitutionCaseDescriptor,
) -> String {
    format!(
        "substitution:{}|target:{}|replacement:{}|scope:{}|expected:{}|law:{}|digest:{}|status:{}",
        item.id,
        item.target,
        item.replacement,
        item.scope,
        item.expected,
        item.law,
        substitution_case_digest(
            item.id,
            item.target,
            item.replacement,
            item.scope,
            item.expected,
            item.law
        ),
        item.status
    )
}

pub fn canonical_symbolic_equality_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_SYMBOLIC_EQUALITY_RULE_DESCRIPTORS {
        rows.push(canonical_symbolic_equality_rule_signature(item));
    }
    for item in LYRALANG_SYMBOLIC_EQUIVALENCE_CLASS_DESCRIPTORS {
        rows.push(canonical_symbolic_equivalence_class_signature(item));
    }
    for item in LYRALANG_SYMBOLIC_NORMALIZATION_CASE_DESCRIPTORS {
        rows.push(canonical_symbolic_normalization_case_signature(item));
    }
    for item in LYRALANG_SYMBOLIC_SUBSTITUTION_CASE_DESCRIPTORS {
        rows.push(canonical_symbolic_substitution_case_signature(item));
    }
    rows.sort();
    rows.join("\n")
}

pub fn canonical_symbolic_equality_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.symbolic_equality.registry",
        &canonical_symbolic_equality_registry_signature(),
    )
}

pub fn parse_descriptor_term(text: &str) -> Result<SymbolicTerm, SymbolicEqualityError> {
    match text {
        "unit" => Ok(SymbolicTerm::Unit),
        "bool.true" => Ok(SymbolicTerm::Bool(true)),
        "bool.false" => Ok(SymbolicTerm::Bool(false)),
        "integer.0" => Ok(SymbolicTerm::Integer(0)),
        "integer.1" => Ok(SymbolicTerm::Integer(1)),
        "symbol.x" => Ok(SymbolicTerm::Symbol("x".to_string())),
        "symbol.y" => Ok(SymbolicTerm::Symbol("y".to_string())),
        "symbol.f" => Ok(SymbolicTerm::Symbol("f".to_string())),
        "pair(symbol.x,bool.true)" => Ok(SymbolicTerm::Pair(
            Box::new(SymbolicTerm::Symbol("x".to_string())),
            Box::new(SymbolicTerm::Bool(true)),
        )),
        "pair(unit,bool.true)" => Ok(SymbolicTerm::Pair(
            Box::new(SymbolicTerm::Unit),
            Box::new(SymbolicTerm::Bool(true)),
        )),
        "record(b=integer.1,a=integer.0)" => Ok(SymbolicTerm::Record(vec![
            ("b".to_string(), SymbolicTerm::Integer(1)),
            ("a".to_string(), SymbolicTerm::Integer(0)),
        ])),
        "record(a=integer.0,b=integer.1)" => Ok(SymbolicTerm::Record(vec![
            ("a".to_string(), SymbolicTerm::Integer(0)),
            ("b".to_string(), SymbolicTerm::Integer(1)),
        ])),
        "record(b=symbol.x,a=unit)" => Ok(SymbolicTerm::Record(vec![
            ("b".to_string(), SymbolicTerm::Symbol("x".to_string())),
            ("a".to_string(), SymbolicTerm::Unit),
        ])),
        "record(a=unit,b=integer.0)" => Ok(SymbolicTerm::Record(vec![
            ("a".to_string(), SymbolicTerm::Unit),
            ("b".to_string(), SymbolicTerm::Integer(0)),
        ])),
        "list(record(b=integer.1,a=integer.0))" => {
            Ok(SymbolicTerm::List(vec![SymbolicTerm::Record(vec![
                ("b".to_string(), SymbolicTerm::Integer(1)),
                ("a".to_string(), SymbolicTerm::Integer(0)),
            ])]))
        }
        "list(record(a=integer.0,b=integer.1))" => {
            Ok(SymbolicTerm::List(vec![SymbolicTerm::Record(vec![
                ("a".to_string(), SymbolicTerm::Integer(0)),
                ("b".to_string(), SymbolicTerm::Integer(1)),
            ])]))
        }
        "apply(symbol.f,record(b=integer.1,a=integer.0))" => Ok(SymbolicTerm::Apply(
            Box::new(SymbolicTerm::Symbol("f".to_string())),
            Box::new(SymbolicTerm::Record(vec![
                ("b".to_string(), SymbolicTerm::Integer(1)),
                ("a".to_string(), SymbolicTerm::Integer(0)),
            ])),
        )),
        "apply(symbol.f,record(a=integer.0,b=integer.1))" => Ok(SymbolicTerm::Apply(
            Box::new(SymbolicTerm::Symbol("f".to_string())),
            Box::new(SymbolicTerm::Record(vec![
                ("a".to_string(), SymbolicTerm::Integer(0)),
                ("b".to_string(), SymbolicTerm::Integer(1)),
            ])),
        )),
        "bind(x=unit in symbol.x)" => Ok(SymbolicTerm::Bind {
            symbol: "x".to_string(),
            value: Box::new(SymbolicTerm::Unit),
            body: Box::new(SymbolicTerm::Symbol("x".to_string())),
        }),
        "bind(y=unit in symbol.y)" => Ok(SymbolicTerm::Bind {
            symbol: "y".to_string(),
            value: Box::new(SymbolicTerm::Unit),
            body: Box::new(SymbolicTerm::Symbol("y".to_string())),
        }),
        "bind(y=unit in symbol.x)" => Ok(SymbolicTerm::Bind {
            symbol: "y".to_string(),
            value: Box::new(SymbolicTerm::Unit),
            body: Box::new(SymbolicTerm::Symbol("x".to_string())),
        }),
        "bind($0=unit in symbol.$0)" => Ok(SymbolicTerm::Bind {
            symbol: "x".to_string(),
            value: Box::new(SymbolicTerm::Unit),
            body: Box::new(SymbolicTerm::Symbol("x".to_string())),
        }),
        _ => Err(SymbolicEqualityError::UnboundDescriptor {
            id: text.to_string(),
        }),
    }
}

pub fn evaluate_substitution_case(id: &str) -> Result<String, SymbolicEqualityError> {
    let Some(case) = symbolic_substitution_case_descriptor(id) else {
        return Err(SymbolicEqualityError::UnboundDescriptor { id: id.to_string() });
    };
    let scope = parse_descriptor_term(case.scope)?;
    let replacement = parse_descriptor_term(case.replacement)?;
    match substitute_symbolic_term(&scope, case.target, &replacement) {
        Ok(term) => normalize_symbolic_term(&term),
        Err(SymbolicEqualityError::CaptureRisk { .. }) => Ok("reject_capture_risk".to_string()),
        Err(error) => Err(error),
    }
}

pub fn evaluated_normalization_case_output(id: &str) -> Result<String, SymbolicEqualityError> {
    let Some(case) = symbolic_normalization_case_descriptor(id) else {
        return Err(SymbolicEqualityError::UnboundDescriptor { id: id.to_string() });
    };
    normalize_symbolic_term(&parse_descriptor_term(case.input)?)
}

pub fn symbolic_equality_digest_for_term(
    term: &SymbolicTerm,
) -> Result<String, SymbolicEqualityError> {
    let normal = normalize_symbolic_term(term)?;
    Ok(stable_hash_label(
        "lyra.p01.symbolic_equality.term",
        &normal,
    ))
}

pub fn canonical_normal_map_for_descriptor_cases() -> BTreeMap<&'static str, &'static str> {
    let mut map = BTreeMap::new();
    for item in LYRALANG_SYMBOLIC_NORMALIZATION_CASE_DESCRIPTORS {
        map.insert(item.input, item.output);
    }
    map
}
