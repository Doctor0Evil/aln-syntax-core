//! ALN Syntax Core - Canonical schema definitions and validation
//!
//! This crate provides the single source of truth for all ALN schema types
//! used across the Sovereign Stack. All guard crates, sovereigntycore,
//! Cyberspectre, and platform adapters import types from this crate.
//!
//! # Example
//!
//! ```rust
//! use aln_syntax_core::schemas::ndm::NdmSnapshot;
//! use aln_syntax_core::validator::SchemaValidator;
//!
//! let validator = SchemaValidator::new();
//! let snapshot = NdmSnapshot::from_aln_bytes(&bytes)?;
//! validator.validate_ndm(&snapshot)?;
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod schemas;
pub mod validator;
pub mod generator;
pub mod types;
pub mod error;
pub mod hex_stamp;

/// Schema version for this release
pub const SCHEMA_VERSION: &str = "1.0.0";

/// Hex-stamp attestation for this release
pub const HEX_STAMP: &str = "0x7a3f9e2d1c8b4a6f5e0d9c8b7a6f5e4d3c2b1a09";

/// Ledger reference for this release
pub const LEDGER_REF: &str = "row:aln-syntax-core:v1.0.0:2026-03-04";

/// Re-export commonly used types
pub use schemas::ndm::NdmState;
pub use schemas::sourze::SourzeCapability;
pub use schemas::row::RowShard;
pub use schemas::rpm::RpmShard;
pub use validator::SchemaValidator;
pub use error::AlnError;

/// Verify hex-stamp integrity
///
/// # Arguments
/// * `data` - Serialized data to verify
/// * `stamp` - Expected hex-stamp
///
/// # Returns
/// * `true` if stamp matches, `false` otherwise
pub fn verify_hex_stamp<T: serde::Serialize>(data: &T, stamp: &str) -> bool {
    hex_stamp::generate_hex_stamp(data) == stamp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_version() {
        assert_eq!(SCHEMA_VERSION, "1.0.0");
    }

    #[test]
    fn test_hex_stamp_format() {
        assert!(HEX_STAMP.starts_with("0x"));
        assert_eq!(HEX_STAMP.len(), 66); // 0x + 64 hex chars
    }
}
