use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EconomicsFrameReport {
    pub id: String,
    pub frame_kind: String,
    pub path: String,
    pub cover_count: usize,
    pub output_count: usize,
    pub receipt_count: usize,
    pub frame_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicInterestOutputReport {
    pub id: String,
    pub output_kind: String,
    pub path: String,
    pub constituency_count: usize,
    pub command_count: usize,
    pub proof_count: usize,
    pub rejection_count: usize,
    pub receipt_count: usize,
    pub output_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EconomicsSuiteReport {
    pub frame_count: usize,
    pub output_count: usize,
    pub proof_count: usize,
    pub frame_reports: Vec<EconomicsFrameReport>,
    pub output_reports: Vec<PublicInterestOutputReport>,
    pub suite_hash: String,
}

pub fn deterministic_economics_suite_report(
    frames: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    )],
    outputs: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    )],
    proof_count: usize,
) -> EconomicsSuiteReport {
    let mut sorted_frames = frames.to_vec();
    sorted_frames.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
    });
    let mut sorted_outputs = outputs.to_vec();
    sorted_outputs.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
    });

    let mut frame_reports = Vec::new();
    let mut output_reports = Vec::new();
    let mut preimage = format!(
        "frames:{}|outputs:{}|proofs:{}",
        sorted_frames.len(),
        sorted_outputs.len(),
        proof_count
    );

    for (id, frame_kind, path, mut covers, mut outputs, mut receipts) in sorted_frames {
        covers.sort();
        outputs.sort();
        receipts.sort();
        let frame_preimage = format!(
            "frame:{}|kind:{}|path:{}|covers:{}|outputs:{}|receipts:{}",
            id,
            frame_kind,
            path,
            covers.join(","),
            outputs.join(","),
            receipts.join(",")
        );
        let frame_hash = stable_hash_label("lyra.p00.economics.frame", &frame_preimage);
        preimage.push('|');
        preimage.push_str(&frame_preimage);
        frame_reports.push(EconomicsFrameReport {
            id,
            frame_kind,
            path,
            cover_count: covers.len(),
            output_count: outputs.len(),
            receipt_count: receipts.len(),
            frame_hash,
        });
    }

    for (
        id,
        output_kind,
        path,
        mut constituencies,
        mut commands,
        mut proofs,
        mut receipts,
        mut rejects,
    ) in sorted_outputs
    {
        constituencies.sort();
        commands.sort();
        proofs.sort();
        receipts.sort();
        rejects.sort();
        let output_preimage = format!(
            "output:{}|kind:{}|path:{}|constituencies:{}|commands:{}|proofs:{}|receipts:{}|rejects:{}",
            id,
            output_kind,
            path,
            constituencies.join(","),
            commands.join(","),
            proofs.join(","),
            receipts.join(","),
            rejects.join(",")
        );
        let output_hash = stable_hash_label("lyra.p00.economics.output", &output_preimage);
        preimage.push('|');
        preimage.push_str(&output_preimage);
        output_reports.push(PublicInterestOutputReport {
            id,
            output_kind,
            path,
            constituency_count: constituencies.len(),
            command_count: commands.len(),
            proof_count: proofs.len(),
            rejection_count: rejects.len(),
            receipt_count: receipts.len(),
            output_hash,
        });
    }

    EconomicsSuiteReport {
        frame_count: frame_reports.len(),
        output_count: output_reports.len(),
        proof_count,
        frame_reports,
        output_reports,
        suite_hash: stable_hash_label("lyra.p00.economics.suite", &preimage),
    }
}
