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
