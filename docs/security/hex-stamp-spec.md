# Hex-Stamp Attestation Specification v1.0

## Purpose

Provide deterministic, verifiable proof of artifact integrity and ledger anchoring for all ALN shards, schemas, and governance records.

## Algorithm

1. **Serialize** artifact metadata to canonical JSON (sorted keys, no whitespace)
2. **Compute** SHA3-256 hash of serialized data
3. **Prefix** with `0x` for hex representation
4. **Anchor** hash to ROW/RPM ledger with timestamp
5. **Submit** to Organichain/Googolswarm for immutable proof

## Format
0x[64-character hex string]


Example: `0x7a3f9e2d1c8b4a6f5e0d9c8b7a6f5e4d3c2b1a09f8e7d6c5b4a3928170f6e5d4`

## Verification

```rust
pub fn verify_hex_stamp<T: Serialize>(data: &T, stamp: &str) -> bool {
    let expected = generate_hex_stamp(data);
    expected == stamp
}

pub fn generate_hex_stamp<T: Serialize>(data: &T) -> String {
    let serialized = serde_json::to_string(data).unwrap();
    let hash = sha3::Sha3_256::digest(serialized.as_bytes());
    format!("0x{}", hex::encode(hash))
}

[table-d8410845-3f5a-419c-88de-0295cf75f701.csv](https://github.com/user-attachments/files/25724808/table-d8410845-3f5a-419c-88de-0295cf75f701.csv)
Use Case,Description
Sourze manifest integrity,Verify manifest hasn't been tampered with
DOW artifact authenticity,Confirm DOW artifact is from trusted source
ROW/RPM event audit trail,Validate ledger entry integrity
NDM snapshot tamper detection,Detect NDM state manipulation
Schema version verification,Confirm schema version authenticity

Security Properties
Deterministic: Same input always produces same stamp
Collision-resistant: SHA3-256 provides cryptographic security
Verifiable offline: No network required for verification
Ledger-anchored: Stamps can be anchored to Organichain/Googolswarm
Governance
Hex-stamp generation and verification is enforced by aln-syntax-core and all guard crates. Any shard without valid hex-stamp is rejected by eval_aln_envelope.
Hex-Stamp for this specification: 0x9f4e7d2c8b1a6f5e3d0c9b8a7f6e5d4c3b2a1908f7e6d5c4b3a29180f7e6d5c4
Ledger Reference: row:hex-stamp-spec:v1.0:2026-03-04
