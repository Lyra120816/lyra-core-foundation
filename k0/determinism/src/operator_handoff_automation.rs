use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffWorkflowReport {
    pub id: String,
    pub stage: String,
    pub workflow_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffCaptureChannelReport {
    pub id: String,
    pub medium: String,
    pub channel_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffTargetReport {
    pub id: String,
    pub target_id: String,
    pub artifact_count: usize,
    pub handoff_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffTruthGateReport {
    pub id: String,
    pub gate_class: String,
    pub gate_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffAutomationReport {
    pub workflow_count: usize,
    pub capture_channel_count: usize,
    pub offline_channel_count: usize,
    pub target_handoff_count: usize,
    pub truth_neutral_handoff_count: usize,
    pub truth_gate_count: usize,
    pub receipt_count: usize,
    pub workflow_reports: Vec<OperatorHandoffWorkflowReport>,
    pub channel_reports: Vec<OperatorHandoffCaptureChannelReport>,
    pub target_reports: Vec<OperatorHandoffTargetReport>,
    pub gate_reports: Vec<OperatorHandoffTruthGateReport>,
    pub automation_hash: String,
}

pub fn deterministic_operator_handoff_automation_report(
    workflows: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    channels: &[(String, String, String, bool, bool, bool, String)],
    handoffs: &[(
        String,
        String,
        String,
        String,
        String,
        Vec<String>,
        String,
        String,
        String,
        String,
    )],
    gates: &[(String, String, String, String, String, String)],
    receipts: &[(String, String, String, String)],
) -> OperatorHandoffAutomationReport {
    let mut ordered_workflows = workflows.to_vec();
    ordered_workflows.sort_by(|left, right| left.0.cmp(&right.0));
    let mut ordered_channels = channels.to_vec();
    ordered_channels.sort_by(|left, right| left.0.cmp(&right.0));
    let mut ordered_handoffs = handoffs.to_vec();
    ordered_handoffs.sort_by(|left, right| left.0.cmp(&right.0));
    let mut ordered_gates = gates.to_vec();
    ordered_gates.sort_by(|left, right| left.0.cmp(&right.0));
    let mut ordered_receipts = receipts.to_vec();
    ordered_receipts.sort_by(|left, right| left.0.cmp(&right.0));

    let mut offline_channel_count = 0usize;
    let mut truth_neutral_handoff_count = 0usize;
    let mut preimage = format!(
        "workflows:{}|channels:{}|handoffs:{}|gates:{}|receipts:{}",
        ordered_workflows.len(),
        ordered_channels.len(),
        ordered_handoffs.len(),
        ordered_gates.len(),
        ordered_receipts.len()
    );

    let mut workflow_reports = Vec::new();
    for (id, stage, trigger, input_set, output_set, deterministic_order, truth_effect, status) in
        ordered_workflows
    {
        let row = format!("workflow:{id}:{stage}:{trigger}:{input_set}:{output_set}:{deterministic_order}:{truth_effect}:{status}");
        preimage.push('|');
        preimage.push_str(&row);
        workflow_reports.push(OperatorHandoffWorkflowReport {
            id,
            stage,
            workflow_hash: stable_hash_label("lyra.p02.operator_handoff.workflow", &row),
        });
    }

    let mut channel_reports = Vec::new();
    for (
        id,
        medium,
        boundary,
        network_allowed,
        operator_ack_required,
        capture_hash_required,
        status,
    ) in ordered_channels
    {
        if !network_allowed {
            offline_channel_count += 1;
        }
        let row = format!("channel:{id}:{medium}:{boundary}:{network_allowed}:{operator_ack_required}:{capture_hash_required}:{status}");
        preimage.push('|');
        preimage.push_str(&row);
        channel_reports.push(OperatorHandoffCaptureChannelReport {
            id,
            medium,
            channel_hash: stable_hash_label("lyra.p02.operator_handoff.channel", &row),
        });
    }

    let mut target_reports = Vec::new();
    for (
        id,
        target_id,
        target_class,
        capture_channel,
        automation_workflow,
        mut required_artifacts,
        pre_import_gate,
        post_import_truth_effect,
        receipt_binding,
        status,
    ) in ordered_handoffs
    {
        required_artifacts.sort();
        if post_import_truth_effect == "none_without_local_replay" {
            truth_neutral_handoff_count += 1;
        }
        let artifact_count = required_artifacts.len();
        let row = format!(
            "handoff:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            id,
            target_id,
            target_class,
            capture_channel,
            automation_workflow,
            required_artifacts.join(","),
            pre_import_gate,
            post_import_truth_effect,
            receipt_binding,
            status
        );
        preimage.push('|');
        preimage.push_str(&row);
        target_reports.push(OperatorHandoffTargetReport {
            id,
            target_id,
            artifact_count,
            handoff_hash: stable_hash_label("lyra.p02.operator_handoff.target", &row),
        });
    }

    let mut gate_reports = Vec::new();
    for (id, gate_class, required_before, rejects, evidence_path, status) in ordered_gates {
        let row =
            format!("gate:{id}:{gate_class}:{required_before}:{rejects}:{evidence_path}:{status}");
        preimage.push('|');
        preimage.push_str(&row);
        gate_reports.push(OperatorHandoffTruthGateReport {
            id,
            gate_class,
            gate_hash: stable_hash_label("lyra.p02.operator_handoff.gate", &row),
        });
    }

    for (id, path, target, status) in ordered_receipts {
        preimage.push_str(&format!("|receipt:{id}:{path}:{target}:{status}"));
    }

    OperatorHandoffAutomationReport {
        workflow_count: workflows.len(),
        capture_channel_count: channels.len(),
        offline_channel_count,
        target_handoff_count: handoffs.len(),
        truth_neutral_handoff_count,
        truth_gate_count: gates.len(),
        receipt_count: receipts.len(),
        workflow_reports,
        channel_reports,
        target_reports,
        gate_reports,
        automation_hash: stable_hash_label("lyra.p02.operator_handoff.automation", &preimage),
    }
}
