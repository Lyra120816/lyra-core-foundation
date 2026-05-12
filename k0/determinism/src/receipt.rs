use crate::k0_hash::stable_hash_label;
use crate::k0_verdict::Verdict;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Receipt {
    pub header: String,
    pub input_hash: String,
    pub canonical_hash: String,
    pub verdict_hash: String,
    pub receipt_hash: String,
    pub verdict: Verdict,
}

impl Receipt {
    pub fn to_text(&self) -> String {
        let mut body = String::new();
        body.push_str(&self.header);
        body.push('\n');
        body.push_str(&format!("input_hash={}\n", self.input_hash));
        body.push_str(&format!("canonical_hash={}\n", self.canonical_hash));
        body.push_str(&format!("verdict_hash={}\n", self.verdict_hash));
        body.push_str(&self.verdict.canonical_text());
        body.push_str(&format!("receipt_hash={}\n", self.receipt_hash));
        body
    }
}

pub fn build_receipt(raw_input: &str, canonical_text: &str, verdict: Verdict) -> Receipt {
    let input_hash = stable_hash_label("lyra.p00.raw", raw_input);
    let canonical_hash = stable_hash_label("lyra.p00.canonical", canonical_text);
    let verdict_text = verdict.canonical_text();
    let verdict_hash = stable_hash_label("lyra.p00.verdict", &verdict_text);

    let preimage = format!(
        "LYRA-P00-RECEIPT v1\ninput_hash={input_hash}\ncanonical_hash={canonical_hash}\nverdict_hash={verdict_hash}\n{verdict_text}"
    );
    let receipt_hash = stable_hash_label("lyra.p00.receipt", &preimage);

    Receipt {
        header: "LYRA-P00-RECEIPT v1".to_string(),
        input_hash,
        canonical_hash,
        verdict_hash,
        receipt_hash,
        verdict,
    }
}

pub fn build_phase_receipt(
    phase: &str,
    raw_input: &str,
    canonical_text: &str,
    verdict: Verdict,
) -> Receipt {
    let phase_lower = phase.to_ascii_lowercase();
    let header = format!("LYRA-{phase}-RECEIPT v1");
    let input_hash = stable_hash_label(&format!("lyra.{phase_lower}.raw"), raw_input);
    let canonical_hash =
        stable_hash_label(&format!("lyra.{phase_lower}.canonical"), canonical_text);
    let verdict_text = verdict.canonical_text();
    let verdict_hash = stable_hash_label(&format!("lyra.{phase_lower}.verdict"), &verdict_text);
    let preimage = format!(
        "{header}\ninput_hash={input_hash}\ncanonical_hash={canonical_hash}\nverdict_hash={verdict_hash}\n{verdict_text}"
    );
    let receipt_hash = stable_hash_label(&format!("lyra.{phase_lower}.receipt"), &preimage);
    Receipt {
        header,
        input_hash,
        canonical_hash,
        verdict_hash,
        receipt_hash,
        verdict,
    }
}
