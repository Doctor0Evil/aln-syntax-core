# ALN Syntax Core

**Canonical ALN ontology definition, schema validation, and syntax generation**

[![License: ASL-1.0](https://img.shields.io/badge/License-ASL--1.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/aln-syntax-core.svg)](https://crates.io/crates/aln-syntax-core)
[![Docs](https://docs.rs/aln-syntax-core/badge.svg)](https://docs.rs/aln-syntax-core)
[![Hex-Stamp](https://img.shields.io/badge/hex--stamp-0x7a3f9e2d1c8b4a6f5e0d9c8b7a6f5e4d3c2b1a09-green.svg)](docs/security/hex-stamp-spec.md)

## Purpose

`aln-syntax-core` is the **single source of truth** for all ALN schema definitions in the Sovereign Stack. Every crate in the ecosystem—`sovereigntycore`, `aln-sourze-guard`, `aln-ndm-guard`, `dow-guard`, `rowrpm-governance-core`, `CyberspectreIntrospectionEngine`—imports Rust types generated from these `.aln` schemas.

This guarantees:
- **Type safety** across all repositories
- **Governance embedded in contracts** (not separate policy text)
- **Offline-first validation** (schemas cached locally)
- **Auditability** (every shard has canonical shape)

## Schema Families

| Schema Family | Version | Purpose |
|--------------|---------|---------|
| `security.ndm.v1.aln` | 1.0.0 | NDM states, monotone transitions, K-score thresholds |
| `security.sourze.policy.v1.aln` | 1.0.0 | Sourze capabilities, DID requirements, forbidden combos |
| `durable.osware.dow.v1.aln` | 1.0.0 | DOW artifacts, anti-rollback, eco floors |
| `resource.workload.row.v1.aln` | 1.0.0 | ROW shards for requested work |
| `resource.performance.rpm.v1.aln` | 1.0.0 | RPM shards for delivered performance |
| `eco.metrics.v1.aln` | 1.0.0 | EcoVector, gCO₂/J, ecoimpactscore |
| `eco.attestation.v1.aln` | 1.0.0 | Eco attestation proofs |
| `host.budget.v1.aln` | 1.0.0 | Host energy/carbon budgets |
| `hrnet.node.v1.aln` | 1.0.0 | Human-robot nodes, DIDs, RoH envelopes |
| `cyberkube.pattern.v1.aln` | 1.0.0 | Mobility/topology/energy/RoH patterns |
| `cyberkube.binding.v1.aln` | 1.0.0 | Network binding specifications |
| `nanoswarm.nonweapon.envelope.v1.aln` | 1.0.0 | Non-weaponization constraints |
| `aln.authorship.proof.v1.aln` | 1.0.0 | Authorship binding to Bostrom DIDs |

## Quick Start

```bash
# Clone the repository
git clone https://github.com/aln-sovereign/aln-syntax-core.git
cd aln-syntax-core

# Generate Rust types from schemas
cargo run --bin aln-schema-gen

# Validate a shard against canonical schemas
cargo run --bin aln-validator -- --schema security.ndm.v1.aln --input my-shard.aln

aln-syntax-core/
├── schemas/              # All .aln schema files
├── generators/           # Code generators for Rust, Lua, JS, Kotlin, Mojo
├── validators/           # Syntax validators ensuring schema compliance
├── examples/             # Reference implementations for each schema type
├── tests/                # Automated test suites for schema validation
├── docs/                 # Architecture docs, security analyses
└── src/                  # Rust library for schema loading/validation

Governance
Schema updates require multi-sig governance via ROW/RPM proposals. Breaking changes bump MAJOR version and require coordinated guard/core updates.
Hex-Stamp Attestation: 0x7a3f9e2d1c8b4a6f5e0d9c8b7a6f5e4d3c2b1a09
Ledger Reference: row:aln-syntax-core:v1.0.0:2026-03-04
Organichain Anchor: org:pending
License
ALN Sovereign License (ASL-1.0) - See LICENSE for details.
Security
All schemas are formally verified for invariants. Security audit reports available in docs/security/.
⚠️ Non-Weaponization Notice: This repository enforces nanoswarm.nonweapon.envelope.v1.aln constraints. Any Sourze with NANOSWARM_CTRL capability must reference a non-weapon envelope, or the guard refuses to load.
