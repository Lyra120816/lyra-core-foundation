# Lyra Core Foundation

Lyra Core Foundation is an experimental deterministic symbolic-computing foundation. This public release contains the P00-P02 foundation layers:

- **P00**: determinism constitution, authority/control law, proof receipts, negative fixtures, and governance validation surfaces.
- **P01**: canonical semantic atoms, core IR, semantic objects, identity/digest law, equality, serialization, adversarial semantic corpus, and semantic closure outputs.
- **P02**: bootstrap trust, seed-runtime law, host-boundary quarantine, target matrices, fallback/replacement law, evidence/handoff surfaces, and retirement/supersession closure outputs.

This is not a full operating system yet. It is the public foundation layer for a deterministic symbolic language/runtime project.

## What is intentionally not included

This repository intentionally does **not** include the private Lyra roadmap, the private agent-control master file, future phase strategy, product roadmap, valuation notes, or private implementation prompts.

Some P00 evidence fixtures refer to a `single_file_master` authority class or to a historical private master filename. Those references are part of the public proof/fixture data model; the private master document itself is not included in this release.

## Repository layout

```text
k0/          deterministic primitives, receipts, canonicalization, replay, benchmark and closure law
lyralang/   symbolic semantic-core support surfaces
interfaces/ phase contracts and consumed public schemas for P00-P02
ops/        validation engines, truth/control surfaces, proof/closure artifacts
tests/      Rust test suites for P00-P02
fixtures/   valid, invalid, negative, and adversarial .lyra inputs
goldens/    golden receipts
receipts/   pass receipts
examples/   public reference examples
docs/       public phase documentation surfaces
products/   public product/output reference surfaces
shells/     public operator shell references for bootstrap surfaces
src/        crate entrypoint and command binaries
```

## Build and validation

This release is intended to be validated locally with Rust/Cargo:

```bash
cargo check
cargo test
```

The release was prepared from a private working snapshot. Local Cargo validation should be run after cloning because this packaging environment does not execute Rust/Cargo.

## Current public status

- P00 artifacts are present through X05.
- P01 artifacts are present through X05.
- P02 artifacts are present through X05.
- Global closure remains dependent on local validation evidence from `cargo check` / `cargo test` in a Rust environment.

## License

This public release is licensed under Apache-2.0. See `LICENSE`.
