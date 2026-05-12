use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferenceLiteralDescriptor {
    pub id: &'static str,
    pub atom: &'static str,
    pub canonical: &'static str,
    pub normal: &'static str,
    pub evaluator: &'static str,
    pub proof: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferenceCompositionDescriptor {
    pub id: &'static str,
    pub operator: &'static str,
    pub arity: &'static str,
    pub input_order: &'static str,
    pub output: &'static str,
    pub law: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferenceEvalSeedDescriptor {
    pub id: &'static str,
    pub input: &'static str,
    pub reduction: &'static str,
    pub expected: &'static str,
    pub law: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceSemanticsError {
    UnknownLiteral { id: String },
    UnknownComposition { id: String },
    UnknownEvalSeed { id: String },
    EmptyRecordKey,
    DuplicateRecordKey { key: String },
    ApplyTargetNotSymbol { actual: String },
    InvalidLiteralInteger { value: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceLiteral {
    Unit,
    Bool(bool),
    Integer(i128),
    Text(String),
    Symbol(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceExpression {
    Literal(ReferenceLiteral),
    Pair(Box<ReferenceExpression>, Box<ReferenceExpression>),
    List(Vec<ReferenceExpression>),
    Record(Vec<(String, ReferenceExpression)>),
    Apply {
        function: Box<ReferenceExpression>,
        argument: Box<ReferenceExpression>,
    },
    Bind {
        symbol: String,
        value: Box<ReferenceExpression>,
        body: Box<ReferenceExpression>,
    },
    ProofStep {
        claim: String,
        witness: Box<ReferenceExpression>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceEvaluationResult {
    pub canonical_input: String,
    pub canonical_output: String,
    pub steps: Vec<String>,
    pub trace_hash: String,
}

pub const LYRALANG_REFERENCE_LITERAL_DESCRIPTORS: &[ReferenceLiteralDescriptor] = &[
    ReferenceLiteralDescriptor {
        id: "unit",
        atom: "value",
        canonical: "literal.unit",
        normal: "unit",
        evaluator: "literal_self",
        proof: "literal_identity",
    },
    ReferenceLiteralDescriptor {
        id: "bool_true",
        atom: "value",
        canonical: "literal.bool.true",
        normal: "bool.true",
        evaluator: "literal_self",
        proof: "literal_identity",
    },
    ReferenceLiteralDescriptor {
        id: "bool_false",
        atom: "value",
        canonical: "literal.bool.false",
        normal: "bool.false",
        evaluator: "literal_self",
        proof: "literal_identity",
    },
    ReferenceLiteralDescriptor {
        id: "integer_zero",
        atom: "value",
        canonical: "literal.integer.0",
        normal: "integer.0",
        evaluator: "literal_self",
        proof: "literal_identity",
    },
    ReferenceLiteralDescriptor {
        id: "integer_one",
        atom: "value",
        canonical: "literal.integer.1",
        normal: "integer.1",
        evaluator: "literal_self",
        proof: "literal_identity",
    },
    ReferenceLiteralDescriptor {
        id: "text_empty",
        atom: "value",
        canonical: "literal.text.empty",
        normal: "text.empty",
        evaluator: "literal_self",
        proof: "literal_identity",
    },
    ReferenceLiteralDescriptor {
        id: "symbol_core",
        atom: "symbol",
        canonical: "literal.symbol.core",
        normal: "symbol.lyra.core",
        evaluator: "literal_self",
        proof: "symbol_identity",
    },
];

pub const LYRALANG_REFERENCE_COMPOSITION_DESCRIPTORS: &[ReferenceCompositionDescriptor] = &[
    ReferenceCompositionDescriptor {
        id: "identity",
        operator: "compose.identity",
        arity: "one",
        input_order: "single",
        output: "same_normal_form",
        law: "identity_preserves_normal_form",
    },
    ReferenceCompositionDescriptor {
        id: "pair",
        operator: "compose.pair",
        arity: "two",
        input_order: "left_then_right",
        output: "pair_normal_form",
        law: "pair_structural_evaluation",
    },
    ReferenceCompositionDescriptor {
        id: "list",
        operator: "compose.list",
        arity: "many",
        input_order: "index_ascending",
        output: "list_normal_form",
        law: "list_order_preserved",
    },
    ReferenceCompositionDescriptor {
        id: "record",
        operator: "compose.record",
        arity: "many_named",
        input_order: "key_sorted_ascii",
        output: "record_normal_form",
        law: "record_key_order_canonical",
    },
    ReferenceCompositionDescriptor {
        id: "apply",
        operator: "compose.apply",
        arity: "two",
        input_order: "function_then_argument",
        output: "application_normal_form",
        law: "symbolic_application_seed",
    },
    ReferenceCompositionDescriptor {
        id: "bind",
        operator: "compose.bind",
        arity: "three",
        input_order: "symbol_value_body",
        output: "bound_scope_normal_form",
        law: "binding_scope_seed",
    },
    ReferenceCompositionDescriptor {
        id: "proof_step",
        operator: "compose.proof_step",
        arity: "two",
        input_order: "claim_then_witness",
        output: "proof_step_normal_form",
        law: "proof_step_receipt_seed",
    },
];

pub const LYRALANG_REFERENCE_EVAL_SEED_DESCRIPTORS: &[ReferenceEvalSeedDescriptor] = &[
    ReferenceEvalSeedDescriptor {
        id: "literal_self",
        input: "literal.integer.1",
        reduction: "literal_self",
        expected: "integer.1",
        law: "literal_identity",
    },
    ReferenceEvalSeedDescriptor {
        id: "pair_structural",
        input: "pair(literal.integer.0,literal.integer.1)",
        reduction: "left_then_right",
        expected: "pair(integer.0,integer.1)",
        law: "pair_structural_evaluation",
    },
    ReferenceEvalSeedDescriptor {
        id: "list_order",
        input: "list(literal.bool.false,literal.bool.true)",
        reduction: "index_ascending",
        expected: "list(bool.false,bool.true)",
        law: "list_order_preserved",
    },
    ReferenceEvalSeedDescriptor {
        id: "record_key_sort",
        input: "record(b:literal.integer.1,a:literal.integer.0)",
        reduction: "key_sorted_ascii",
        expected: "record(a=integer.0,b=integer.1)",
        law: "record_key_order_canonical",
    },
    ReferenceEvalSeedDescriptor {
        id: "apply_symbolic",
        input: "apply(literal.symbol.core,literal.unit)",
        reduction: "function_then_argument",
        expected: "apply(symbol.lyra.core,unit)",
        law: "symbolic_application_seed",
    },
    ReferenceEvalSeedDescriptor {
        id: "bind_scope",
        input: "bind(x,literal.integer.1,literal.symbol.core)",
        reduction: "symbol_value_body",
        expected: "bind(x=integer.1;symbol.lyra.core)",
        law: "binding_scope_seed",
    },
    ReferenceEvalSeedDescriptor {
        id: "proof_step",
        input: "proof_step(claim.core,literal.unit)",
        reduction: "claim_then_witness",
        expected: "proof_step(claim.core;unit)",
        law: "proof_step_receipt_seed",
    },
];

pub fn reference_literal_ids() -> Vec<&'static str> {
    LYRALANG_REFERENCE_LITERAL_DESCRIPTORS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn reference_composition_ids() -> Vec<&'static str> {
    LYRALANG_REFERENCE_COMPOSITION_DESCRIPTORS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn reference_eval_seed_ids() -> Vec<&'static str> {
    LYRALANG_REFERENCE_EVAL_SEED_DESCRIPTORS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn reference_literal_descriptor(id: &str) -> Option<ReferenceLiteralDescriptor> {
    LYRALANG_REFERENCE_LITERAL_DESCRIPTORS
        .iter()
        .copied()
        .find(|item| item.id == id)
}

pub fn reference_composition_descriptor(id: &str) -> Option<ReferenceCompositionDescriptor> {
    LYRALANG_REFERENCE_COMPOSITION_DESCRIPTORS
        .iter()
        .copied()
        .find(|item| item.id == id)
}

pub fn reference_eval_seed_descriptor(id: &str) -> Option<ReferenceEvalSeedDescriptor> {
    LYRALANG_REFERENCE_EVAL_SEED_DESCRIPTORS
        .iter()
        .copied()
        .find(|item| item.id == id)
}

pub fn canonical_reference_literal_signature(descriptor: ReferenceLiteralDescriptor) -> String {
    format!(
        "reference_literal:{}|atom:{}|canonical:{}|normal:{}|evaluator:{}|proof:{}",
        descriptor.id,
        descriptor.atom,
        descriptor.canonical,
        descriptor.normal,
        descriptor.evaluator,
        descriptor.proof
    )
}

pub fn canonical_reference_composition_signature(
    descriptor: ReferenceCompositionDescriptor,
) -> String {
    format!(
        "reference_composition:{}|operator:{}|arity:{}|input_order:{}|output:{}|law:{}",
        descriptor.id,
        descriptor.operator,
        descriptor.arity,
        descriptor.input_order,
        descriptor.output,
        descriptor.law
    )
}

pub fn canonical_reference_eval_seed_signature(descriptor: ReferenceEvalSeedDescriptor) -> String {
    format!(
        "reference_eval_seed:{}|input:{}|reduction:{}|expected:{}|law:{}|trace:{}",
        descriptor.id,
        descriptor.input,
        descriptor.reduction,
        descriptor.expected,
        descriptor.law,
        reference_eval_seed_trace_hash(descriptor)
    )
}

pub fn canonical_reference_semantics_registry_signature() -> String {
    let mut rows: Vec<String> = Vec::new();
    rows.extend(
        LYRALANG_REFERENCE_LITERAL_DESCRIPTORS
            .iter()
            .copied()
            .map(canonical_reference_literal_signature),
    );
    rows.extend(
        LYRALANG_REFERENCE_COMPOSITION_DESCRIPTORS
            .iter()
            .copied()
            .map(canonical_reference_composition_signature),
    );
    rows.extend(
        LYRALANG_REFERENCE_EVAL_SEED_DESCRIPTORS
            .iter()
            .copied()
            .map(canonical_reference_eval_seed_signature),
    );
    rows.sort();
    rows.join("\n")
}

pub fn reference_eval_seed_trace_hash(descriptor: ReferenceEvalSeedDescriptor) -> String {
    let preimage = format!(
        "LYRA-P01-REFERENCE-EVAL-SEED v1\nid={}\ninput={}\nreduction={}\nexpected={}\nlaw={}\n",
        descriptor.id, descriptor.input, descriptor.reduction, descriptor.expected, descriptor.law
    );
    stable_hash_label("lyra.p01.reference_semantics.eval_seed", &preimage)
}

pub fn reference_literal_from_descriptor(
    id: &str,
) -> Result<ReferenceExpression, ReferenceSemanticsError> {
    match id {
        "unit" => Ok(ReferenceExpression::Literal(ReferenceLiteral::Unit)),
        "bool_true" => Ok(ReferenceExpression::Literal(ReferenceLiteral::Bool(true))),
        "bool_false" => Ok(ReferenceExpression::Literal(ReferenceLiteral::Bool(false))),
        "integer_zero" => Ok(ReferenceExpression::Literal(ReferenceLiteral::Integer(0))),
        "integer_one" => Ok(ReferenceExpression::Literal(ReferenceLiteral::Integer(1))),
        "text_empty" => Ok(ReferenceExpression::Literal(ReferenceLiteral::Text(
            String::new(),
        ))),
        "symbol_core" => Ok(ReferenceExpression::Literal(ReferenceLiteral::Symbol(
            "lyra.core".to_string(),
        ))),
        _ => Err(ReferenceSemanticsError::UnknownLiteral { id: id.to_string() }),
    }
}

pub fn evaluate_reference_expression(
    expression: &ReferenceExpression,
) -> Result<ReferenceEvaluationResult, ReferenceSemanticsError> {
    let canonical_input = canonical_expression_text(expression)?;
    let mut steps = Vec::new();
    let canonical_output = evaluate_to_normal_form(expression, &mut steps)?;
    let trace_preimage = format!(
        "input={canonical_input}\noutput={canonical_output}\nsteps={}\n",
        steps.join(";")
    );
    let trace_hash = stable_hash_label("lyra.p01.reference_semantics.trace", &trace_preimage);
    Ok(ReferenceEvaluationResult {
        canonical_input,
        canonical_output,
        steps,
        trace_hash,
    })
}

pub fn canonical_expression_text(
    expression: &ReferenceExpression,
) -> Result<String, ReferenceSemanticsError> {
    match expression {
        ReferenceExpression::Literal(literal) => canonical_literal_text(literal),
        ReferenceExpression::Pair(left, right) => Ok(format!(
            "pair({},{})",
            canonical_expression_text(left)?,
            canonical_expression_text(right)?
        )),
        ReferenceExpression::List(items) => {
            let mut parts = Vec::new();
            for item in items {
                parts.push(canonical_expression_text(item)?);
            }
            Ok(format!("list({})", parts.join(",")))
        }
        ReferenceExpression::Record(items) => {
            let mut seen = std::collections::BTreeSet::new();
            let mut sorted = Vec::new();
            for (key, value) in items {
                if key.is_empty() {
                    return Err(ReferenceSemanticsError::EmptyRecordKey);
                }
                if !seen.insert(key.clone()) {
                    return Err(ReferenceSemanticsError::DuplicateRecordKey { key: key.clone() });
                }
                sorted.push((key.clone(), canonical_expression_text(value)?));
            }
            sorted.sort_by(|left, right| left.0.cmp(&right.0));
            Ok(format!(
                "record({})",
                sorted
                    .into_iter()
                    .map(|(key, value)| format!("{key}:{value}"))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
        }
        ReferenceExpression::Apply { function, argument } => Ok(format!(
            "apply({},{})",
            canonical_expression_text(function)?,
            canonical_expression_text(argument)?
        )),
        ReferenceExpression::Bind {
            symbol,
            value,
            body,
        } => Ok(format!(
            "bind({},{},{})",
            symbol,
            canonical_expression_text(value)?,
            canonical_expression_text(body)?
        )),
        ReferenceExpression::ProofStep { claim, witness } => Ok(format!(
            "proof_step({},{})",
            claim,
            canonical_expression_text(witness)?
        )),
    }
}

pub fn canonical_literal_text(
    literal: &ReferenceLiteral,
) -> Result<String, ReferenceSemanticsError> {
    match literal {
        ReferenceLiteral::Unit => Ok("literal.unit".to_string()),
        ReferenceLiteral::Bool(true) => Ok("literal.bool.true".to_string()),
        ReferenceLiteral::Bool(false) => Ok("literal.bool.false".to_string()),
        ReferenceLiteral::Integer(value) => {
            if *value < 0 {
                return Err(ReferenceSemanticsError::InvalidLiteralInteger {
                    value: value.to_string(),
                });
            }
            Ok(format!("literal.integer.{value}"))
        }
        ReferenceLiteral::Text(value) if value.is_empty() => Ok("literal.text.empty".to_string()),
        ReferenceLiteral::Text(value) => Ok(format!(
            "literal.text.{}",
            stable_hash_label("lyra.p01.reference_semantics.text", value)
        )),
        ReferenceLiteral::Symbol(value) => Ok(format!("literal.symbol.{value}")),
    }
}

fn evaluate_to_normal_form(
    expression: &ReferenceExpression,
    steps: &mut Vec<String>,
) -> Result<String, ReferenceSemanticsError> {
    match expression {
        ReferenceExpression::Literal(literal) => {
            steps.push("literal_self".to_string());
            normal_literal_text(literal)
        }
        ReferenceExpression::Pair(left, right) => {
            let left_value = evaluate_to_normal_form(left, steps)?;
            let right_value = evaluate_to_normal_form(right, steps)?;
            steps.push("pair_structural_evaluation".to_string());
            Ok(format!("pair({left_value},{right_value})"))
        }
        ReferenceExpression::List(items) => {
            let mut values = Vec::new();
            for item in items {
                values.push(evaluate_to_normal_form(item, steps)?);
            }
            steps.push("list_order_preserved".to_string());
            Ok(format!("list({})", values.join(",")))
        }
        ReferenceExpression::Record(items) => {
            let mut seen = std::collections::BTreeSet::new();
            let mut sorted = Vec::new();
            for (key, value) in items {
                if key.is_empty() {
                    return Err(ReferenceSemanticsError::EmptyRecordKey);
                }
                if !seen.insert(key.clone()) {
                    return Err(ReferenceSemanticsError::DuplicateRecordKey { key: key.clone() });
                }
                sorted.push((key.clone(), evaluate_to_normal_form(value, steps)?));
            }
            sorted.sort_by(|left, right| left.0.cmp(&right.0));
            steps.push("record_key_order_canonical".to_string());
            Ok(format!(
                "record({})",
                sorted
                    .into_iter()
                    .map(|(key, value)| format!("{key}={value}"))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
        }
        ReferenceExpression::Apply { function, argument } => {
            let function_value = evaluate_to_normal_form(function, steps)?;
            if !function_value.starts_with("symbol.") {
                return Err(ReferenceSemanticsError::ApplyTargetNotSymbol {
                    actual: function_value,
                });
            }
            let argument_value = evaluate_to_normal_form(argument, steps)?;
            steps.push("symbolic_application_seed".to_string());
            Ok(format!("apply({function_value},{argument_value})"))
        }
        ReferenceExpression::Bind {
            symbol,
            value,
            body,
        } => {
            let bound_value = evaluate_to_normal_form(value, steps)?;
            let body_value = evaluate_to_normal_form(body, steps)?;
            steps.push("binding_scope_seed".to_string());
            Ok(format!("bind({symbol}={bound_value};{body_value})"))
        }
        ReferenceExpression::ProofStep { claim, witness } => {
            let witness_value = evaluate_to_normal_form(witness, steps)?;
            steps.push("proof_step_receipt_seed".to_string());
            Ok(format!("proof_step({claim};{witness_value})"))
        }
    }
}

fn normal_literal_text(literal: &ReferenceLiteral) -> Result<String, ReferenceSemanticsError> {
    match literal {
        ReferenceLiteral::Unit => Ok("unit".to_string()),
        ReferenceLiteral::Bool(true) => Ok("bool.true".to_string()),
        ReferenceLiteral::Bool(false) => Ok("bool.false".to_string()),
        ReferenceLiteral::Integer(value) => {
            if *value < 0 {
                return Err(ReferenceSemanticsError::InvalidLiteralInteger {
                    value: value.to_string(),
                });
            }
            Ok(format!("integer.{value}"))
        }
        ReferenceLiteral::Text(value) if value.is_empty() => Ok("text.empty".to_string()),
        ReferenceLiteral::Text(value) => Ok(format!(
            "text.{}",
            stable_hash_label("lyra.p01.reference_semantics.text", value)
        )),
        ReferenceLiteral::Symbol(value) => Ok(format!("symbol.{value}")),
    }
}
