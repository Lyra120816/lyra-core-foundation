use crate::k0_canonical::{canonical_surface_text, CanonicalizationError};
use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicEngineStep {
    pub order: String,
    pub name: String,
    pub input_hash: String,
    pub output_hash: String,
}

impl DeterministicEngineStep {
    pub fn canonical_line(&self) -> String {
        format!(
            "step:{}|name:{}|input:{}|output:{}",
            self.order, self.name, self.input_hash, self.output_hash
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicEngineTrace {
    pub label: String,
    pub input_hash: String,
    pub canonical_hash: String,
    pub trace_hash: String,
    pub steps: Vec<DeterministicEngineStep>,
}

impl DeterministicEngineTrace {
    pub fn canonical_text(&self) -> String {
        let mut output = String::new();
        output.push_str("LYRA-K0-DETERMINISTIC-ENGINE-TRACE v1\n");
        output.push_str(&format!("label={}\n", self.label));
        output.push_str(&format!("input_hash={}\n", self.input_hash));
        output.push_str(&format!("canonical_hash={}\n", self.canonical_hash));
        for step in &self.steps {
            output.push_str(&step.canonical_line());
            output.push('\n');
        }
        output.push_str(&format!("trace_hash={}\n", self.trace_hash));
        output
    }
}

pub fn deterministic_engine_trace(
    label: &str,
    raw_input: &str,
) -> Result<DeterministicEngineTrace, CanonicalizationError> {
    let canonical = canonical_surface_text(raw_input)?;
    let input_hash = stable_hash_label("lyra.k0.engine.raw", raw_input);
    let canonical_hash = stable_hash_label("lyra.k0.engine.canonical", &canonical);
    let steps = vec![
        DeterministicEngineStep {
            order: "001".to_string(),
            name: "canonicalize_input".to_string(),
            input_hash: input_hash.clone(),
            output_hash: canonical_hash.clone(),
        },
        DeterministicEngineStep {
            order: "002".to_string(),
            name: "stable_hash_canonical".to_string(),
            input_hash: canonical_hash.clone(),
            output_hash: stable_hash_label("lyra.k0.engine.step.hash", &canonical_hash),
        },
        DeterministicEngineStep {
            order: "003".to_string(),
            name: "emit_replay_witness".to_string(),
            input_hash: canonical_hash.clone(),
            output_hash: stable_hash_label(
                "lyra.k0.engine.step.replay",
                &format!("{label}\n{canonical_hash}"),
            ),
        },
    ];
    let mut trace_preimage = String::new();
    trace_preimage.push_str(label);
    trace_preimage.push('\n');
    trace_preimage.push_str(&input_hash);
    trace_preimage.push('\n');
    trace_preimage.push_str(&canonical_hash);
    trace_preimage.push('\n');
    for step in &steps {
        trace_preimage.push_str(&step.canonical_line());
        trace_preimage.push('\n');
    }
    let trace_hash = stable_hash_label("lyra.k0.engine.trace", &trace_preimage);
    Ok(DeterministicEngineTrace {
        label: label.to_string(),
        input_hash,
        canonical_hash,
        trace_hash,
        steps,
    })
}
