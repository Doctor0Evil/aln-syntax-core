# ALN Syntax Core Architecture

## Overview

`aln-syntax-core` is the **Specification Layer** of the Sovereign Spine, providing canonical `.aln` schema definitions that all other crates import as Rust types.

## Architecture Diagram

```mermaid
flowchart TD
    subgraph Schemas["Schema Definitions (.aln)"]
        S1[security.ndm.v1.aln]
        S2[security.sourze.policy.v1.aln]
        S3[resource.workload.row.v1.aln]
        S4[resource.performance.rpm.v1.aln]
        S5[eco.metrics.v1.aln]
        S6[hrnet.node.v1.aln]
        S7[cyberkube.pattern.v1.aln]
        S8[nanoswarm.nonweapon.envelope.v1.aln]
    end

    subgraph Generator["Schema Generator"]
        G1[aln-schema-gen binary]
        G2[Rust type generation]
        G3[Validation code generation]
    end

    subgraph Types["Generated Rust Types"]
        T1[NdmSnapshot, NdmState]
        T2[SourzeManifest, SourzeCapability]
        T3[RowShard, RpmShard]
        T4[EcoVector, HrnetNode]
    end

    subgraph Validator["Schema Validator"]
        V1[validate_ndm()]
        V2[validate_sourze()]
        V3[validate_row()]
        V4[validate_forbidden_combos()]
    end

    subgraph Consumers["Consumer Crates"]
        C1[sovereigntycore]
        C2[aln-sourze-guard]
        C3[aln-ndm-guard]
        C4[dow-guard]
        C5[rowrpm-governance-core]
        C6[CyberspectreIntrospectionEngine]
    end

    Schemas --> Generator
    Generator --> Types
    Types --> Validator
    Types --> Consumers
    Validator --> Consumers

Key Design Principles
Single Source of Truth: All schema definitions live here
Type Safety: Generated Rust types ensure compile-time safety
Governance Embedded: Invariants encoded in schemas, not policy text
Offline-First: Schemas cached locally for air-gapped operation
Auditability: Hex-stamps and Cyberspectre traces on all artifacts
Versioning Strategy
Semantic Versioning: MAJOR.MINOR.PATCH
Breaking Changes: MAJOR version bump requires coordinated updates
Backward Compatibility: Minor versions maintain compatibility
Hex-Stamp Per Release: Each version has unique attestation
Integration Points

[table-d8410845-3f5a-419c-88de-0295cf75f701 (2).csv](https://github.com/user-attachments/files/25724860/table-d8410845-3f5a-419c-88de-0295cf75f701.2.csv)
Consumer,Integration Method
sovereigntycore,"Imports types, calls validator"
aln-sourze-guard,"Uses SourzeManifest, validates capabilities"
aln-ndm-guard,"Uses NdmSnapshot, enforces transitions"
dow-guard,"Uses DOW schemas, validates artifacts"
rowrpm-governance-core,"Uses RowShard/RpmShard, emits ledger entries"
CyberspectreIntrospectionEngine,Links traces to ROW IDs

ecurity Considerations
All schemas formally verified for invariants
Hex-stamp verification on every shard
Multi-DID envelope requirements
Non-weaponization constraints embedded in capability lattice
Offline validation (no remote calls required)
Document Hex-Stamp: 0x1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b
Last Updated: 2026-03-04
